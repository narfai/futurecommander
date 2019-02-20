use crate::VirtualPath;
use crate::VirtualFileSystem;
use std::path::Path;

pub fn mkdir(vfs: &mut VirtualFileSystem, identity: &Path) {
    vfs.read_virtual(identity);
    match vfs.get_state().get(identity) {
        Some(virtual_existing) => { println!("Already exists {:?}", virtual_existing); },
        None => {
            vfs.add.attach(
                identity,
                None,
                true
            );
        }
    }
}
