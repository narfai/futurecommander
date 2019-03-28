/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommanderVfs.
 *
 * FutureCommanderVfs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommanderVfs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommanderVfs.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::env::current_exe;

use std::path::{ PathBuf };

use crate::representation::{ VirtualKind };
use crate::file_system::{ VirtualFileSystem };
use crate::errors::VfsError;

pub fn get_sample_path() -> PathBuf {
    current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("samples")
}

#[cfg(test)]
mod virtual_file_system_tests {
    use super::*;

    #[test]
    fn virtual_file_system_test_samples_ok(){
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
    fn virtual_file_system_resolve(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let a = sample_path.join("A");
        let b = sample_path.join("B");
        let ab = sample_path.join("A/B");
        let abcdef = sample_path.join("A/B/C/D/E/F");

        vfs.copy(b.as_path(), a.as_path(), None).unwrap();

        let virtual_state = vfs.get_virtual_state().unwrap();

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
    fn virtual_file_system_resolve_through(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let a = sample_path.join("A");
        let b = sample_path.join("B");

        let ab = sample_path.join("A/B");
        let bd = sample_path.join("B/D");

        vfs.copy(b.as_path(), a.as_path(), None).unwrap();
        vfs.copy(ab.as_path(), bd.as_path(), None).unwrap();

        let virtual_state = vfs.get_virtual_state().unwrap();

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
    fn virtual_file_system_stat_none_if_deleted(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();
        let a = sample_path.join("A");

        assert!(vfs.stat(a.as_path()).unwrap().is_some());

        vfs.remove(a.as_path()).unwrap();

        assert!(vfs.stat(a.as_path()).unwrap().is_none())
    }

    #[test]
    fn virtual_file_system_stat_virtual(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();
        let z = sample_path.join("Z");

        vfs.create(z.as_path(),VirtualKind::Directory).unwrap();

        let stated = vfs.stat(z.as_path()).unwrap().unwrap();
        assert_eq!(stated.to_kind(), VirtualKind::Directory);
        assert_eq!(stated.as_identity(), z);
        assert!(stated.as_source().is_none())
    }

    #[test]
    fn virtual_file_system_stat_real(){
        let sample_path = get_sample_path();
        let vfs = VirtualFileSystem::new();
        let a = sample_path.join("A");

        let stated = vfs.stat(a.as_path()).unwrap().unwrap();
        assert_eq!(stated.to_kind(), VirtualKind::Directory);
        assert_eq!(stated.as_identity(), a.as_path());
        assert_eq!(stated.as_source(), Some(a.as_path()))
    }

    #[test]
    fn virtual_file_system_stat_related(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();
        let abdg = sample_path.join("A/B/D/G");//Note : should exists in samples

        vfs.copy(
            sample_path.join("B").as_path(),
            sample_path.join("A").as_path(),
            None
        ).unwrap();

        let stated = vfs.stat(abdg.as_path()).unwrap();

        assert!(stated.is_some());

        let stated = stated.unwrap();

        assert_eq!(stated.to_kind(), VirtualKind::Directory);
        assert_eq!(stated.as_identity(), abdg.as_path());
        assert_eq!(stated.as_source(), Some(sample_path.join("B/D/G").as_path()))
    }

    //Error testing

    #[test]
    fn virtual_file_system_copy_or_move_directory_into_itself_must_not_be_allowed(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let source = sample_path.join("B");
        let destination = sample_path.join("B/D");

        match vfs.copy(
            source.as_path(),
            destination.as_path(),
            None
        ) {
            Err(VfsError::CopyIntoItSelft(err_source, err_destination)) => {
                assert_eq!(source.as_path(), err_source.as_path());
                assert_eq!(destination.as_path(), err_destination.as_path());
            }
            Err(error) => panic!("{}", error),
            Ok(_) => panic!("Should not be able to copy into itself")
        };
    }

    #[test]
    fn virtual_file_system_copy_source_does_not_exists(){}

    #[test]
    fn virtual_file_system_copy_destination_does_not_exists(){}

    #[test]
    fn virtual_file_system_create_already_exists(){}

    #[test]
    fn virtual_file_system_remove_does_not_exists(){}

    // No-Backwards tests

    #[test]
    fn virtual_file_system_reset_empty(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();
        vfs.create(sample_path.join("VIRTUALA").as_path(), VirtualKind::File).unwrap();
        vfs.create(sample_path.join("VIRTUALB").as_path(), VirtualKind::Directory).unwrap();

        assert!(!vfs.is_empty());

        vfs.reset();

        assert!(vfs.is_empty());
    }

    #[test]
    fn virtual_file_system_status_exists(){}

    #[test]
    fn virtual_file_system_status_exists_virtually(){}

    #[test]
    fn virtual_file_system_status_exists_through_virtual_parent(){}

    #[test]
    fn virtual_file_system_status_exists_not_exists(){}

    #[test]
    fn virtual_file_system_status_exists_deleted(){}

    #[test]
    fn virtual_file_system_status_exists_removed_virtually(){}

    #[test]
    fn virtual_file_system_create(){}

    #[test]
    fn virtual_file_system_remove(){}

    #[test]
    fn virtual_file_system_copy(){}

    #[test]
    fn virtual_file_system_copy_with_rename(){

    }
}
