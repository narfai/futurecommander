/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommander.
 *
 * FutureCommander is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommander is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommander.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::fs::{ File, create_dir, remove_dir_all, remove_file };
use std::io::Write;

use std::env::current_exe;
use std::path::{ PathBuf, Path };

use crate::*;

pub fn get_sample_path() -> PathBuf {
    current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("samples")
}

#[cfg(test)]
mod virtual_file_system_tests {
    use super::*;

    #[test]
    fn test_samples_ok(){
        let sample_path = get_sample_path();
        assert!(sample_path.join("A").exists());
        assert!(sample_path.join("B").exists());
        assert!(sample_path.join("F").exists());
        assert!(sample_path.join("B/D").exists());
        assert!(sample_path.join("B/D/E").exists());
        assert!(sample_path.join("B/D/G").exists());

        assert!(sample_path.join("A").is_dir());
        assert!(sample_path.join("B").is_dir());
        assert!(sample_path.join("F").is_file());
        assert!(sample_path.join("B/D").is_dir());
        assert!(sample_path.join("B/D/E").is_dir());
        assert!(sample_path.join("B/D/G").is_dir());
    }

    #[test]
    fn resolve(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let a = sample_path.join("A");
        let b = sample_path.join("B");
        let ab = sample_path.join("A/B");
        let abcdef = sample_path.join("A/B/C/D/E/F");

        Virtual(Copy::new(
            b.as_path(),
            a.as_path(),
            None
        )).execute(&mut vfs).unwrap();

        let virtual_state = vfs.virtual_state().unwrap();

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(ab.as_path()).unwrap().unwrap()
        );
        assert_eq!(
            b.join("C/D/E/F").as_path(),
            virtual_state.resolve(abcdef.as_path()).unwrap().unwrap()
        );
    }

    #[test]
    fn resolve_through(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let a = sample_path.join("A");
        let b = sample_path.join("B");

        let ab = sample_path.join("A/B");
        let bd = sample_path.join("B/D");

        Virtual(Copy::new(
            b.as_path(),
            a.as_path(),
            None)
        ).execute(&mut vfs).unwrap();

        Virtual(Copy::new(
            ab.as_path(),
            bd.as_path(),
            None
        )).execute(&mut vfs).unwrap();

        let virtual_state = vfs.virtual_state().unwrap();

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(ab.as_path()).unwrap().unwrap()
        );

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(bd.join("B").as_path()).unwrap().unwrap()
        );
    }

    #[test]
    fn stat_none_if_deleted(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();
        let a = sample_path.join("A");

        assert!(
            Virtual(Status::new(a.as_path()))
                .retrieve(&vfs)
                .unwrap()
                .virtual_identity()
                .is_some()
        );

        Virtual(Remove::new(a.as_path()))
            .execute(&mut vfs).unwrap();


        assert!(
            Virtual(Status::new(a.as_path()))
                .retrieve(&vfs)
                .unwrap()
                .virtual_identity()
                .is_none()
        );
    }

    #[test]
    fn stat_virtual(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();
        let z = sample_path.join("Z");

        Virtual(Create::new(
            z.as_path(),
            VirtualKind::Directory
        )).execute(&mut vfs).unwrap();

        let stated = Virtual(Status::new(z.as_path()))
            .retrieve(&vfs)
            .unwrap()
            .virtual_identity()
            .unwrap();

        assert_eq!(stated.to_kind(), VirtualKind::Directory);
        assert_eq!(stated.as_identity(), z);
        assert!(stated.as_source().is_none())
    }

    #[test]
    fn stat_real(){
        let sample_path = get_sample_path();
        let vfs = VirtualFileSystem::new();
        let a = sample_path.join("A");

        let stated = Virtual(Status::new(a.as_path()))
            .retrieve(&vfs)
            .unwrap()
            .virtual_identity()
            .unwrap();

        assert_eq!(stated.to_kind(), VirtualKind::Directory);
        assert_eq!(stated.as_identity(), a.as_path());
        assert_eq!(stated.as_source(), Some(a.as_path()))
    }

    #[test]
    fn stat_related(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();
        let abdg = sample_path.join("A/B/D/G");//Note : should exists in samples

        Virtual(Copy::new(
            sample_path.join("B").as_path(),
            sample_path.join("A").as_path(),
            None
        )).execute(&mut vfs).unwrap();

        let stated = Virtual(Status::new(abdg.as_path()))
            .retrieve(&vfs)
            .unwrap()
            .virtual_identity()
            .unwrap();

        assert_eq!(stated.to_kind(), VirtualKind::Directory);
        assert_eq!(stated.as_identity(), abdg.as_path());
        assert_eq!(stated.as_source(), Some(sample_path.join("B/D/G").as_path()))
    }

    //Error testing

    #[test]
    fn copy_or_move_directory_into_itself_must_not_be_allowed(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let source = sample_path.join("B");
        let destination = sample_path.join("B/D");

        match Virtual(Copy::new(
            source.as_path(),
            destination.as_path(),
            None
        )).execute(&mut vfs) {
            Err(VfsError::CopyIntoItSelft(err_source, err_destination)) => {
                assert_eq!(source.as_path(), err_source.as_path());
                assert_eq!(destination.as_path(), err_destination.as_path());
            }
            Err(error) => panic!("{}", error),
            Ok(_) => panic!("Should not be able to copy into itself")
        };
    }

    #[test]
    fn copy_source_does_not_exists(){}

    #[test]
    fn copy_destination_does_not_exists(){}

    #[test]
    fn create_already_exists(){}

    #[test]
    fn remove_does_not_exists(){}

    // No-Backwards tests

    #[test]
    fn reset_empty(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        Virtual(Create::new(
            sample_path.join("VIRTUALA").as_path(),
            VirtualKind::File
        )).execute(&mut vfs).unwrap();

        Virtual(Create::new(
            sample_path.join("VIRTUALB").as_path(),
            VirtualKind::Directory)
        ).execute(&mut vfs).unwrap();

        Virtual(Remove::new(
            sample_path.join("VIRTUALB").as_path()
        )).execute(&mut vfs).unwrap();

        assert!(vfs.has_addition());
        assert!(vfs.has_subtraction());

        vfs.reset();

        assert!(!vfs.has_addition());
        assert!(!vfs.has_subtraction());
    }

    #[test]
    fn status_exists(){}

    #[test]
    fn status_exists_virtually(){}

    #[test]
    fn status_exists_through_virtual_parent(){}

    #[test]
    fn status_exists_not_exists(){}

    #[test]
    fn status_exists_deleted(){}

    #[test]
    fn status_exists_removed_virtually(){}

    #[test]
    fn create(){}

    #[test]
    fn remove(){}

    #[test]
    fn copy(){}

    #[test]
    fn copy_with_rename(){

    }
}

