use crate::VirtualPath;
use crate::VirtualFileSystem;
use std::path::Path;

pub fn touch(vfs: &mut VirtualFileSystem, identity: &Path) {
    vfs.read_virtual(identity);
    if !vfs.get_state().exists(identity) {
        vfs.add.attach(
            identity,
            None,
            false
        );
    }
}
