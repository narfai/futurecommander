use crate::VirtualDelta;
use crate::VirtualFileSystem;
use std::path::Path;

//TODO -> Result
pub fn cp(vfs: &mut VirtualFileSystem, source_identity: &Path, destination_identity: &Path) {
   vfs.copy(source_identity, destination_identity);
}
