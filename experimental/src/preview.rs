/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

mod node;
mod read_filesystem;
mod write_filesystem;
mod internal;

pub use node::{ PreviewNode, PreviewNodeKind };

#[derive(Default)]
pub struct Preview {
    root: PreviewNode
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod test {
    use std::{
        collections::HashSet,
        path::{ PathBuf, Path }
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

        // let chroot = Chroot::new("read_dir_preview_iso_with_real");
        // let chroot_path = chroot.init_empty();
        //
        // let a = chroot.create_dir("A");
        // let a_d = chroot.create_file("A/D");
        // let a_e = chroot.create_file("A/E");
        // let c = chroot.create_file("C");
        //
        // let z = chroot_path.join("Z");
        // let c_d = chroot_path.join("C/D");
        // let c_e = chroot_path.join("C/E");
        //
        // let mut container = Container::default();
        // container.rename(&a, &z).unwrap();
        // container.rename(&c, &a).unwrap();
        // container.rename(&z, &c).unwrap();
        //
        // assert!(a.preview_is_a_file(&container));
        // assert!(c.preview_is_a_dir(&container));
        // assert!(c_d.preview_is_a_file(&container));
        // assert!(c_e.preview_is_a_file(&container));
        // assert!(! z.preview_exists(&container));
        //
        // chroot.clean();

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