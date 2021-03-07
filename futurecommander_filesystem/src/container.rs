
use std::{
    path::{ Path },
    collections::vec_deque::VecDeque
};
use serde::{ Serialize };
use crate::{
    Kind,
    DomainError,
    QueryError,
    ReadableFileSystem,
    EntryAdapter,
    EntryCollection,
    infrastructure::{
        FileSystemAdapter,
        VirtualFileSystem,
        VirtualStatus,
        RealFileSystem,
    },
    guard::{
        Guard,
        PresetGuard,
        ZealousGuard,
        SkipGuard,
        BlindGuard
    },
    operation::{
        CopyRequest,
        RemoveRequest,
        MoveRequest,
        CreateRequest,
        OperationWrapper,
        Request,
        OperationGeneratorInterface,
        OperationGenerator,
        OperationInterface,
    },
};

pub struct Container {
    virtual_fs  : FileSystemAdapter<VirtualFileSystem>,
    real_fs     : FileSystemAdapter<RealFileSystem>,
    operation_queue : VecDeque<OperationWrapper>
}

impl Container {
    pub fn new() -> Container {
        Container {
            virtual_fs: FileSystemAdapter(VirtualFileSystem::default()),
            real_fs:    FileSystemAdapter(RealFileSystem::default()),
            operation_queue: VecDeque::new()
        }
    }

    pub fn copy(&mut self, source: &Path, destination: &Path, guard: &mut dyn Guard) -> Result<(), DomainError> {
        let mut generator = OperationGenerator::new(
            CopyRequest::new(
                source.to_path_buf(),
                destination.to_path_buf()
            )
        );
        while let Some(operation) = generator.next(&self.virtual_fs)? {
            if guard.authorize(operation.request().target(), operation.strategy().into())? {
                self.emit(operation)?;
            }
        }
        Ok(())
    }

    pub fn mov(&mut self, source: &Path, destination: &Path, guard: &mut dyn Guard) -> Result<(), DomainError> {
        let mut generator = OperationGenerator::new(
            MoveRequest::new(
                source.to_path_buf(),
                destination.to_path_buf()
            )
        );
        while let Some(operation) = generator.next(&self.virtual_fs)? {
            if guard.authorize(operation.request().target(), operation.strategy().into())? {
                self.emit(operation)?;
            }
        }
        Ok(())
    }

    pub fn create(&mut self, path: &Path, kind: Kind, guard: &mut dyn Guard) -> Result<(), DomainError> {
        let mut generator = OperationGenerator::new(
            CreateRequest::new(
                path.to_path_buf(),
                kind
            )
        );
        while let Some(operation) = generator.next(&self.virtual_fs)? {
            if guard.authorize(operation.request().target(), operation.strategy().into())? {
                self.emit(operation)?;
            }
        }
        Ok(())
    }

    pub fn remove(&mut self, path: &Path, guard: &mut dyn Guard) -> Result<(), DomainError> {
        let mut generator = OperationGenerator::new(RemoveRequest::new(path.to_path_buf()));
        while let Some(operation) = generator.next(&self.virtual_fs)? {
            if guard.authorize(operation.request().target(), operation.strategy().into())? {
                self.emit(operation)?;
            }
        }
        Ok(())
    }

    pub fn emit<O: OperationInterface>(&mut self, operation: O) -> Result<(), DomainError> {
        operation.apply(&mut self.virtual_fs)?;
        self.operation_queue.push_back(operation.into());
        Ok(())
    }

    pub fn apply(&mut self) -> Result<(), DomainError> {
        while let Some(operation_wrapper) = self.operation_queue.pop_front() {
            operation_wrapper.apply(&mut self.real_fs)?;
        }
        self.reset();
        Ok(())
    }

    pub fn reset(&mut self) {
        self.virtual_fs.as_inner_mut().reset();
        self.operation_queue.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.virtual_fs.as_inner().is_empty()
    }

    pub fn to_json(&self) -> Result<String, DomainError> {
        let mut serializable : Vec<&OperationWrapper> = Vec::new();
        for operation in self.operation_queue.iter() {
            serializable.push(operation);
        }
        Ok(serde_json::to_string(&serializable)?)
    }

    pub fn emit_json(&mut self, json: String) -> Result<(), DomainError> {
        let operations : Vec<OperationWrapper> = serde_json::from_str(json.as_str()).unwrap();
        for operation in operations {
            operation.clone().apply(&mut self.virtual_fs)?;
            self.operation_queue.push_back(operation);
        }
        Ok(())
    }
}

impl ReadableFileSystem for Container {
    type Item = EntryAdapter<VirtualStatus>;

    fn read_dir(&self, path: &Path) -> Result<EntryCollection<Self::Item>,QueryError> {
        self.virtual_fs.read_dir(path)
    }

    fn status(&self, path: &Path) -> Result<Self::Item, QueryError> {
        self.virtual_fs.status(path)
    }

    fn is_directory_empty(&self, path: &Path) -> Result<bool, QueryError> {
        self.virtual_fs.is_directory_empty(path)
    }
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        sample::Samples,
        Entry,
        guard::ZealousGuard
    };

    #[test]
    fn copy_directory_recursively() {
        let chroot = Samples::init_simple_chroot("container_copy_directory_recursively");
        let mut container = Container::new();

        container.copy(
            &chroot.join("RDIR"),
            &chroot.join("COPIED"),
            &mut ZealousGuard
        ).unwrap();

        assert!(container.status(chroot.join("COPIED").as_path()).unwrap().exists());
        assert!(container.status(chroot.join("COPIED/RFILEA").as_path()).unwrap().exists());
        assert!(container.status(chroot.join("COPIED/RFILEB").as_path()).unwrap().exists());
    }

    #[test]
    fn can_export_virtual_state_into_json_string() {
        let chroot = Samples::init_simple_chroot("can_export_virtual_state_into_json_string");
        let mut container = Container::new();
        container.copy(
            &chroot.join("RDIR"),
            &chroot.join("COPIED"),
            &mut ZealousGuard
        ).unwrap();

        let expected : String = format!(
            "[{{\"Copy\":{{\"strategy\":\"DirectoryCopy\",\"request\":{{\"source\":\"{}\",\"destination\":\"{}\"}}}}}}]",
            chroot.join("RDIR").to_string_lossy(),
            chroot.join("COPIED").to_string_lossy(),
        );

        assert_eq!(container.to_json().unwrap(), expected);
    }

    #[test]
    fn can_import_virtual_state_from_json_string() {
        let chroot = Samples::init_simple_chroot("can_import_virtual_state_from_json_string");
        let mut container_a = Container::new();
        container_a.copy(
            &chroot.join("RDIR"),
            &chroot.join("COPIED"),
            &mut ZealousGuard
        ).unwrap();

        assert!(!container_a.is_empty());
        let a_stat = container_a.status(chroot.join("COPIED").as_path()).unwrap();
        assert!(a_stat.exists());
        assert!(a_stat.is_dir());

        let mut container_b = Container::new();

        container_b.emit_json(container_a.to_json().unwrap()).unwrap();

        assert!(!container_b.is_empty());
        let b_stat = container_b.status(chroot.join("COPIED").as_path()).unwrap();
        assert!(b_stat.exists());
        assert!(b_stat.is_dir());
    }
}
