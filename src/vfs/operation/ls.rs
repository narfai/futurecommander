use crate::VirtualPath;
use crate::VirtualFileSystem;
use std::path::Path;

pub fn ls(vfs: &mut VirtualFileSystem, identity: &Path){
    match vfs.read_dir(identity) {
        Ok(virtual_children) => {
            for child in virtual_children {
                println!("{:?}", child);
            }
        },
        Err(error) => println!("Error : {}", error)
    }
}