pub fn init_real_samples_idempotently(arbitrary_identifier: &str) -> PathBuf {
    let chroot = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap()
        .join(Path::new("samples").join(format!("real_tests_{}", arbitrary_identifier)));

    if chroot.exists() {
        remove_dir_all(chroot.as_path()).unwrap();
    }

    create_dir(chroot.as_path()).unwrap();
    assert!(chroot.exists());

    create_dir(chroot.join("RDIR")).unwrap();
    assert!(chroot.join("RDIR").exists());

    let mut file = File::create(chroot.join("RDIR").join("RFILEA")).unwrap();
    file.write_all(b"[A]Gummies candy biscuit jelly cheesecake. Liquorice gingerbread oat cake marzipan gummies muffin. Sweet liquorice dessert. Caramels chupa chups lollipop dragee gummies sesame snaps. Tootsie roll lollipop chocolate cake chocolate jelly jelly-o sesame snaps gummies. Topping topping bear claw candy canes bonbon muffin cupcake. Tart croissant liquorice croissant tootsie roll cupcake powder icing. Dessert souffle cake ice cream pie cookie. Brownie cotton candy pudding ice cream pudding cotton candy gingerbread gummi bears. Dragee biscuit croissant chocolate bar cheesecake marshmallow wafer macaroon. Sweet roll chupa chups gummi bears oat cake halvah marshmallow souffle pie. Jujubes pastry fruitcake macaroon jelly lemon drops chocolate cake chocolate cake."
    ).unwrap();
    assert!(chroot.join("RDIR").join("RFILEA").exists());

    let mut file = File::create(chroot.join("RDIR").join("RFILEB")).unwrap();

    file.write_all(b"[B]Gummies candy biscuit jelly cheesecake. Liquorice gingerbread oat cake marzipan gummies muffin. Sweet liquorice dessert. Caramels chupa chups lollipop dragee gummies sesame snaps. Tootsie roll lollipop chocolate cake chocolate jelly jelly-o sesame snaps gummies. Topping topping bear claw candy canes bonbon muffin cupcake. Tart croissant liquorice croissant tootsie roll cupcake powder icing. Dessert souffle cake ice cream pie cookie. Brownie cotton candy pudding ice cream pudding cotton candy gingerbread gummi bears. Dragee biscuit croissant chocolate bar cheesecake marshmallow wafer macaroon. Sweet roll chupa chups gummi bears oat cake halvah marshmallow souffle pie. Jujubes pastry fruitcake macaroon jelly lemon drops chocolate cake chocolate cake."
    ).unwrap();
    assert!(chroot.join("RDIR").join("RFILEB").exists());

    chroot
}


