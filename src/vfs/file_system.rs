use std::env::current_exe;

use std::path::{ Path, PathBuf };
use std::fs::{ ReadDir };
use std::collections::{ HashSet };

use crate::delta::VirtualDelta;
use crate::path::VirtualPath;

use std::hash::{Hash, Hasher};

#[derive(Debug, Eq)]
pub struct LsResult {
    pub path: VirtualPath,
    pub is_directory: bool
}

impl PartialEq for LsResult {
    fn eq(&self, other: &LsResult) -> bool {
        self.path.eq(&other.path)
    }
}

impl LsResult {
    fn from(path: &VirtualPath, is_directory: bool) -> LsResult {
        LsResult{
            path: path.clone(),
            is_directory
        }
    }
}

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

    pub fn convert_to_real_path(&self, path: &Path) -> Option<PathBuf> {
        let state = self.get_state();
        match state.get(&VirtualPath::from_path_buf(path.to_path_buf())) {
            Some(vpath) => Some(vpath.as_source_path().to_path_buf()),
            None => None
        }
    }

    //TODO -> Result ?
    pub fn read_virtual(&mut self, path: &Path) {
        if let Some(real_path) = self.convert_to_real_path(path) {
            if path != real_path {
//                println!("CONVERT {:?} TO {:?}", path, real_path);
                self.read(real_path.as_path(), Some(path));
            }
        }

        self.read(path, None);
    }

    pub fn read(&mut self, path: &Path, virtual_parent: Option<&Path>){
        if path.is_dir() {
            path.read_dir().and_then(|results: ReadDir| {
                for result in results {
                    match result {
                        Ok(result) => {
                            self.real.attach(match virtual_parent {
                                Some(parent) => {
                                    let vpath = VirtualPath::from_path_buf(result.path());
//                                    println!("VIRTUALLY ADD FROM FS : PARENT {:?}, NEW PATH: {:?}", parent, vpath.with_new_parent(parent));
                                    self.add.attach(vpath.with_new_parent(parent), result.path().is_dir());
                                    vpath
                                },
                                None => VirtualPath::from_path_buf(result.path())
                            }, result.path().is_dir());
                        },
                        Err(error) => { println!("{:?}", error); }
                    };
                }
                Ok(())
            }).unwrap();
        }
    }

    pub fn get_state(&self) -> VirtualDelta {
        &(&self.real - &self.sub) + &self.add
    }


    //TODO -> Result
    pub fn rm(&mut self, path: &VirtualPath) {
        self.read_virtual(path.as_path());
        let state = self.get_state();
        if state.exists(&path) {
            let mut sub_delta = VirtualDelta::new();
            for child in state.walk(path) {
                sub_delta.attach(child.clone(), state.is_directory(&child));
            }
            sub_delta.attach(path.clone(), state.is_directory(&path));
            self.sub = &self.sub + &sub_delta;
            self.add = &self.add - &sub_delta;
        } else {
            println!("No such file or directory");
        }
    }

    //TODO -> Result
    pub fn copy(&mut self, source: VirtualPath, destination: VirtualPath) {
        self.read_virtual(source.as_path());
        self.read_virtual(destination.as_path());
        self.read_virtual(destination.parent().unwrap().as_path());

        let state = self.get_state();
        if !state.exists(&source) {
            panic!("Source {:?} does not exists", source);
        } else if !state.is_directory(&destination) {
            panic!("Destination {:?} isnt a directory", destination);
        } else {
            let mut add_delta = VirtualDelta::new();

            let owned_destination = state.get(&destination).unwrap();
            let owned_source = state.get(&source).unwrap();

            let src = owned_source.as_source_path();
            let dst = owned_destination.as_path().join(source.file_name());

            if state.exists(&VirtualPath::from_path_buf(dst.to_path_buf())) {
                println!("Destination file virtually exists : {:?}", dst);
            } else {
                add_delta.attach(
                    VirtualPath::from(dst.to_path_buf(), Some(src.to_path_buf())),
                    state.is_directory(&source)
                );

                for child in self.get_state().walk(&source) {
                    let src = child.as_source_path();
                    let dst = child.with_new_parent(dst.as_path()).as_path().to_path_buf();
                    add_delta.attach(
                        VirtualPath::from(dst.to_path_buf(), Some(src.to_path_buf())),
                        self.get_state().is_directory(&VirtualPath::from_path_buf(src.to_path_buf()))
                    );
                }

                self.add = &self.add + &add_delta;
                self.sub = &self.sub - &add_delta;
            }
        }
    }

    pub fn ls(&mut self, path: VirtualPath) -> Vec<LsResult>{
        self.read_virtual(path.as_path());
        let state = self.get_state();
        let mut result_set : Vec<LsResult> = Vec::new();
        if state.is_directory(&path) {
            if let Some(children) = state.children(&path) {
                for child in children {
                    result_set.push(LsResult::from(&child, state.is_directory(&child)));
                }
            }
        } else {
            self.read_virtual(path.parent().unwrap().as_path());
            if let Some(child) = self.get_state().get(&path) {
                result_set.push(LsResult::from(&child, state.is_directory(&child)));
            }
        }
        result_set
    }
}

