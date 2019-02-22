use std::path::{ Path, PathBuf };
use std::fs::{ ReadDir };
use crate::delta::VirtualDelta;
use crate::path::{ VirtualPath, VirtualKind };

#[derive(Debug)]
pub enum VirtualNodeState {
    Real,
    Added,
    Removed,
    SubDangling,
    AddSubDangling,
    Override,
    Unknown
}

#[derive(Debug)]
pub struct VirtualFileSystem {
    pub real: VirtualDelta,
    pub add: VirtualDelta,
    pub sub: VirtualDelta
}

impl VirtualFileSystem {
    pub fn new() -> VirtualFileSystem {
        VirtualFileSystem {
            real: VirtualDelta::new(),
            add: VirtualDelta::new(),
            sub: VirtualDelta::new()
        }
    }

    pub fn convert_to_real_path(&self, identity: &Path) -> Option<PathBuf> {
        match self.get_state().get(identity) {
            Some(virtual_identity) => Some(virtual_identity.to_referent_source()),
            None => None
        }
    }

    //TODO -> Result ?
    //Find a way for async
    pub fn read_virtual(&mut self, identity: &Path) {
        if let Some(real_identity) = self.convert_to_real_path(identity) {
            if identity != real_identity {
                if real_identity.as_path().is_dir() {
                    self.read(real_identity.as_path(), Some(identity));
                }
            }
        }

        if identity.is_dir() {
            self.read(identity, None);
        }
    }

    pub fn read(&mut self, identity: &Path, virtual_parent: Option<&Path>){
//        println!("FS READ");
        identity.read_dir().and_then(|results: ReadDir| {
            for result in results {
                match result {
                    Ok(result) => {
                        let virtual_path = match virtual_parent {
                            Some(parent) => {                                ;
                                let virtual_path = VirtualPath::from_path_buf(result.path())
                                    .with_new_parent(parent)
                                    .with_kind(match result.path().is_dir() {
                                        true => VirtualKind::Directory,
                                        false => VirtualKind::File
                                    });

                                self.add.exp_attach_virtual( &virtual_path );
                                virtual_path
                            },
                            None => VirtualPath::from_path_buf(result.path())
                        };
                        self.real.exp_attach_virtual(&virtual_path);
                    },
                    Err(error) => { println!("{:?}", error); }
                };
            }
            Ok(())
        }).unwrap();
    }

    pub fn virtualize(&mut self, identity: &Path) -> VirtualDelta {
        match self.get_node_state(identity) {
            VirtualNodeState::Added => {
                let mut matching_identity = match identity.is_file() {
                    true => VirtualPath::get_parent_or_root(identity),
                    false => identity.to_path_buf()
                };

                let state = self.get_state();
                if state.is_directory_empty(matching_identity.as_path()) && !self.real.is_directory_empty(matching_identity.as_path()) {
                    match state.get(matching_identity.as_path()) {
                        Some(virtual_identity) => self.exp_read_dir(virtual_identity.as_referent_source(), Some(virtual_identity.as_identity())),
                        None => {}
                    }
                }
            },
            VirtualNodeState::Removed => {},
            VirtualNodeState::Real => {},
            VirtualNodeState::Unknown => { self.exp_read_dir(identity, None); },
            VirtualNodeState::SubDangling => { self.sub.detach(identity); println!("VIRTUALIZE Detached dangling {:?}", identity); },
            VirtualNodeState::AddSubDangling => {
                self.sub.detach(identity);
                self.add.detach(identity);
                println!("VIRTUALIZE Detached dangling {:?}", identity);
            },
            VirtualNodeState::Override => panic!("VIRTUALIZE OVERRIDE {:?}", identity)
        };
        self.get_state()
    }

    pub fn exp_read_dir(&mut self, identity: &Path, virtual_parent: Option<&Path>)  {
        identity.read_dir().and_then(|results: ReadDir| {
            for result in results {
                match result {
                    Ok(result) => {
                        let virtual_path = match virtual_parent {
                            Some(parent) => {                                ;
                                let virtual_path = VirtualPath::from_path_buf(result.path())
                                    .with_new_parent(parent)
                                    .with_kind(match result.path().is_dir() {
                                        true => VirtualKind::Directory,
                                        false => VirtualKind::File
                                    });

                                self.add.exp_attach_virtual( &virtual_path );
                                virtual_path
                            },
                            None => VirtualPath::from_path_buf(result.path())
                        };
                        self.real.exp_attach_virtual(&virtual_path);
                    },
                    Err(error) => { println!("{:?}", error); }
                };
            }
            Ok(())
        }).unwrap();
    }

    //pub fn exp_add()
    //pub fn exp_remove()
    //pub fn exp_rename() ?

    pub fn get_node_state(&self, identity: &Path) -> VirtualNodeState {
        match self.add.exists(identity) {
            true => match self.sub.exists(identity) {
                true => VirtualNodeState::AddSubDangling,
                false => match self.real.exists(identity) {
                    true => VirtualNodeState::Override,
                    false => VirtualNodeState::Added
                },
            },
            false => match self.sub.exists(identity) {
                true => match self.real.exists(identity) {
                    true => VirtualNodeState::Removed,
                    false => VirtualNodeState::SubDangling
                },
                false => match self.real.exists(identity) {
                    true => VirtualNodeState::Real,
                    false => VirtualNodeState::Unknown
                },
            },
        }
    }

    pub fn get_state(&self) -> VirtualDelta {
        &(&self.real - &self.sub) + &self.add
    }

    pub fn get_add_state(&self) -> VirtualDelta {
        self.add.clone()
    }

    pub fn get_sub_state(&self) -> VirtualDelta {
        self.sub.clone()
    }

    pub fn get_real_state(&self) -> VirtualDelta {
        self.real.clone()
    }



    pub fn children(identity: &Path) {

    }
}
