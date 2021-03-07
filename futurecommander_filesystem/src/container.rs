// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{
    path::{ Path },
    collections::vec_deque::VecDeque
};

use crate::{
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
        ZealousGuard,
        Guard
    },
    operation::{
        Operation,
        OperationWrapper,
        Request,
        OperationGeneratorInterface,
        OperationGenerator,
        OperationInterface,
        Scheduler
    },
};

/*
== TODO test ==
User choices about capabilities should be preserved between emit & apply

*/

#[derive(Debug)]
//pub struct OperationQueue(VecDeque<OperationWrapper>);
pub struct OperationQueue(VecDeque<FileSystemOperation>);

impl Default for OperationQueue {
    fn default() -> OperationQueue {
        OperationQueue(VecDeque::new())
    }
}

impl OperationQueue {
    pub fn pop_front(&mut self) -> Option<FileSystemOperation>{
        self.0.pop_front()
    }

    pub fn push_back(&mut self, operation: FileSystemOperation){
        self.0.push_back(operation)
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        let mut serializable : Vec<&FileSystemOperation> = Vec::new();
        for operation in self.0.iter() {
            serializable.push(operation);
        }
        serde_json::to_string(&serializable)
    }
}

#[derive(Debug)]
pub struct Container {
    virtual_fs  : FileSystemAdapter<VirtualFileSystem>,
    real_fs     : FileSystemAdapter<RealFileSystem>,
    operation_queue : OperationQueue
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Container {
    pub fn new() -> Container {
        Container {
            virtual_fs: FileSystemAdapter(VirtualFileSystem::default()),
            real_fs:    FileSystemAdapter(RealFileSystem::default()),
            operation_queue: OperationQueue::default()
        }
    }

    pub fn apply(&mut self) -> Result<(), DomainError> {
        while let Some(operation) = self.operation_queue.pop_front() {
            operation.atomize(&self.real_fs, Box::new(ZealousGuard))?
                .apply(&mut self.real_fs)?;
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

    pub fn vfs(&self) -> &FileSystemAdapter<VirtualFileSystem> {
        &self.virtual_fs
    }

    pub fn rfs(&self) -> &FileSystemAdapter<RealFileSystem> {
        &self.real_fs
    }

    pub fn to_json(&self) -> Result<String, DomainError> {
        Ok(self.operation_queue.serialize()?)
    }

    pub fn emit_json(&mut self, json: String) -> Result<(), DomainError> {
        let operations : Vec<FileSystemOperation> = serde_json::from_str(json.as_str()).unwrap();
        for operation in operations {
            self.emit(operation.clone(), Box::new(ZealousGuard))?;
            self.operation_queue.push_back(operation);
        }
        Ok(())
    }

    fn emit<R: Request, G: Guard>(&mut self, request: R, guard: G) -> Result<(), DomainError> {
        let mut generator = OperationGenerator::new(request);
        while let Some(operation) = generator.next(self) {
            if guard.authorize(operation.strategy().into(), false, operation.request().target)? {
                operation.apply(self.virtual_fs)?;
                self.operation_queue.push_back(operation);
            }
        }
        Ok(())
    }

    /* pub fn emit_operation(&mut self, operation: OperationWrapper) -> Result<(), DomainError>{
        operation.apply(&mut self.virtual_fs)?;
        self.operation_queue.push_back(operation);
        Ok(())
    } */
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
        event::CopyOperationDefinition,
        sample::Samples,
        Entry
    };

    #[test]
    fn copy_directory_recursively() {
        let chroot = Samples::init_simple_chroot("container_copy_directory_recursively");
        let mut container = Container::new();
        let event = FileSystemOperation::copy(
            CopyOperationDefinition::new(
                chroot.join("RDIR").as_path(),
                chroot.join("COPIED").as_path(),
                false,
                false
            )
        );

        container.emit(event, Box::new(ZealousGuard)).unwrap();

        assert!(container.status(chroot.join("COPIED").as_path()).unwrap().exists());
        assert!(container.status(chroot.join("COPIED/RFILEA").as_path()).unwrap().exists());
        assert!(container.status(chroot.join("COPIED/RFILEB").as_path()).unwrap().exists());
    }

    #[test]
    fn can_export_virtual_state_into_json_string() {
        let chroot = Samples::init_simple_chroot("can_export_virtual_state_into_json_string");
        let mut container = Container::new();
        let event = FileSystemOperation::copy(
            CopyOperationDefinition::new(
                chroot.join("RDIR").as_path(),
                chroot.join("COPIED").as_path(),
                false,
                false
            )
        );

        container.emit(event, Box::new(ZealousGuard)).unwrap();
        let expected : String = format!(
            "[{{\"Copy\":[{{\"source\":\"{}\",\"destination\":\"{}\",\"merge\":false,\"overwrite\":false}},{{}}]}}]",
            chroot.join("RDIR").to_string_lossy(),
            chroot.join("COPIED").to_string_lossy(),
        );

        assert_eq!(container.to_json().unwrap(), expected);
    }

    #[test]
    fn can_import_virtual_state_from_json_string() {
        let chroot = Samples::init_simple_chroot("can_import_virtual_state_from_json_string");
        let mut container_a = Container::new();
        let event = FileSystemOperation::copy(
            CopyOperationDefinition::new(
                chroot.join("RDIR").as_path(),
                chroot.join("COPIED").as_path(),
                false,
                false
            )
        );

        container_a.emit(event, Box::new(ZealousGuard)).unwrap();
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
