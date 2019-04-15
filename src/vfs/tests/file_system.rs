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

extern crate futurecommander_vfs;

#[cfg_attr(tarpaulin, skip)]
mod fs_integration {
    use futurecommander_vfs::{
        RealFileSystem,
        Samples
    };

    #[test]
    pub fn copy_file_to_file() {
        let chroot = Samples::init_simple_chroot("copy_file_to_file");
        let fs = RealFileSystem::default();

        fs.copy_file_to_file(
            chroot.join("RDIR/RFILEA").as_path(),
            chroot.join("COPIED").as_path(),
            &|_read| { /*println!("read {}", read);*/ },
            false
        ).unwrap();

        assert!(chroot.join("COPIED").exists());
        assert!(chroot.join("COPIED").is_file());
        assert!(chroot.join("COPIED").metadata().unwrap().len() > 1);
    }

    #[test]
    pub fn create_file() {
        let chroot = Samples::init_simple_chroot("create_file");
        let fs = RealFileSystem::default();

        fs.create_file(chroot.join("FILE").as_path()).unwrap();

        assert!(chroot.join("FILE").exists());
        assert!(chroot.join("FILE").is_file());
    }

    #[test]
    pub fn create_directory() {
        let chroot = Samples::init_simple_chroot("create_directory");
        let fs = RealFileSystem::default();

        fs.create_directory(chroot.join("DIRECTORY").as_path(), false).unwrap();

        assert!(chroot.join("DIRECTORY").exists());
        assert!(chroot.join("DIRECTORY").is_dir());
    }

    #[test]
    pub fn remove_file() {
        let chroot = Samples::init_simple_chroot("remove_file");
        let fs = RealFileSystem::default();

        fs.remove_file(chroot.join("RDIR/RFILEA").as_path()).unwrap();

        assert!(!chroot.join("RDIR/RFILEA").exists());
    }

    #[test]
    pub fn remove_directory_recursively() {
        let chroot = Samples::init_simple_chroot("remove_directory");
        let fs = RealFileSystem::default();

        fs.remove_directory(chroot.join("RDIR").as_path()).unwrap();

        assert!(!chroot.join("RDIR").exists());
        assert!(!chroot.join("RDIR/RFILEA").exists());
        assert!(!chroot.join("RDIR/RFILEB").exists());
    }
}
