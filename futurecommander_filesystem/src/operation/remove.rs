mod generator;
mod request;
mod scheduling;
mod strategy;

use crate::{
    operation::{
        generator::{ OperationGenerator },
        operation::{ Operation }
    }
};

use self::{ generator::RemoveGeneratorState };

pub use self::{
    request::RemoveRequest,
    strategy::RemoveStrategy
};

type RemoveOperation = Operation<RemoveStrategy, RemoveRequest>;
type RemoveGenerator<'a, E> = OperationGenerator<RemoveGeneratorState<'a, E>, RemoveRequest>;


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod real_tests {
    use super::*;

    use crate::{
        sample::Samples,
        port::{ FileSystemAdapter },
        infrastructure::{ RealFileSystem },
        operation::{ OperationInterface, OperationGeneratorInterface },
    };

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
    use super::*;

    use crate::{
        sample::Samples,
        port::{ FileSystemAdapter },
        infrastructure::{ VirtualFileSystem },
        operation::{ OperationInterface, OperationGeneratorInterface }
    };

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
