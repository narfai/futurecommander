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

#[derive(Clone)]
pub struct MoveBatchDefinition {
    source: PathBuf,
    destination: PathBuf
}

impl MoveBatchDefinition {
    pub fn new(source: PathBuf, destination: PathBuf) -> Self {
        MoveBatchDefinition {
            source,
            destination
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MoveScheduling {
    DirectoryMerge,
    FileOverwrite,
    FileMove,
    DirectoryMoveBefore,
    DirectoryMoveAfter,
}

pub struct MoveOperation {
    scheduling: MoveScheduling,
    transaction: Vec<Atomic>,
    definition: MoveBatchDefinition
}

impl MoveOperation {
    fn schedule<F: ReadableFileSystem>(fs: &F, definition: &MoveBatchDefinition) -> Result<MoveScheduling, DomainError> {
        let source = fs.status(&definition.source)?;

        if !source.exists() {
            return Err(DomainError::SourceDoesNotExists(definition.source.to_path_buf()))
        }

        let destination = fs.status(&definition.destination)?;

        if source.is_dir() && destination.is_contained_by(&source) {
            return Err(DomainError::CopyIntoItSelf(source.to_path(), destination.to_path()));
        }

        if destination.exists() {
            if source.is_dir() {
                if destination.is_dir() {
                    Ok(MoveScheduling::DirectoryMerge)
                } else {
                    Err(DomainError::MergeFileWithDirectory(source.to_path(), destination.to_path()))
                }
            } else if source.is_file() {
                if destination.is_file() {
                    Ok(MoveScheduling::FileOverwrite)
                } else {
                    Err(DomainError::OverwriteDirectoryWithFile(source.to_path(), destination.to_path()))
                }
            } else {
                Err(DomainError::Custom(String::from("Unknown node source type")))
            }
        } else if source.is_dir() {
            Ok(MoveScheduling::DirectoryMoveBefore)
        } else if source.is_file() {
            Ok(MoveScheduling::FileMove)
        } else {
            Err(DomainError::Custom(String::from("Unknown node source type")))
        }
    }

    fn transaction(scheduling: &MoveScheduling, source: PathBuf, destination: PathBuf) -> Vec<Atomic> {
        match scheduling {
            MoveScheduling::FileMove => vec![
                Atomic::MoveFileToFile{ source, destination }
            ],
            MoveScheduling::FileOverwrite => vec![
                Atomic::RemoveFile(destination.clone()),
                Atomic::MoveFileToFile{ source, destination }
            ],
            MoveScheduling::DirectoryMoveBefore => vec![
                Atomic::BindDirectoryToDirectory { source, destination }
            ],
            MoveScheduling::DirectoryMoveAfter => vec![
                Atomic::RemoveMaintainedEmptyDirectory(source)
            ],
            MoveScheduling::DirectoryMerge => vec![
                Atomic::RemoveEmptyDirectory(source)
            ],
            _ => Vec::new()
        }
    }

    fn new(scheduling: MoveScheduling, definition: MoveBatchDefinition) -> Self {
        MoveOperation {
            transaction: Self::transaction(&scheduling, definition.source.clone(), definition.destination.clone()),
            scheduling,
            definition
        }
    }
}

impl Operation for MoveOperation {
    fn apply<F: WriteableFileSystem>(&self, fs: &mut F) -> Result<(), InfrastructureError> {
        for atomic_operation in self.transaction.iter() {
            atomic_operation.apply(fs)?
        }
        Ok(())
    }
}

pub enum MoveOperationGeneratorState<'a, E: Entry + 'a> {
    Uninitialized,
    SelfOperation(MoveScheduling),
    ChildrenOperation {
        scheduling: MoveScheduling,
        children_iterator: Box<dyn Iterator<Item = E> + 'a>,
        opt_operation_generator: Option<Box<MoveOperationGenerator<'a, E>>>
    },
    Terminated
}

pub struct MoveOperationGenerator<'a, E: Entry + 'a> {
    definition: MoveBatchDefinition,
    state: MoveOperationGeneratorState<'a, E>
}

impl <'a, E: Entry + 'a>MoveOperationGenerator<'a, E> {
    pub fn new(definition: MoveBatchDefinition) -> Self {
        MoveOperationGenerator {
            state: MoveOperationGeneratorState::Uninitialized,
            definition
        }
    }
}

impl <'a, E: Entry> OperationGenerator<E> for MoveOperationGenerator<'a, E> {
    type Item = MoveOperation;
    fn next<F: ReadableFileSystem<Item=E>>(&mut self, fs: &F) -> Result<Option<Self::Item>, DomainError> {
        match &mut self.state {
            MoveOperationGeneratorState::Uninitialized => {
                let scheduling = MoveOperation::schedule(fs, &self.definition)?;
                match scheduling {
                    MoveScheduling::DirectoryMerge => {
                        self.state = MoveOperationGeneratorState::ChildrenOperation {
                            scheduling,
                            children_iterator: Box::new(fs.read_dir(&self.definition.source)?.into_iter()),
                            opt_operation_generator: None
                        }
                    },
                    _ => {
                        self.state = MoveOperationGeneratorState::SelfOperation(scheduling)
                    }
                }

                self.next(fs)
            },
            MoveOperationGeneratorState::SelfOperation(scheduling) => {
                let _scheduling = scheduling.clone();
                self.state = match _scheduling {
                    MoveScheduling::DirectoryMoveBefore => MoveOperationGeneratorState::ChildrenOperation {
                        scheduling: _scheduling,
                        children_iterator: Box::new(fs.read_maintained(&self.definition.source)?.into_iter()),
                        opt_operation_generator: None
                    },
                    _ => MoveOperationGeneratorState::Terminated
                };

                Ok(Some(MoveOperation::new(_scheduling, self.definition.clone())))
            },
            MoveOperationGeneratorState::ChildrenOperation { scheduling, children_iterator, opt_operation_generator }=> {
                if let Some(operation_generator) = opt_operation_generator {
                    if let Some(operation) = operation_generator.next(fs)? {
                        return Ok(Some(operation));
                    }
                }
                if let Some(entry) = children_iterator.next() {
                    *opt_operation_generator = Some(Box::new(
                        MoveOperationGenerator::new(
                            MoveBatchDefinition::new(
                                entry.to_path(),
                                //TODO #NoUnwrap
                                self.definition.destination.join(entry.name().unwrap()).to_path_buf()
                            )
                        )
                    ));
                } else {
                    self.state = match scheduling {
                        MoveScheduling::DirectoryMerge => MoveOperationGeneratorState::SelfOperation(MoveScheduling::DirectoryMerge),
                        MoveScheduling::DirectoryMoveBefore => MoveOperationGeneratorState::SelfOperation(MoveScheduling::DirectoryMoveAfter),
                        _ => MoveOperationGeneratorState::Terminated
                    }
                }
                self.next(fs)
            },
            MoveOperationGeneratorState::Terminated => Ok(None)
        }
    }
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod real_tests {
    use super::*;

    use crate::{
        sample::Samples,
        port::{ FileSystemAdapter },
        infrastructure::{ RealFileSystem }
    };

    #[test]
    fn move_operation_dir(){
        let chroot = Samples::init_simple_chroot("move_operation_dir");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = MoveOperationGenerator::new(MoveBatchDefinition::new(
            chroot.join("RDIR"),
            chroot.join("MOVED"),
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!chroot.join("RDIR").exists());
        assert!(chroot.join("MOVED").exists());
        assert!(chroot.join("MOVED/RFILEA").exists());
        assert!(chroot.join("MOVED/RFILEB").exists());
    }

    #[test]
    fn move_operation_dir_merge_overwrite(){
        let chroot = Samples::init_simple_chroot("move_operation_dir_merge_overwrite");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        let mut generator = MoveOperationGenerator::new(MoveBatchDefinition::new(
            chroot.join("RDIR"),
            chroot.join("RDIR2"),
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!chroot.join("RDIR").exists());
        assert!(chroot.join("RDIR2/RFILEA").exists());
        assert!(chroot.join("RDIR2/RFILEB").exists());
        assert!(chroot.join("RDIR2/RFILEC").exists());
        assert_eq!(
            a_len,
            chroot.join("RDIR2/RFILEA").metadata().unwrap().len()
        )
    }

    #[test]
    fn move_operation_file(){
        let chroot = Samples::init_simple_chroot("move_operation_file");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        let mut generator = MoveOperationGenerator::new(MoveBatchDefinition::new(
            chroot.join("RDIR/RFILEA"),
            chroot.join("MOVED"),
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!chroot.join("RDIR/RFILEA").exists());
        assert!(chroot.join("MOVED").exists());
        assert_eq!(
            a_len,
            chroot.join("MOVED").metadata().unwrap().len()
        )
    }

    #[test]
    fn move_operation_file_overwrite(){
        let chroot = Samples::init_simple_chroot("move_operation_file_overwrite");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        let mut generator = MoveOperationGenerator::new(MoveBatchDefinition::new(
            chroot.join("RDIR/RFILEA"),
            chroot.join("RDIR2/RFILEA"),
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!chroot.join("RDIR/RFILEA").exists());
        assert!(chroot.join("RDIR2/RFILEA").exists());
        assert_eq!(
            a_len,
            chroot.join("RDIR2/RFILEA").metadata().unwrap().len()
        )
    }
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod virtual_tests {
    use super::*;

    use crate::{
        sample::Samples,
        Kind,
        port::{ FileSystemAdapter },
        infrastructure::{ VirtualFileSystem }
    };

    #[test]
    fn virtual_move_operation_directory(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = MoveOperationGenerator::new(MoveBatchDefinition::new(
            samples_path.join("A"),
            samples_path.join("Z"),
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("A").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("Z/A").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(samples_path.join("Z").as_path()).unwrap());
    }

    #[test]
    fn virtual_move_operation_directory_merge(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        //Avoid need of override because of .gitkeep file present in both directory
        let gitkeep = samples_path.join("B/.gitkeep");
        fs.as_inner_mut().mut_sub_state().attach(gitkeep.as_path(),Some(gitkeep.as_path()), Kind::File).unwrap();

        let mut generator = MoveOperationGenerator::new(MoveBatchDefinition::new(
            samples_path.join("B"),
            samples_path.join("A"),
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("B").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("A/D").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(samples_path.join("A/D").as_path()).unwrap());
    }

    #[test]
    fn virtual_move_operation_file(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = MoveOperationGenerator::new(MoveBatchDefinition::new(
            samples_path.join("F"),
            samples_path.join("Z"),
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("F").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("Z").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_file(samples_path.join("Z").as_path()).unwrap());
    }

    #[test]
    fn virtual_move_operation_file_overwrite(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = MoveOperationGenerator::new(MoveBatchDefinition::new(
            samples_path.join("F"),
            samples_path.join("A/C"),
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("F").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(samples_path.join("A/C").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_file(samples_path.join("A/C").as_path()).unwrap());
    }
}
