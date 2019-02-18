use std::env::current_exe;

use std::path::{ Path };
use std::fs::{ ReadDir };

use crate::delta::VirtualDelta;
use crate::path::VirtualPath;

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

    //TODO -> Result ?
    pub fn read(&mut self, path: &Path) {
        let state = self.get_state();
        let virtual_path = VirtualPath::from_path_buf(path.to_path_buf()).to_referent_source(&state);

        if virtual_path.exists() && virtual_path.is_dir() {
            virtual_path.read_dir()
                .and_then(|results: ReadDir| {
                    for result in results {
                        let result = result?;
                        self.real.attach(VirtualPath::from_path_buf(result.path()), result.path().is_dir());
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
        self.read(path.as_path());
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
        self.read(source.as_path());
        self.read(destination.as_path());
        let state = self.get_state();
        if !state.exists(&source) {
            println!("Source {:?} does not exists", source);
        } else if !state.is_directory(&destination) {
            println!("Destination {:?} isnt a directory", destination);
        } else {
            if let Some(destination) = state.get(&destination) {
                println!("SOURCE {:?}", source);
                let owned_source = state.get(&source).unwrap();
                let mut add_delta = VirtualDelta::new();
                let src = owned_source.to_referent_source(&state);
                let dst = destination.as_path().join(source.file_name());

                println!("PARENT REF SOURCE : {:?}", src);

                if state.exists(&VirtualPath::from_path_buf(dst.to_path_buf())) {
                    println!("Destination file already exists : {:?}", dst);
                } else {
                    add_delta.attach(
                        VirtualPath::from(dst.to_path_buf(), Some(src)),
                        state.is_directory(&source)
                    );
                    for child in self.get_state().walk(&source) {
                        let src = child.to_referent_source(&state);
                        println!("CHILD REF SOURCE : {:?}", src);
                        let dst = child.with_new_parent(dst.as_path()).as_path().to_path_buf();
                        add_delta.attach(
                            VirtualPath::from(dst.to_path_buf(), Some(src.to_path_buf())),
                            self.get_state().is_directory(&VirtualPath::from_path_buf(src))
                        );
                    }
                    self.add = &self.add + &add_delta;
                    self.sub = &self.sub - &add_delta;
                }
            }
        }
    }
}

#[test]
fn vfs_test_assets_ok(){
    let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
    let mut vfs = VirtualFileSystem::new();
    vfs.read(sample_path.as_path());
    vfs.read(sample_path.join(&Path::new("A")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D/E")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D/G")).as_path());
    let state = vfs.get_state();
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/C")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/G")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("F")))));
    assert!(state.is_directory(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A")))));
}

#[test]
fn vfs_rm(){
    let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
    let mut vfs = VirtualFileSystem::new();
    vfs.read(sample_path.as_path());
    vfs.read(sample_path.join(&Path::new("A")).as_path());
    vfs.read(sample_path.join(&Path::new("B")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D/E")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D/G")).as_path());
    vfs.rm(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B"))));
    let state = vfs.get_state();
    assert!(!state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B")))));
    assert!(!state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E")))));
    assert!(!state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/G")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A")))));
}

#[test]
fn vfs_copy(){
    let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
    let mut vfs = VirtualFileSystem::new();
    vfs.read(sample_path.as_path());
    vfs.read(sample_path.join(&Path::new("A")).as_path());
    vfs.read(sample_path.join(&Path::new("B")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D/E")).as_path());
    vfs.read(sample_path.join(&Path::new("B/D/G")).as_path());

    vfs.copy(
        VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D"))),
        VirtualPath::from_path_buf(sample_path.join(&Path::new("A")))
    );

    let state = vfs.get_state();
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/D")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/D/E")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("A/D/G")))));

    vfs.copy(
        VirtualPath::from_path_buf(sample_path.join(&Path::new("A/D"))),
        VirtualPath::from_path_buf(sample_path.join(&Path::new("B")))
    );

    let state = vfs.get_state();
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/E")))));
    assert!(state.exists(&VirtualPath::from_path_buf(sample_path.join(&Path::new("B/D/G")))));
}
