use std::env::current_exe;

use std::path::{ Path, PathBuf };
use std::fs::{ ReadDir };

use crate::delta::VirtualDelta;
use crate::path::VirtualPath;

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
    //pub fn is_virtual(&self, path: &Path) -> bool

    pub fn convert_to_real_path(&self, path: &Path) -> Option<PathBuf> {
        let state = self.get_state();
        match state.get(path) {
            Some(vpath) => Some(vpath.as_source_path().to_path_buf()),
            None => None
        }
    }

    //TODO -> Result ?
    pub fn read_virtual(&mut self, path: &Path) {
        //To reduce fs read and keep self.real with a consistent state
        //if self.is_virtual(path) {
        if let Some(real_path) = self.convert_to_real_path(path) {
            if path != real_path {
//                println!("CONVERT {:?} TO {:?}", path, real_path);
                if real_path.as_path().is_dir() {
                    self.read(real_path.as_path(), Some(path));
                }
            }
        }
        if path.is_dir() {
            self.read(path, None);
        }
    }

    pub fn read(&mut self, path: &Path, virtual_parent: Option<&Path>){
        println!("FS READ");
        path.read_dir().and_then(|results: ReadDir| {
            for result in results {
                match result {
                    Ok(result) => {
                        let virtual_path = match virtual_parent {
                            Some(parent) => {
                                let vpath = VirtualPath::from_path_buf(result.path());
//                                    println!("VIRTUALLY ADD FROM FS : PARENT {:?}, NEW PATH: {:?}", parent, vpath.with_new_parent(parent));
                                self.add.attach(vpath.with_new_parent(parent).get_path(), vpath.get_source(), result.path().is_dir());
                                vpath
                            },
                            None => VirtualPath::from_path_buf(result.path())
                        };
                        self.real.attach(virtual_path.get_path(), virtual_path.get_source(), result.path().is_dir());
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
    pub fn rm(&mut self, path: &Path) {
        self.read_virtual(path);
        self.read_virtual(path.parent().unwrap());
        let state = self.get_state();
        if state.exists(path) {
            let mut sub_delta = VirtualDelta::new();
            for child in state.walk(path) {
                sub_delta.attach(child.to_path_buf(), state.get(child).unwrap().get_source(), state.is_directory(&child));
            }
            sub_delta.attach(path.to_path_buf(), state.get(path).unwrap().get_source(), state.is_directory(&path));
            self.sub = &self.sub + &sub_delta;
            self.add = &self.add - &sub_delta;
        } else {
            println!("No such file or directory");
        }
    }

    //TODO -> Result
    pub fn copy(&mut self, source: &Path, destination: &Path) {
        self.read_virtual(source);
        self.read_virtual(destination);
        if let Some(parent) = destination.parent() {
            self.read_virtual(parent);
        }

        let state = self.get_state();
        if !state.exists(&source) {
            panic!("Source {:?} does not exists", source);
        } else if !state.is_directory(destination) {
            panic!("Destination {:?} isnt a directory", destination);
        } else {
            let mut add_delta = VirtualDelta::new();

            let owned_destination = state.get(destination).unwrap().as_path();
            let owned_source = state.get(&source).unwrap();

            let src_parent = owned_source.as_source_path();
            let dst_parent = owned_destination.join(source.file_name().unwrap());

            if state.exists(dst_parent.as_path()) {
                println!("Destination file virtually exists : {:?}", dst_parent);
            } else {
                add_delta.attach(
                    dst_parent.to_path_buf(),
                    Some(src_parent.to_path_buf()),
                    state.is_directory(&source)
                );

                for child_path in state.walk(&source) {
                    let src = state.get(&child_path).unwrap().as_source_path();
                    let dst = VirtualPath::from_path(child_path).with_new_parent(dst_parent.as_path()).as_path().to_path_buf();
                    add_delta.attach(
                        dst,
                        Some(src.to_path_buf()),
                        state.is_directory(src)
                    );
                }

                self.add = &self.add + &add_delta;
                self.sub = &self.sub - &add_delta;
            }
        }
    }

    pub fn ls(&mut self, path: &Path) -> Vec<LsResult>{
        self.read_virtual(path);

        let state = self.get_state();
        let mut result_set : Vec<LsResult> = Vec::new();

        if state.is_directory(path) {
            if let Some(children) = state.children(path) {
                for child in children {
                    result_set.push(LsResult::from(&child, state.is_directory(child.as_path())));
                }
            }
        } else {
            self.read_virtual(path.parent().unwrap());
            if let Some(child) = state.get(path) {
                result_set.push(LsResult::from(&child, state.is_directory(child.as_path())));
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
        assert!(state.exists(sample_path.join(&Path::new("A/C")).as_path()));
        assert!(state.exists(sample_path.join(&Path::new("B/D/E")).as_path()));
        assert!(state.exists(sample_path.join(&Path::new("B/D/G")).as_path()));
        assert!(state.exists(sample_path.join(&Path::new("F")).as_path()));
        assert!(state.is_directory(sample_path.join(&Path::new("A")).as_path()));
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

        vfs.rm(sample_path.join(&Path::new("B")).as_path());

        let state = vfs.get_state();
        assert!(!state.exists(sample_path.join(&Path::new("B")).as_path()));
        assert!(!state.exists(sample_path.join(&Path::new("B/D/E")).as_path()));
        assert!(!state.exists(sample_path.join(&Path::new("B/D/G")).as_path()));
        assert!(state.exists(sample_path.join(&Path::new("A")).as_path()));
    }

    #[test]
    fn virtual_file_system_copy(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
        vfs.read_virtual(sample_path.as_path());

        vfs.copy(
            sample_path.join(&Path::new("B")).as_path(),
            sample_path.join(&Path::new("A")).as_path()
        );

        let results = vfs.ls(sample_path.join(&Path::new("A/B/D")).as_path());
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
            real_source.as_path(),
            sample_path.join(&Path::new("A")).as_path()
        );
        vfs.copy(
            sample_path.join(&Path::new("A/F")).as_path(),
            sample_path.join(&Path::new("B")).as_path()
        );
        vfs.copy(
            sample_path.join(&Path::new("B/F")).as_path(),
            sample_path.join(&Path::new("B/D/E")).as_path()
        );

        let results = vfs.ls(sample_path.join(&Path::new("B/D/E")).as_path());

        assert!(!results.is_empty());
        assert!(results.contains(&LsResult::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/F"))), false)));
    }
}