#[cfg(test)]
mod real_file_system_tests {
    use super::*;

    #[test]
    pub fn copy_file_to_file(){
        let chroot = init_real_samples_idempotently("copy_file_to_file");
        let fs = RealFileSystem::new(false);

        fs.copy_file_to_file(
            chroot.join("RDIR/RFILEA").as_path(),
            chroot.join("COPIED").as_path(),
            &|_read| { /*println!("read {}", read);*/ }
        ).unwrap();

        assert!(chroot.join("COPIED").exists());
        assert!(chroot.join("COPIED").is_file());
        assert!(chroot.join("COPIED").metadata().unwrap().len() > 1);
    }

    #[test]
    pub fn copy_file_to_directory(){
        let chroot = init_real_samples_idempotently("copy_file_to_directory");
        let fs = RealFileSystem::new(false);

        fs.copy_file_into_directory(
            chroot.join("RDIR/RFILEA").as_path(),
            chroot.as_path(),
            &|_read| { /*println!("read {}", read);*/ }
        ).unwrap();

        assert!(chroot.join("RFILEA").exists());
        assert!(chroot.join("RFILEA").is_file());
    }

    #[test]
    pub fn copy_directory_to_directory(){
        let chroot = init_real_samples_idempotently("copy_directory_to_directory");
        let fs = RealFileSystem::new(false);

        fs.copy_directory_into_directory(
            chroot.join("RDIR").as_path(),
            chroot.join("COPIED").as_path(),
            &|_read| { /*println!("read {}", read);*/ }
        ).unwrap();

        assert!(chroot.join("COPIED").exists());
        assert!(chroot.join("COPIED").is_dir());
        assert!(chroot.join("COPIED/RFILEA").exists());
        assert!(chroot.join("COPIED/RFILEB").exists());
    }

    #[test]
    pub fn create_file(){
        let chroot = init_real_samples_idempotently("create_file");
        let fs = RealFileSystem::new(false);

        fs.create_file(chroot.join("FILE").as_path()).unwrap();

        assert!(chroot.join("FILE").exists());
        assert!(chroot.join("FILE").is_file());
    }

    #[test]
    pub fn create_directory(){
        let chroot = init_real_samples_idempotently("create_directory");
        let fs = RealFileSystem::new(false);

        fs.create_directory(chroot.join("DIRECTORY").as_path()).unwrap();

        assert!(chroot.join("DIRECTORY").exists());
        assert!(chroot.join("DIRECTORY").is_dir());
    }

    #[test]
    pub fn remove_file(){
        let chroot = init_real_samples_idempotently("remove_file");
        let fs = RealFileSystem::new(false);

        fs.remove_file(chroot.join("RDIR/RFILEA").as_path()).unwrap();

        assert!(!chroot.join("RDIR/RFILEA").exists());
    }

    #[test]
    pub fn remove_directory(){
        let chroot = init_real_samples_idempotently("remove_directory");
        let fs = RealFileSystem::new(false);

        fs.remove_directory(chroot.join("RDIR").as_path()).unwrap();

        assert!(!chroot.join("RDIR").exists());
        assert!(!chroot.join("RDIR/RFILEA").exists());
        assert!(!chroot.join("RDIR/RFILEB").exists());
    }
}
