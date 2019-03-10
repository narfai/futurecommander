use crate::VirtualFileSystem;
use std::path::Path;

pub fn mv(vfs: &mut VirtualFileSystem, source_identity: &Path, destination_identity: &Path) {
    vfs.mv(source_identity, destination_identity);
}
