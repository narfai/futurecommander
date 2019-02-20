use crate::VirtualDelta;
use crate::VirtualFileSystem;
use std::path::Path;

//TODO -> Result
pub fn cp(vfs: &mut VirtualFileSystem, source_identity: &Path, destination_identity: &Path) {
    vfs.read_virtual(source_identity);
    vfs.read_virtual(destination_identity);
    if let Some(parent) = destination_identity.parent() {
        vfs.read_virtual(parent);
    }

    let state = vfs.get_state();
    match state.get(source_identity) {
        Some(virtual_source) => {
            match state.get(destination_identity) {
                Some(virtual_destination) => {
                    if !state.is_directory(virtual_destination.as_identity()) {
                        panic!("Destination {:?} isnt a directory", virtual_destination);
                    }
                    let mut add_delta = VirtualDelta::new();
                    let virtual_new = virtual_destination
                        .join(virtual_source.file_name())
                        .with_source(Some(virtual_source.as_referent_source()));

                    add_delta.attach_virtual(
                        &virtual_new,
                        state.is_directory(virtual_source.as_identity())
                    );

                    for virtual_child in state.walk(virtual_source.as_identity()) {
                        let virtual_new_child = virtual_child.clone()
                            .with_new_parent(virtual_new.as_identity())
                            .with_source(Some(virtual_child.as_referent_source()));

                        add_delta.attach_virtual(
                            &virtual_new_child,
                            state.is_directory(virtual_child.as_identity())
                        );
                    }

                    vfs.add = &vfs.add + &add_delta;
                    vfs.sub = &vfs.sub - &add_delta;
                },
                None => { panic!("Destination {:?} does not exists", destination_identity); }
            }
        },
        None => { panic!("Source {:?} does not exists", source_identity); }
    }
}
