use std::{
    path::{ Path, PathBuf },
    iter
};

use crate::{
    errors::DomainError,
    infrastructure::errors::InfrastructureError,
    port::{
        ReadableFileSystem,
        WriteableFileSystem,
        Atomic,
        Entry,
        Operation,
        OperationGenerator
    }
};

pub struct CopyBatchDefinition {
    source: PathBuf,
    destination: PathBuf
}

impl CopyBatchDefinition {
    pub fn new(source: PathBuf, destination: PathBuf) -> Self {
        CopyBatchDefinition {
            source,
            destination
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CopyScheduling {
    DirectoryMerge,
    FileOverwrite,
    FileCopy,
    DirectoryCopy
}

pub struct CopyOperation {
    pub scheduling: CopyScheduling,
    pub transaction: Vec<Atomic>,
    pub source: PathBuf,
    pub destination: PathBuf
}

impl CopyOperation {
    fn schedule<F: ReadableFileSystem>(fs: &F, source_path: &Path, destination_path: &Path) -> Result<CopyScheduling, DomainError> {
        let source = fs.status(source_path)?;
        if !source.exists() {
            return Err(DomainError::SourceDoesNotExists(source_path.to_path_buf()))
        }

        let destination = fs.status(destination_path)?;
        if source.is_dir() && destination.is_contained_by(&source) {
            return Err(DomainError::CopyIntoItSelf(source.to_path(), destination.to_path()));
        }

        if destination.exists() {
            if source.is_dir() {
                if destination.is_dir() {
                    Ok(CopyScheduling::DirectoryMerge)
                } else {
                    Err(DomainError::MergeFileWithDirectory(source.to_path(), destination.to_path()))
                }
            } else if source.is_file() {
                if destination.is_file() {
                    Ok(CopyScheduling::FileOverwrite)
                } else {
                    Err(DomainError::OverwriteDirectoryWithFile(source.to_path(), destination.to_path()))
                }
            } else {
                Err(DomainError::Custom(String::from("Unknown node source type")))
            }
        } else if source.is_dir() {
            Ok(CopyScheduling::DirectoryCopy)
        } else if source.is_file() {
            Ok(CopyScheduling::FileCopy)
        } else {
            Err(DomainError::Custom(String::from("Unknown node source type")))
        }
    }

    fn transaction(scheduling: &CopyScheduling, source: PathBuf, destination: PathBuf) -> Vec<Atomic> {
        match scheduling {
            CopyScheduling::FileCopy => vec![
                Atomic::CopyFileToFile{ source, destination }
            ],
            CopyScheduling::FileOverwrite => vec![
                Atomic::RemoveFile(destination.clone()),
                Atomic::CopyFileToFile{ source, destination }
            ],
            CopyScheduling::DirectoryCopy => vec![
                Atomic::BindDirectoryToDirectory { source, destination }
            ],
            _ => Vec::new()
        }
    }

    fn new(scheduling: CopyScheduling, source_path: &Path, destination_path: &Path) -> Self {
        CopyOperation {
            transaction: Self::transaction(&scheduling, source_path.to_path_buf(), destination_path.to_path_buf()),
            scheduling,
            source: source_path.to_path_buf(),
            destination: destination_path.to_path_buf()
        }
    }
}

impl Operation for CopyOperation {
    fn apply<F: WriteableFileSystem>(&self, fs: &mut F) -> Result<(), InfrastructureError> {
        for atomic_operation in self.transaction.iter() {
            atomic_operation.apply(fs)?
        }
        Ok(())
    }
}

pub enum CopyOperationGeneratorState<'a, E: Entry + 'a> {
    Uninitialized,
    SelfOperation(CopyScheduling),
    ChildrenOperation {
        children_iterator: Box<dyn Iterator<Item = E> + 'a>,
        opt_operation_generator: Option<Box<CopyOperationGenerator<'a, E>>>
    },
    Terminated
}

pub struct CopyOperationGenerator<'a, E: Entry + 'a> {
    definition: CopyBatchDefinition,
    state: CopyOperationGeneratorState<'a, E>
}

impl <'a, E: Entry + 'a>CopyOperationGenerator<'a, E> {
    pub fn new(definition: CopyBatchDefinition) -> Self {
        CopyOperationGenerator {
            state: CopyOperationGeneratorState::Uninitialized,
            definition
        }
    }
}

impl <'a, E: Entry> OperationGenerator<E> for CopyOperationGenerator<'a, E> {
    type Item = CopyOperation;
    fn next<F: ReadableFileSystem<Item=E>>(&mut self, fs: &F) -> Result<Option<Self::Item>, DomainError> {
        match &mut self.state {
            CopyOperationGeneratorState::Uninitialized => {
                let scheduling = CopyOperation::schedule(fs, &self.definition.source, &self.definition.destination)?;
                self.state = CopyOperationGeneratorState::SelfOperation(scheduling);
                self.next(fs)
            },
            CopyOperationGeneratorState::SelfOperation(scheduling) => {
                let _scheduling = scheduling.clone();

                self.state = match _scheduling {
                    CopyScheduling::DirectoryMerge => CopyOperationGeneratorState::ChildrenOperation {
                        children_iterator: Box::new(fs.read_dir(&self.definition.source)?.into_iter()),
                        opt_operation_generator: None
                    },
                    CopyScheduling::DirectoryCopy => CopyOperationGeneratorState::ChildrenOperation {
                        children_iterator: Box::new(fs.read_maintained(&self.definition.source)?.into_iter()),
                        opt_operation_generator: None
                    },
                    _ => CopyOperationGeneratorState::Terminated
                };

                Ok(Some(CopyOperation::new(_scheduling, &self.definition.source, &self.definition.destination)))
            },
            CopyOperationGeneratorState::ChildrenOperation { children_iterator, opt_operation_generator }=> {
                if let Some(operation_generator) = opt_operation_generator {
                    if let Some(operation) = operation_generator.next(fs)? {
                        return Ok(Some(operation));
                    }
                }
                if let Some(entry) = children_iterator.next() {
                    *opt_operation_generator = Some(Box::new(
                        CopyOperationGenerator::new(
                            CopyBatchDefinition::new(
                                entry.to_path(),
                                //TODO #NoUnwrap
                                self.definition.destination.join(entry.name().unwrap()).to_path_buf()
                            )
                        )
                    ));
                } else {
                    self.state = CopyOperationGeneratorState::Terminated;
                }
                self.next(fs)
            },
            CopyOperationGeneratorState::Terminated => Ok(None)
        }
    }
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod real_tests {
    use super::*;

    use crate::{
        sample::Samples,
        infrastructure::RealFileSystem,
        port::{ FileSystemAdapter }
    };

    #[test]
    fn copy_operation_dir(){
        let chroot = Samples::init_simple_chroot("operation_copy_operation_dir");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = CopyOperationGenerator::new(CopyBatchDefinition::new(
            chroot.join("RDIR"),
            chroot.join("COPIED"),
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(chroot.join("RDIR/RFILEA").exists());
        assert!(chroot.join("COPIED/RFILEA").exists());
    }

    #[test]
    fn copy_operation_dir_merge_overwrite(){
        let chroot = Samples::init_simple_chroot("operation_copy_operation_dir_merge_overwrite");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = CopyOperationGenerator::new(CopyBatchDefinition::new(
            chroot.join("RDIR"),
            chroot.join("RDIR2"),
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(chroot.join("RDIR/RFILEB").exists());
        assert!(chroot.join("RDIR2/RFILEA").exists());
        assert!(chroot.join("RDIR2/RFILEB").exists());
        assert!(chroot.join("RDIR2/RFILEC").exists());
        assert_eq!(
            chroot.join("RDIR/RFILEA").metadata().unwrap().len(),
            chroot.join("RDIR2/RFILEA").metadata().unwrap().len()
        )
    }

    #[test]
    fn copy_operation_file(){
        let chroot = Samples::init_simple_chroot("operation_copy_operation_file");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = CopyOperationGenerator::new(CopyBatchDefinition::new(
            chroot.join("RDIR/RFILEB"),
            chroot.join("RDIR2/RFILEB"),
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(chroot.join("RDIR/RFILEB").exists());
        assert!(chroot.join("RDIR2/RFILEB").exists());
        assert_eq!(
            chroot.join("RDIR/RFILEB").metadata().unwrap().len(),
            chroot.join("RDIR2/RFILEB").metadata().unwrap().len()
        )
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod virtual_tests {
    use super::*;

    use crate::{
        sample::Samples,
        port::{
            FileSystemAdapter
        },
        infrastructure::{
            VirtualFileSystem
        },
        Kind,
        capability::{
            ZealousGuard
        }
    };

    #[test]
    fn virtual_copy_operation_directory(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = CopyOperationGenerator::new(CopyBatchDefinition::new(
            samples_path.join("A"),
            samples_path.join("Z"),
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("Z").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(samples_path.join("Z").as_path()).unwrap());
    }

    #[test]
    fn virtual_copy_operation_directory_merge(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        //Avoid need of override because of .gitkeep file present in both directory
        let gitkeep = samples_path.join("B/.gitkeep");
        fs.as_inner_mut().mut_sub_state().attach(gitkeep.as_path(),Some(gitkeep.as_path()), Kind::File).unwrap();

        let mut generator = CopyOperationGenerator::new(CopyBatchDefinition::new(
            samples_path.join("B"),
            samples_path.join("A"),
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("A/D").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(samples_path.join("A/D").as_path()).unwrap());
    }

    #[test]
    fn virtual_copy_operation_file(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = CopyOperationGenerator::new(CopyBatchDefinition::new(
            samples_path.join("F"),
            samples_path.join("Z"),
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("Z").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_file(samples_path.join("Z").as_path()).unwrap());
    }

    #[test]
    fn virtual_copy_operation_file_overwrite(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = CopyOperationGenerator::new(CopyBatchDefinition::new(
            samples_path.join("F"),
            samples_path.join("A/C"),
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("A/C").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_file(samples_path.join("A/C").as_path()).unwrap());
    }
}
