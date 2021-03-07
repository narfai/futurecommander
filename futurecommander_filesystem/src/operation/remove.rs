// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

mod generator;
mod request;
mod scheduling;
mod strategy;

use super::{
    OperationGenerator,
    Operation
};
pub use self::{
    generator::RemoveGeneratorState,
    request::RemoveRequest,
    strategy::RemoveStrategy
};

type RemoveOperation = Operation<RemoveStrategy, RemoveRequest>;
type RemoveGenerator<'a, E> = OperationGenerator<RemoveGeneratorState<'a, E>, RemoveRequest>;


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod real_tests {
    use crate::{
        sample::Samples,
        infrastructure::{
            RealFileSystem,
            FileSystemAdapter
        },
    };
    use super::super::{
        OperationInterface,
        OperationGeneratorInterface
    };
    use super::*;


    #[test]
    fn operation_remove_operation_file() {
        let chroot = Samples::init_simple_chroot("operation_remove_operation_file");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = OperationGenerator::new(RemoveRequest::new(
            chroot.join("RDIR/RFILEA")
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!chroot.join("RDIR/RFILEA").exists());
    }

    #[test]
    fn operation_remove_operation_directory() {
        let chroot = Samples::init_simple_chroot("operation_remove_operation_directory");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = OperationGenerator::new(RemoveRequest::new(
            chroot.join("RDIR3")
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!chroot.join("RDIR3").exists());
    }

    #[test]
    fn operation_remove_operation_directory_recursive() {
        let chroot = Samples::init_simple_chroot("operation_remove_operation_directory_recursive");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = OperationGenerator::new(RemoveRequest::new(
            chroot.join("RDIR")
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!chroot.join("RDIR").exists());
    }
}



#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod virtual_tests {
    use crate::{
        sample::Samples,
        infrastructure::{
            VirtualFileSystem,
            FileSystemAdapter
        }
    };
    use super::super::{
        OperationInterface,
        OperationGeneratorInterface
    };
    use super::*;

    #[test]
    fn operation_virtual_remove_operation_file() {
        let chroot = Samples::init_simple_chroot("operation_virtual_remove_operation_file");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = OperationGenerator::new(RemoveRequest::new(
            chroot.join("RDIR/RFILEA")
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("RDIR/RFILEA").as_path()).unwrap());
    }

    #[test]
    fn operation_virtual_remove_operation_directory() {
        let chroot = Samples::init_simple_chroot("operation_virtual_remove_operation_directory");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = OperationGenerator::new(RemoveRequest::new(
            chroot.join("RDIR3")
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("RDIR3").as_path()).unwrap());
    }

    #[test]
    fn operation_virtual_remove_operation_directory_recursive() {
        let chroot = Samples::init_simple_chroot("operation_virtual_remove_operation_directory_recursive");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = OperationGenerator::new(RemoveRequest::new(
            chroot.join("RDIR")
        ));

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(!fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("RDIR").as_path()).unwrap());
    }
}
