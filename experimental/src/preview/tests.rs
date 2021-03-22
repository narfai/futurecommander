use super::{
    Preview
};

use crate::sample::*;

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

    let chroot = init_empty_chroot("preview_file_dir_interversion");
    let preview = Preview::default();


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
    │   ├── D (File)
    │   └── E (File)
    └── B (Directory)
           ├── F (File)
           └── G (File)

    mv A Z
    mv B A
    mv Z B

    TO
    ├── A (Directory)
    │   ├── F (File)
    │   └── G (File)
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
    │   ├── D (File)
    │   └── E (File)
    └── B (Directory)
           ├── F (File)
           └── G (File)

    mv A B/A
    cp B A

    TO
    ├── A (Directory)
    │   ├── A (Directory)
    │   │   ├── D (File)
    │   │   └── E (File)
    │   ├── F (File)
    │   └── G (File)
    └── B (Directory)
           ├── A (Directory)
           │   ├── D (File)
           │   └── E (File)
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
    │   ├── F (File)
    │   └── G (File)
    └── A (File)
    */
}