#[cfg(test)]
mod virtual_file_system_tests {
    use super::*;

    #[test]
    fn virtual_file_system_test_assets_ok(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
        vfs.read_virtual(sample_path.as_path());
        vfs.read_virtual(sample_path.join(&Path::new("A")).as_path());
        vfs.read_virtual(sample_path.join(&Path::new("B/D")).as_path());
        vfs.read_virtual(sample_path.join(&Path::new("B/D/E")).as_path());
        vfs.read_virtual(sample_path.join(&Path::new("B/D/G")).as_path());
        let state = vfs.get_state();
        assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/C")))));
        assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E")))));
        assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/G")))));
        assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("F")))));
        assert!(state.is_directory(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A")))));
    }

    #[test]
    fn virtual_file_system_rm(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
        vfs.read_virtual(sample_path.as_path());
        vfs.read_virtual(sample_path.join(&Path::new("A")).as_path());
        vfs.read_virtual(sample_path.join(&Path::new("B")).as_path());
        vfs.read_virtual(sample_path.join(&Path::new("B/D")).as_path());
        vfs.read_virtual(sample_path.join(&Path::new("B/D/E")).as_path());
        vfs.read_virtual(sample_path.join(&Path::new("B/D/G")).as_path());
        vfs.rm(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B"))));
        let state = vfs.get_state();
        assert!(!state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B")))));
        assert!(!state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E")))));
        assert!(!state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/G")))));
        assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A")))));
    }

    #[test]
    fn virtual_file_system_copy(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
        vfs.read_virtual(sample_path.as_path());

        vfs.copy(
            VirtualPath::from_path_buf(sample_path.join(&Path::new("B"))),
            VirtualPath::from_path_buf(sample_path.join(&Path::new("A")))
        );

        let results = vfs.ls(VirtualPath::from_path_buf(sample_path.join(&Path::new("A/B/D"))));
        assert!(!results.is_empty());
        assert!(results.contains(&LsResult::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/B/D/E"))), true)));
        assert!(results.contains(&LsResult::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/B/D/G"))), true)));
    }

    #[test]
    fn virtual_file_system_copy_preserve_source_and_node_kind(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
        vfs.read_virtual(sample_path.as_path());

        let real_source = VirtualPath::from_path_buf(sample_path.join(&Path::new("F")));

        vfs.copy(
            real_source.clone(),
            VirtualPath::from_path_buf(sample_path.join(&Path::new("A")))
        );
        vfs.copy(
            VirtualPath::from_path_buf(sample_path.join(&Path::new("A/F"))),
            VirtualPath::from_path_buf(sample_path.join(&Path::new("B")))
        );
        vfs.copy(
            VirtualPath::from_path_buf(sample_path.join(&Path::new("B/F"))),
            VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E")))
        );

        let state = vfs.get_state();

        let results = vfs.ls(VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E"))));

        assert!(!results.is_empty());
        assert!(results.contains(&LsResult::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/F"))), false)));
    }
}
