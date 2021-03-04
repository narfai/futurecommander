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

use self::{ generator::CopyGeneratorState };

pub use self::{
    request::CopyRequest,
    strategy::CopyStrategy
};

type CopyOperation = Operation<CopyStrategy, CopyRequest>;
type CopyGenerator<'a, E> = OperationGenerator<CopyGeneratorState<'a, E>, CopyRequest>;


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod real_tests {
    use super::*;

    use crate::{
        sample::Samples,
        infrastructure::RealFileSystem,
        port::{ FileSystemAdapter },
        operation::{ OperationInterface, OperationGeneratorInterface },
    };

    #[test]
    fn copy_operation_dir(){
        let chroot = Samples::init_simple_chroot("operation_copy_operation_dir");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = OperationGenerator::new(CopyRequest::new(
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

        let mut generator = OperationGenerator::new(CopyRequest::new(
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

        let mut generator = OperationGenerator::new(CopyRequest::new(
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
        port::{ FileSystemAdapter },
        infrastructure::{ VirtualFileSystem },
        Kind,
        operation::{ OperationInterface, OperationGeneratorInterface },
    };

    #[test]
    fn virtual_copy_operation_directory(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = OperationGenerator::new(CopyRequest::new(
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

        let mut generator = OperationGenerator::new(CopyRequest::new(
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

        let mut generator = OperationGenerator::new(CopyRequest::new(
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

        let mut generator = OperationGenerator::new(CopyRequest::new(
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
