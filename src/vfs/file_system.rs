use std::path::{ Path, PathBuf };
use std::fs::{ ReadDir };
use crate::delta::VirtualDelta;
use crate::path::VirtualPath;

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
                            Some(parent) => {
                                let virtual_path = VirtualPath::from_path_buf(result.path())
                                    .with_new_parent(parent);
//                                println!("ADD VIRTUAL CHILD {:?} => {:?}", result.path(), virtual_path);
                                self.add.attach_virtual( &virtual_path, result.path().is_dir());
                                virtual_path
                            },
                            None => VirtualPath::from_path_buf(result.path())
                        };
                        self.real.attach_virtual(&virtual_path, result.path().is_dir());
                    },
                    Err(error) => { println!("{:?}", error); }
                };
            }
            Ok(())
        }).unwrap();
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

    pub fn get(identity: &Path) {
        /*
            Exists
                in state
                in add
                in sub
            Is directory
                Is root
                    Is empty
                and is in real fs
                    Is empty
                and is virtually
                    Is empty
            Is file
                in real fs
                virtually

            TODO recursive method to attach directories from bottom to top till it match some existing dir in state
            TODO / IDEA be able to slice a delta into a subtree


            Cache the virtual_state
            VIRTUALIZATION
            1. maintain real tree

            2. maintain virtual tree

        */
    }

    pub fn children(identity: &Path) {

    }
}
