use std::path::{ Path };
use crate::{ VirtualDelta, VirtualChildren, VirtualPath, VirtualKind, VfsError };

#[derive(Debug)]
pub struct VirtualFileSystem {
    pub add: VirtualDelta,
    pub sub: VirtualDelta
}

impl VirtualFileSystem {
    pub fn new() -> VirtualFileSystem {
        VirtualFileSystem {
            add: VirtualDelta::new(),
            sub: VirtualDelta::new()
        }
    }

    pub fn read_dir(&self, path: &Path) -> Result<VirtualChildren, VfsError> {
        let virtual_state = self.get_virtual_state();
        match self.exists_virtually(path) {
            true => match virtual_state.is_directory(path) {
                Some(true) => match virtual_state.get(path) {
                    Some(virtual_identity) =>
                        match virtual_identity.as_source() {
                            Some(source_path) =>
                                match VirtualChildren::from_file_system(
                                    source_path,
                                    Some(virtual_identity.as_identity())
                                ) {
                                    Ok(virtual_children) => Ok(
                                        &(&virtual_children - &self.sub.children(path).unwrap())
                                        + &self.add.children(path).unwrap()
                                    ),
                                    Err(error) => Err(VfsError::from(error))
                                },
                            None => Err(VfsError::HasNoSource(path.to_path_buf()))
                        },
                    None => Err(VfsError::VirtuallyDoesNotExists(path.to_path_buf()))
                },
                Some(false) => Err(VfsError::IsNotADirectory(path.to_path_buf())),
                None => Err(VfsError::VirtuallyDoesNotExists(path.to_path_buf()))
            },
            false => {
                let mut real_children = match VirtualChildren::from_file_system(path, None) {
                    Ok(virtual_children) => virtual_children,
                    Err(error) => return Err(VfsError::from(error))
                };

                if let Some(to_del_children) = self.sub.children(path) {
                    println!("TO DEL CHILDREN {:?}", to_del_children);
                    real_children = &real_children - &to_del_children;
                }

                if let Some(to_add_children) = self.add.children(path) {
                    println!("TO ADD CHILDREN {:?}", to_add_children);
                    real_children = &real_children + &to_add_children;
                }

                Ok(real_children)
            }
        }
    }

    pub fn copy(&mut self, source: &Path, destination: &Path) -> Result<VirtualPath, VfsError>{
        let virtual_state = self.get_virtual_state();
        let referent_source = match virtual_state.get(source) {
            Some(source_identity) => source_identity.as_referent_source(),
            None => match source.exists() {
                true => source,
                false => return Err(VfsError::DoesNotExists(source.to_path_buf()))
            }
        };

        if !self.exists(source) {
            return Err(VfsError::DoesNotExists(source.to_path_buf()))
        }

        let kind = match self.is_directory_virtually(destination) {
            Some(true) => VirtualKind::Directory,
            Some(false) => return Err(VfsError::IsNotADirectory(destination.to_path_buf())),
            None => match source.exists() {
                true => match source.is_dir() {
                    true => VirtualKind::Directory,
                    false => return Err(VfsError::IsNotADirectory(destination.to_path_buf()))
                }
                false => return Err(VfsError::DoesNotExists(destination.to_path_buf()))
            }
        };

        let new_identity = &VirtualPath::from_path(source)
            .with_new_parent(destination)
            .with_source(Some(referent_source))
            .with_kind(kind);

        if self.exists(new_identity.as_identity()) {
           return Err(VfsError::AlreadyExists(new_identity.to_identity()))
        }

        self.add.attach_virtual(new_identity);

        if self.sub.exists(new_identity.as_identity()) {
            self.sub.detach(new_identity.as_identity())
        }

        Ok(new_identity.clone())
    }

    pub fn remove(&mut self, path: &Path) -> Result<VirtualPath, VfsError> {
        let identity = match self.add.get(path) {
            Some(identity) => {
                let cloned = identity.clone();
                self.add.detach(cloned.as_identity());
                cloned
            },
            None => match path.exists() {
                true => VirtualPath::from_path(path).with_kind(match path.is_dir() {
                        true => VirtualKind::Directory,
                        false => VirtualKind::File
                    }),
                false => return Err(VfsError::DoesNotExists(path.to_path_buf()))
            }
        };

        return match self.sub.get(path) {
            Some(_) => Err(VfsError::DoesNotExists(path.to_path_buf())),
            None => {
                self.sub.attach_virtual(&identity);
                Ok(identity.clone())
            }
        }
    }

    pub fn mkdir(&mut self, path: &Path) -> Result<(), VfsError>{
        match self.exists(path) {
            true => Err(VfsError::AlreadyExists(path.to_path_buf())),
            false => {
                self.add.attach(path, None, true);
                Ok(())
            }
        }
    }

    pub fn touch(&mut self, path: &Path) -> Result<(), VfsError>{
        match self.exists(path) {
            true => Err(VfsError::AlreadyExists(path.to_path_buf())),
            false => {
                self.add.attach(path, None, false);
                Ok(())
            }
        }
    }

    pub fn mv(&mut self, source: &Path, destination: &Path) -> Result<VirtualPath, VfsError>{
        let result = self.copy(source, destination);
        self.remove(source);
        result
    }

    pub fn exists(&self, path: &Path) -> bool {
        self.exists_virtually(path) || path.exists()
    }

    pub fn exists_virtually(&self, path: &Path) -> bool {
        self.get_virtual_state().exists(path)
    }

    pub fn is_directory_virtually(&self, path: &Path) -> Option<bool> {
        self.get_virtual_state().is_directory(path)
    }

    pub fn get_add_state(&self) -> VirtualDelta {
        self.add.clone()
    }

    pub fn get_sub_state(&self) -> VirtualDelta {
        self.sub.clone()
    }

    pub fn get_virtual_state(&self) -> VirtualDelta {
        &self.add - &self.sub
    }
}
