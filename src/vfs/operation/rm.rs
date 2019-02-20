use crate::VirtualPath;
use crate::VirtualDelta;
use crate::VirtualFileSystem;
use std::path::Path;

pub fn rm(vfs: &mut VirtualFileSystem, identity: &Path) {
    vfs.read_virtual(identity);
    vfs.read_virtual(identity.parent().unwrap());
    let state = vfs.get_state();
    match state.get(identity) {
        Some(virtual_existing) => {
            let mut sub_delta = VirtualDelta::new();
            for virtual_child in state.walk(identity) {
                sub_delta.attach_virtual(
                    virtual_child,
                    state.is_directory(virtual_child.as_identity())
                );
            }
            sub_delta.attach_virtual(
                virtual_existing,
                state.is_directory(virtual_existing.as_identity())
            );
            vfs.sub = &vfs.sub + &sub_delta;
            vfs.add = &vfs.add - &sub_delta;
        },
        None => {
            println!("No such file or directory");
        }
    }
}

