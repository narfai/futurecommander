use crate::VirtualFileSystem;
use std::path::Path;
use crate::operation::rm::rm;
use crate::operation::cp::cp;

pub fn mv(vfs: &mut VirtualFileSystem, source_identity: &Path, destination_identity: &Path) {
    cp(vfs, source_identity, destination_identity);
    rm(vfs, source_identity);
}
