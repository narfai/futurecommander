mod generator;
mod serializable_kind;
mod request;
mod scheduling;
mod strategy;

use crate::{
    operation::{
        generator::{ OperationGenerator },
        operation::{ Operation }
    }
};

use self::{ generator::CreateGeneratorState };

pub use self::{
    request::CreateRequest,
    strategy::CreateStrategy
};

type CreateOperation = Operation<CreateStrategy, CreateRequest>;
type CreateGenerator = OperationGenerator<CreateGeneratorState, CreateRequest>;



#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod real_tests {
    use super::*;

    use crate::{
        Kind,
        sample::Samples,
        port::{ FileSystemAdapter },
        infrastructure::{ RealFileSystem },
        operation::{ OperationInterface, OperationGeneratorInterface },
    };

    #[test]
    fn create_operation_directory(){
        let chroot = Samples::init_simple_chroot("create_operation_directory");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = OperationGenerator::new(
            CreateRequest::new(
                chroot.join("CREATED"),
                Kind::Directory
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(chroot.join("CREATED").is_dir());
        assert!(chroot.join("CREATED").exists());
    }


    #[test]
    fn create_operation_directory_recursive(){
        let chroot = Samples::init_simple_chroot("create_operation_directory_recursive");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = OperationGenerator::new(
            CreateRequest::new(
                chroot.join("CREATED/NESTED/DIRECTORY"),
                Kind::Directory
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(chroot.join("CREATED/NESTED/DIRECTORY").exists());
        assert!(chroot.join("CREATED/NESTED/DIRECTORY").is_dir());
    }

    #[test]
    fn create_operation_file(){
        let chroot = Samples::init_simple_chroot("create_operation_file");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = OperationGenerator::new(
            CreateRequest::new(
                chroot.join("CREATED"),
                Kind::File
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(chroot.join("CREATED").exists());
        assert!(chroot.join("CREATED").is_file());
    }

    #[test]
    fn create_operation_file_overwrite(){
        let chroot = Samples::init_simple_chroot("create_operation_file_overwrite");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let a_len = chroot.join("RDIR/RFILEA").metadata().unwrap().len();

        let mut generator = OperationGenerator::new(
            CreateRequest::new(
                chroot.join("RDIR/RFILEA"),
                Kind::File
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(chroot.join("RDIR/RFILEA").exists());
        assert_ne!(a_len, chroot.join("RDIR/RFILEA").metadata().unwrap().len());
    }
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod virtual_tests {
    use super::*;

    use crate::{
        Kind,
        sample::Samples,
        port::{ FileSystemAdapter, ReadableFileSystem, Entry },
        infrastructure::{ VirtualFileSystem },
        operation::{ OperationInterface, OperationGeneratorInterface },
    };


    #[test]
    fn virtual_create_operation_directory(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_directory");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = OperationGenerator::new(
            CreateRequest::new(
                chroot.join("CREATED"),
                Kind::Directory
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("CREATED").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(chroot.join("CREATED").as_path()).unwrap());
    }


    #[test]
    fn virtual_create_operation_directory_recursive(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_directory_recursive");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = OperationGenerator::new(
            CreateRequest::new(
                chroot.join("CREATED/NESTED/DIRECTORY"),
                Kind::Directory
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("CREATED/NESTED/DIRECTORY").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_directory(chroot.join("CREATED/NESTED/DIRECTORY").as_path()).unwrap());
    }

    #[test]
    fn virtual_create_operation_file(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_file");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = OperationGenerator::new(
            CreateRequest::new(
                chroot.join("CREATED"),
                Kind::File
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        assert!(fs.as_inner().virtual_state().unwrap().is_virtual(chroot.join("CREATED").as_path()).unwrap());
        assert!(fs.as_inner().virtual_state().unwrap().is_file(chroot.join("CREATED").as_path()).unwrap());
    }

    #[test]
    fn virtual_create_operation_file_overwrite(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_file_overwrite");
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = OperationGenerator::new(
            CreateRequest::new(
                chroot.join("RDIR/RFILEA"),
                Kind::File
            )
        );

        while let Some(operation) = generator.next(&fs).unwrap() {
            operation.apply(&mut fs).unwrap();
        }

        let entry = fs.status(chroot.join("RDIR/RFILEA").as_path()).unwrap();

        assert!(entry.exists());
        assert!(entry.is_file());
    }
}
