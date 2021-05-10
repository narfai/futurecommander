/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

mod operation;
mod read_filesystem;
mod write_filesystem;

use crate::{ Result, Preview, FileSystem };

#[derive(Default)]
pub struct Container {
    operation_list: Vec<operation::Operation>,
    preview: Preview,
    filesystem: FileSystem
}

impl Container {
    pub fn apply(&mut self) -> Result<()> {
        for op in &self.operation_list {
            op.apply(&mut self.filesystem)?
        }
        self.operation_list = Vec::new();
        Ok(())
    }
}

//TODO apply
//TODO to_json
//TODO apply_json

//TODO guards & so on ...


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
#[cfg(unix)]
mod test {
    use std::{
        path::{ PathBuf, Path },
        io::{ stdout }
    };
    use crate::{
        Container,
        PathExt,
        WriteFileSystem,
        sample::*
    };
    use super::*;

    #[test]
    fn file_dir_interversion() {
        /*
        FROM
        ├── A (Directory)
        │   ├── D (File)
        │   └── E (File)
        └── C (File)

        mv A Z
        mv C A
        mv Z C

        TO
        ├── A (File)
        └── C (Directory)
            ├── D (File)
            └── E (File)
        */

        let chroot = Chroot::new("file_dir_interversion");
        let chroot_path = chroot.init_empty();

        let a = chroot.create_dir("A");
        let _a_d = chroot.create_file("A/D");
        let _a_e = chroot.create_file("A/E");
        let c = chroot.create_file("C");

        let z = chroot_path.join("Z");
        let c_d = chroot_path.join("C/D");
        let c_e = chroot_path.join("C/E");

        let mut container = Container::default();
        container.rename(&a, &z).unwrap();
        container.rename(&c, &a).unwrap();
        container.rename(&z, &c).unwrap();

        assert!(a.preview_is_a_file(&container));
        assert!(c.preview_is_a_dir(&container));
        assert!(c_d.preview_is_a_file(&container));
        assert!(c_e.preview_is_a_file(&container));
        assert!(! z.preview_exists(&container));

        chroot.clean();
    }

    #[test]
    fn file_file_interversion() {
        /*
        FROM
        ├── A (File) "A"
        └── C (File) "C"

        mv A Z
        mv C A
        mv Z C

        TO
        ├── A (File) "C"
        └── C (File) "A"
        */

        let chroot = Chroot::new("file_file_interversion");
        let chroot_path = chroot.init_empty();

        let a = chroot.create_file("A");
        let c = chroot.create_file("C");
        let z = chroot_path.join("Z");

        let mut container = Container::default();
        container.rename(&a, &z).unwrap();
        container.rename(&c, &a).unwrap();
        container.rename(&z, &c).unwrap();

        assert!(a.preview_is_a_file(&container));
        assert!(c.preview_is_a_file(&container));
        assert!(! z.preview_exists(&container));

        chroot.clean();
    }

    #[test]
    fn dir_dir_interversion() {
        /*
        FROM
        ├── A (Directory)
        │   ├── D (File)
        │   └── E (File)
        └── B (Directory)
               ├── F (File)
               └── G (File)

        mv A Z
        mv B A
        mv Z B

        TO
        ├── A (Directory)
        │   ├── F (File)
        │   └── G (File)
        └── B (Directory)
            ├── D (File)
            └── E (File)
        */


        let chroot = Chroot::new("dir_dir_interversion");
        let chroot_path = chroot.init_empty();

        let a = chroot.create_dir("A");
        let _a_d = chroot.create_file("A/D");
        let _a_e = chroot.create_file("A/E");

        let b = chroot.create_dir("B");
        let _b_f = chroot.create_file("B/F");
        let _b_g = chroot.create_file("B/G");

        let z = chroot_path.join("Z");
        let a_f = chroot_path.join("A/F");
        let a_g = chroot_path.join("A/G");
        let b_d = chroot_path.join("B/D");
        let b_e = chroot_path.join("B/E");

        let mut container = Container::default();
        container.rename(&a, &z).unwrap();
        container.rename(&b, &a).unwrap();
        container.rename(&z, &b).unwrap();

        assert!(a.preview_is_a_dir(&container));
        assert!(b.preview_is_a_dir(&container));
        assert!(a_f.preview_is_a_file(&container));
        assert!(a_g.preview_is_a_file(&container));
        assert!(b_d.preview_is_a_file(&container));
        assert!(b_e.preview_is_a_file(&container));
        assert!(! z.preview_exists(&container));

        chroot.clean();
    }

    #[test]
    fn multi_level_interversion() {
        /*
        FROM
        ├── A (Directory)
        │   ├── D (File)
        │   └── E (File)
        └── B (Directory)
            ├── F (File)
            └── G (File)

        mv A B/A
        cp B A

        TO
        ├── A (Directory)
        │   ├── A (Directory)
        │   │   ├── D (File)
        │   │   └── E (File)
        │   ├── F (File)
        │   └── G (File)
        └── B (Directory)
            ├── A (Directory)
            │   ├── D (File)
            │   └── E (File)
            ├── F (File)
            └── G (File)
        */

        // let chroot = Chroot::new("multi_level_interversion");
        // let chroot_path = chroot.init_empty();
        //
        // let a = chroot.create_dir("A");
        // let _a_d = chroot.create_file("A/D");
        // let _a_e = chroot.create_file("A/E");
        //
        // let b = chroot.create_dir("B");
        // let _b_f = chroot.create_file("B/F");
        // let _b_g = chroot.create_file("B/G");
        //
        // let a_a = chroot_path.join("A/A");
        // let a_f = chroot_path.join("A/F");
        // let a_g = chroot_path.join("A/G");
        // let a_a_d = chroot_path.join("A/A/D");
        // let a_a_e = chroot_path.join("A/A/E");
        //
        // let b_f = chroot_path.join("B/F");
        // let b_g = chroot_path.join("B/G");
        // let b_a = chroot_path.join("B/A");
        // let b_a_d = chroot_path.join("B/A/D");
        // let b_a_e = chroot_path.join("B/A/E");
        //
        // let mut container = Container::default();
        // container.rename(&a, &b_a).unwrap();
        // TODO recursive Copy

        // chroot.clean();
    }

    #[test]
    fn copy_then_delete_then_create() {
        /*
        FROM
        └── A (Directory)
            ├── D (File)
            └── E (File)

        cp A B
        rm A
        touch A

        TO
        ├── B (Directory)
        │   ├── F (File)
        │   └── G (File)
        └── A (File)
        */
    }
}