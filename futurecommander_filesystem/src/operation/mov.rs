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

use self::{ generator::MoveGeneratorState };

pub use self::{
    request::MoveRequest,
    strategy::MoveStrategy
};

type MoveOperation = Operation<MoveStrategy, MoveRequest>;
type MoveGenerator<'a, E> = OperationGenerator<MoveGeneratorState<'a, E>, MoveRequest>;


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
    fn move_operation_dir(){
        let chroot = Samples::init_simple_chroot("move_operation_dir");
        let mut fs = FileSystemAdapter(RealFileSystem::default());

        let mut generator = OperationGenerator::new(MoveRequest::new(
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

        let mut generator = OperationGenerator::new(MoveRequest::new(
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

        let mut generator = OperationGenerator::new(MoveRequest::new(
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

        let mut generator = OperationGenerator::new(MoveRequest::new(
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
        infrastructure::{ VirtualFileSystem },
        operation::{ OperationInterface, OperationGeneratorInterface },
    };

    #[test]
    fn virtual_move_operation_directory(){
        let samples_path = Samples::static_samples_path();
        let mut fs = FileSystemAdapter(VirtualFileSystem::default());

        let mut generator = OperationGenerator::new(MoveRequest::new(
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

        let mut generator = OperationGenerator::new(MoveRequest::new(
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

        let mut generator = OperationGenerator::new(MoveRequest::new(
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

        let mut generator = OperationGenerator::new(MoveRequest::new(
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
