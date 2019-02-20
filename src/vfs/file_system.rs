use std::path::{ Path, PathBuf };
use std::fs::{ ReadDir };

use crate::delta::VirtualDelta;
use crate::path::VirtualPath;
use std::iter::Iterator;


#[derive(Debug)]
pub struct VirtualFileSystem {
    real: VirtualDelta,
    add: VirtualDelta,
    sub: VirtualDelta
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


    //TODO -> Result
    pub fn rm(&mut self, identity: &Path) {
        self.read_virtual(identity);
        self.read_virtual(identity.parent().unwrap());
        let state = self.get_state();
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
                self.sub = &self.sub + &sub_delta;
                self.add = &self.add - &sub_delta;
            },
            None => {
                println!("No such file or directory");
            }
        }
    }

    //TODO -> Result
    pub fn copy(&mut self, source_identity: &Path, destination_identity: &Path) {
        self.read_virtual(source_identity);
        self.read_virtual(destination_identity);
        if let Some(parent) = destination_identity.parent() {
            self.read_virtual(parent);
        }

        let state = self.get_state();
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

                        self.add = &self.add + &add_delta;
                        self.sub = &self.sub - &add_delta;
                    },
                    None => { panic!("Destination {:?} does not exists", destination_identity); }
                }
            },
            None => { panic!("Source {:?} does not exists", source_identity); }
        }
    }

    pub fn mv(&mut self, source_identity: &Path, destination_identity: &Path) {
        self.copy(source_identity, destination_identity);
        self.rm(source_identity);
    }

    pub fn mkdir(&mut self, identity: &Path) {
        self.read_virtual(identity);
        match self.get_state().get(identity) {
            Some(virtual_existing) => { println!("Already exists {:?}", virtual_existing); },
            None => {
                self.add.attach(
                    identity,
                    None,
                    true
                );
            }
        }
    }

    pub fn touch(&mut self, identity: &Path) {
        self.read_virtual(identity);
        if !self.get_state().exists(identity) {
            self.add.attach(
                identity,
                None,
                false
            );
        }
    }
    pub fn ls(&mut self, identity: &Path) -> Option<Vec<LsItem>>{
        self.read_virtual(identity);

        let state = self.get_state();
        let mut result_set : Vec<LsItem> = Vec::new();

        if state.is_directory(identity) {
            if let Some(children) = state.children(identity) {
                for child in children {
                    result_set.push(LsItem::from(&child, state.is_directory(child.as_identity())));
                }
            }
        } else if let Some(parent) = identity.parent() {
            self.read_virtual(parent);
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

    pub fn tree(&mut self, identity: &Path) {
        self._tree(identity, None, false, true);
    }

    fn _tree(&mut self, identity: &Path, depth_list: Option<Vec<(bool,bool)>>, parent_first: bool, parent_last: bool) {
        self.read_virtual(identity);

        let file_name = match identity.file_name() {
            Some(file_name) => file_name.to_string_lossy().to_string(),
            None => "/".to_string()
        };


        if let Some(depth_list) = &depth_list {
            let mut depth_delimiter = "".to_string();
            for (first, last) in depth_list {
                if *last {
                    depth_delimiter += "    ";
                } else {
                    depth_delimiter += "│   ";
                }
            }
            println!(
                "{}{}── {}",
                depth_delimiter,
                match parent_last {
                    false => "├",
                    true => "└"
                },
                file_name
            );
        } else {
            println!("{}", file_name);
            println!("│");
        }

        match self.get_state().children(identity){
            Some(children) => {
                let new_depth_list = match depth_list {
                    Some(depth_list) => {
                        let mut new = depth_list.clone();
                        new.push((parent_first, parent_last));
                        new
                    },
                    None => vec![]
                };

                let length = children.len();

                for (index, virtual_child) in children.iter().enumerate() {
                    self._tree(
                        virtual_child.as_identity(),
                        Some(new_depth_list.clone()),
                        index == 0,
                        index == (length - 1)
                    );
                }
            },
            None => {}
        };
    }
}

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

