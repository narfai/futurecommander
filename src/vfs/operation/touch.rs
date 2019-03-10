use crate::VirtualFileSystem;
use std::path::Path;

pub fn touch(vfs: &mut VirtualFileSystem, identity: &Path) {
    vfs.touch(identity);
}
