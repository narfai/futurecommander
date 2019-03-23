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

use std::env::current_exe;

use std::path::{ PathBuf, Path };

use futurecommandervfs::{ VirtualPath, VirtualKind, VirtualDelta, VirtualFileSystem, VirtualChildren };
use crate::operation::{ Operation, CopyOperation, MoveOperation, NewDirectoryOperation, NewFileOperation };

pub fn get_sample_path() -> PathBuf {
    current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples")
}

#[cfg(test)]
mod virtual_shell_tests {
    use super::*;

    #[test]
    fn virtual_shell_system_rm(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let b_path = sample_path.join(&Path::new("B"));

        vfs.remove(b_path.as_path());

        assert!(!vfs.exists(b_path.as_path()));
    }

    #[test]
    fn virtual_shell_system_cp_only(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        CopyOperation::new(
            sample_path.join("B").as_path(),
            sample_path.join("A").as_path(),
        ).execute(&mut vfs);

        let path = sample_path.join("A/B/D/B");
        CopyOperation::new(
            sample_path.join("A/B").as_path(),
            sample_path.join("A/B/D").as_path(),//TODO this kind of operation shouldn't be possible with move / remove
        ).execute(&mut vfs);



//        match vfs.read_dir(sample_path.join(&Path::new("A/B/D")).as_path()) {
//            Ok(virtual_children) => {
//                assert!(virtual_children.len() > 0);
//                virtual_children.contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/B/D/E"))));
//                virtual_children.contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/B/D/G"))));
//            },
//            Err(error) => panic!("Error : {}", error)
//        }

    }

    #[test]
    fn virtual_shell_system_cp_preserve_source_and_node_kind(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        CopyOperation::new(
            sample_path.join("B").as_path(),
            sample_path.join("A").as_path(),
        ).execute(&mut vfs);

        CopyOperation::new(
            sample_path.join("F").as_path(),
            sample_path.join("B").as_path(),
        ).execute(&mut vfs);

        CopyOperation::new(
            sample_path.join("B/F").as_path(),
            sample_path.join("B/D/E").as_path(),
        ).execute(&mut vfs);

        match vfs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Ok(virtual_children) => {
                assert!(virtual_children.len() > 0);
                virtual_children.contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/F"))));

            },
            Err(error) => panic!("Error : {}", error)
        }
    }

    #[test]
    fn virtual_shell_system_mv(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        let real_source = VirtualPath::from_path_buf(sample_path.join(&Path::new("F")));

        MoveOperation::new(
            real_source.as_identity(),
            sample_path.join(&Path::new("A")).as_path()
        ).execute(&mut vfs);

        MoveOperation::new(
            sample_path.join(&Path::new("A/F")).as_path(),
            sample_path.join(&Path::new("B")).as_path()
        ).execute(&mut vfs);

        MoveOperation::new(
            sample_path.join(&Path::new("B/F")).as_path(),
            sample_path.join(&Path::new("B/D/E")).as_path()
        ).execute(&mut vfs);

        assert!(
            vfs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path()).unwrap()
                .contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/F"))))
        );
        assert!(
            !vfs.read_dir(sample_path.join(&Path::new("A/")).as_path()).unwrap()
                .contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/F")))) //Parent
        );
        assert!(
            !vfs.read_dir(sample_path.join(&Path::new("B")).as_path()).unwrap()
                .contains(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/F"))))
        );
    }

    #[test]
    fn virtual_shell_system_mkdir(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        NewDirectoryOperation::new(
            sample_path.join(&Path::new("B/D/E/MKDIRED")).as_path()
        ).execute(&mut vfs);

        let virtual_children = vfs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path()).unwrap();

        assert!(virtual_children.len() > 0);
        assert!(
            virtual_children.contains(
                &VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/MKDIRED")))
            )
        );
    }

    #[test]
    fn virtual_shell_system_touch(){
        let sample_path = get_sample_path();
        let mut vfs = VirtualFileSystem::new();

        NewFileOperation::new(
            sample_path.join(&Path::new("B/D/E/TOUCHED")).as_path()
        ).execute(&mut vfs);

        match vfs.read_dir(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Ok(virtual_children) => {
                assert!(virtual_children.len() > 0);
                assert!(
                    virtual_children.contains(
                        &VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/TOUCHED")))
                    )
                );

            },
            Err(error) => panic!("Error : {}", error)
        }
    }
}
