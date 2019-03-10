use crate::VirtualFileSystem;
use std::path::Path;

pub fn rm(vfs: &mut VirtualFileSystem, identity: &Path) {
    vfs.remove(identity);
}

