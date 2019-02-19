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
        self.path.eq(&other.path) && self.is_directory.eq(&other.is_directory)
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

    pub fn convert_to_real_path(&self, identity: &Path) -> Option<PathBuf> {
        match self.get_state().get(identity) {
            Some(virtual_identity) => Some(virtual_identity.to_referent_source()),
            None => None
        }
    }

    //TODO -> Result ?
    pub fn read_virtual(&mut self, identity: &Path) {
        //To reduce fs read and keep self.real with a consistent state
        //if self.is_virtual(path) {
        if let Some(real_identity) = self.convert_to_real_path(identity) {
            if identity != real_identity {
//                println!("CONVERT {:?} TO {:?}", path, real_path);
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
        println!("FS READ");
        identity.read_dir().and_then(|results: ReadDir| {
            for result in results {
                match result {
                    Ok(result) => {
                        let virtual_path = match virtual_parent {
                            Some(parent) => {
                                let virtual_path = VirtualPath::from_path_buf(result.path())
                                    .with_new_parent(parent);
//                                    println!("VIRTUALLY ADD FROM FS : PARENT {:?}, NEW PATH: {:?}", parent, vpath.with_new_parent(parent));
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

    pub fn ls(&mut self, identity: &Path) -> Option<Vec<LsResult>>{
        self.read_virtual(identity);

        let state = self.get_state();
        let mut result_set : Vec<LsResult> = Vec::new();

        if state.is_directory(identity) {
            if let Some(children) = state.children(identity) {
                for child in children {
                    result_set.push(LsResult::from(&child, state.is_directory(child.as_identity())));
                }
            }
        } else if let Some(parent) = identity.parent() {
            self.read_virtual(parent);
            if let Some(child) = state.get(identity) {
                result_set.push(LsResult::from(&child, state.is_directory(child.as_identity())));
            }
        }
        if result_set.is_empty() {
            None
        } else {
            Some(result_set)
        }
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

        match vfs.ls(sample_path.join(&Path::new("A/B/D")).as_path()) {
            Some(results) => {
                assert!(!results.is_empty());
                assert!(results.contains(&LsResult::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/B/D/E"))), true)));
                assert!(results.contains(&LsResult::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/B/D/G"))), true)));
            },
            None => { panic!("No results") }
        }

    }

    #[test]
    fn virtual_file_system_copy_preserve_source_and_node_kind(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
        vfs.read_virtual(sample_path.as_path());

        let real_source = VirtualPath::from_path_buf(sample_path.join(&Path::new("F")));

        vfs.copy(
            real_source.as_identity(),
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

        match vfs.ls(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Some(results) => {
                assert!(!results.is_empty());
                assert!(results.contains(&LsResult::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/F"))), false)));
            },
            None => { panic!("No results"); }
        }
    }

    #[test]
    fn virtual_file_system_mv(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
        vfs.read_virtual(sample_path.as_path());

        let real_source = VirtualPath::from_path_buf(sample_path.join(&Path::new("F")));

        vfs.mv(
            real_source.as_identity(),
            sample_path.join(&Path::new("A")).as_path()
        );
        vfs.mv(
            sample_path.join(&Path::new("A/F")).as_path(),
            sample_path.join(&Path::new("B")).as_path()
        );
        vfs.mv(
            sample_path.join(&Path::new("B/F")).as_path(),
            sample_path.join(&Path::new("B/D/E")).as_path()
        );

        match vfs.ls(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Some(results) => {
                assert!(!results.is_empty());
                assert!(results.contains(&LsResult::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/F"))), false)));
            },
            None => { panic!("No results"); }
        }

        assert!(vfs.ls(sample_path.join(&Path::new("F")).as_path()).is_none());
        assert!(vfs.ls(sample_path.join(&Path::new("A/F")).as_path()).is_none());
        assert!(vfs.ls(sample_path.join(&Path::new("B/F")).as_path()).is_none());
    }

    #[test]
    fn virtual_file_system_mkdir(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
        vfs.read_virtual(sample_path.as_path());

        vfs.mkdir(sample_path.join(&Path::new("B/D/E/MKDIRED")).as_path());
        match vfs.ls(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Some(results) => {
                println!("{:?}", results);
                assert!(!results.is_empty());
                assert!(results.contains(&LsResult::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/MKDIRED"))), true)));
            },
            None => { panic!("No results"); }
        }
    }

    #[test]
    fn virtual_file_system_touch(){
        let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
        let mut vfs = VirtualFileSystem::new();
        vfs.read_virtual(sample_path.as_path());

        vfs.touch(sample_path.join(&Path::new("B/D/E/TOUCHED")).as_path());
        match vfs.ls(sample_path.join(&Path::new("B/D/E")).as_path()) {
            Some(results) => {
                assert!(!results.is_empty());
                assert!(results.contains(&LsResult::from(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E/TOUCHED"))), false)));
            },
            None => { panic!("No results"); }
        }
    }
}
