use crate::VirtualPath;
use crate::VirtualFileSystem;
use std::path::Path;

#[derive(Debug, Eq)]
pub struct LsItem {
    pub path: VirtualPath,
    pub is_directory: bool
}

impl PartialEq for LsItem {
    fn eq(&self, other: &LsItem) -> bool {
        self.path.eq(&other.path) && self.is_directory.eq(&other.is_directory)
    }
}

impl LsItem {
    pub fn from(path: &VirtualPath, is_directory: bool) -> LsItem {
        LsItem{
            path: path.clone(),
            is_directory
        }
    }
}

pub fn ls(vfs: &mut VirtualFileSystem, identity: &Path) -> Option<Vec<LsItem>>{
    vfs.read_virtual(identity);

    let state = vfs.get_state();
    let mut result_set : Vec<LsItem> = Vec::new();

    if state.is_directory(identity) {
        if let Some(children) = state.children(identity) {
            for child in children.iter() {
                result_set.push(LsItem::from(&child, state.is_directory(child.as_identity())));
            }
        }
    } else if let Some(parent) = identity.parent() {
        vfs.read_virtual(parent);
        if let Some(child) = state.get(identity) {
            result_set.push(LsItem::from(&child, state.is_directory(child.as_identity())));
        }
    }

    if result_set.is_empty() {
        None
    } else {
        Some(result_set)
    }
}
