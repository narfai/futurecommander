mod node;

use std::{
    path::{ Path, PathBuf },
    ffi::OsStr
};

use self::super::{
    FileSystemError,
    WriteFileSystem,
    ReadFileSystem
};

use crate::{
    path::normalize,
    Result,
    filesystem::{
        Metadata,
        ReadDir,
        DirEntry,
        FileTypeExt,
        MetadataExt,
        PathExt
    }
};

pub use self::{
    node::{ Node, Kind }
};

pub struct Preview {
    root: node::Node
}

impl ReadFileSystem for Preview {
    /// Errors :
    /// * The user lacks permissions to perform `metadata` call on `path`.
    /// * `path` does not exist.
    fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<Metadata> {
        let path = path.as_ref();
        if let Some(node) = self.root.find_at_path(path)? {
            if node.is_deleted(){
                Err(FileSystemError::Custom(String::from("Path does not exists")))
            } else {
                node.into_virtual_metadata()
            }
        } else if path.exists() {
            path.metadata()?.into_virtual_metadata()
        } else {
            Err(FileSystemError::Custom(String::from("Path does not exists")))
        }
    }

    /// Errors :
    /// * The provided `path` doesn't exist.
    /// * The process lacks permissions to view the contents.
    /// * The `path` points at a non-directory file.
    fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<ReadDir> {
        let path = path.as_ref();
        if let Some(node) = self.root.find_at_path(path)? {
            if node.is_deleted(){
                Err(FileSystemError::Custom(String::from("Path does not exists")))
            } else if let Kind::Directory(children) = node.kind() {
                Ok(ReadDir::new(path, children.iter().map(|node| node.clone()).collect()))
            } else {
                Err(FileSystemError::Custom(String::from("Not a directory")))
            }
        } else if path.exists() {
            if path.is_dir() {
                Ok(ReadDir::new(path, Vec::new()))
            } else {
                Err(FileSystemError::Custom(String::from("Not a directory")))
            }
        } else {
            Err(FileSystemError::Custom(String::from("Path does not exists")))
        }
    }
}

impl Preview {
    fn _create_file(&mut self, path: &Path) -> Result<()> {
        match path.file_name() {
            Some(file_name) => {
                self.root
                    .filter(|parent_path, child| &parent_path.join(child.name()) != path)?
                    .insert_at(path, &Node::new_file(&file_name.to_string_lossy(), None))?;
                Ok(())
            },
            None => Err(FileSystemError::Custom(String::from("Cannot obtain file name")))
        }
    }

    fn _create_dir(&mut self, path: &Path) -> Result<()> {
        match path.file_name() {
            Some(file_name) => {
                self.root
                    .filter(|parent_path, child| &parent_path.join(child.name()) != path)?
                    .insert_at(path, &Node::new_directory(&file_name.to_string_lossy()))?;
                Ok(())
            },
            None => Err(FileSystemError::Custom(String::from("Cannot obtain file name")))
        }
    }

    fn _rename_file(&mut self, from: &Path, to: &Path) -> Result<()> {
        let source = self.root.find_at_path(from)?
            .and_then(|node| node.source())
            .and_then(|src| Some(src.to_path_buf()))
            .or(Some(from.to_path_buf()));

        self.root
            .filter(|parent_path, child| &parent_path.join(child.name()) != from || &parent_path.join(child.name()) != to)?
            .insert_at(
                to.parent().unwrap(),
                &Node::new_deleted(&from.file_name().unwrap().to_string_lossy())
            )?.insert_at(
                to.parent().unwrap(),
                &Node::new_file(&to.file_name().unwrap().to_string_lossy(), source)
            )?;
        Ok(())
    }

    fn _rename(&mut self, from: &Path, to: &Path) -> Result<()> {
        if from.preview_is_a_dir(self) {
            for child_result in from.preview_read_dir(self)? {
                let child = child_result?;
                if child.file_type()?.is_dir() {
                    self._rename(&from.join(child.file_name()), &to.join(child.file_name()))?;
                }
            }
            self._create_dir(to)?;
            self._remove(from)?;
        } else {
            self._rename_file(from, to)?;
        }
        Ok(())
    }

    fn _copy(&mut self, from: &Path, to: &Path) -> Result<u64> {
        self.root
            .filter(|parent_path, child| &parent_path.join(child.name()) != to)?
            .insert_at(
                to.parent().unwrap(),
                &Node::new_file(&to.file_name().unwrap().to_string_lossy(), Some(from.to_path_buf()))
            )?;
        Ok(0)
    }

    //TODO
    fn _are_on_same_filesystem(&self, _left: &Path, _right: &Path) -> bool {
        true
    }

    fn _remove(&mut self, path: &Path) -> Result<()> {
        self.root
            .filter(|parent_path, child| &parent_path.join(child.name()) != path)?
            .insert_at(path, &Node::new_deleted(&path.file_name().unwrap().to_string_lossy()))?;

        Ok(())
    }

    fn _has_to_exist(&self, path: &Path, error: FileSystemError) -> Result<()> {
        if path.preview_exists(self) {
            Ok(())
        } else {
            Err(error)
        }
    }

    fn _has_to_not_exist(&self, path: &Path, error: FileSystemError) -> Result<()> {
        if path.preview_exists(self) {
            Err(error)
        } else {
            Ok(())
        }
    }
}

// TODO map same error behaviors according to real std::fs
// TODO use then / or to prevent if else nesting hell
impl WriteFileSystem for Preview {
    fn create_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            self._has_to_not_exist(path, FileSystemError::Custom(String::from("Path already exists")))?;
            self._has_to_exist(parent, FileSystemError::Custom(String::from("Parent doesn't exists")))?;

            if parent.preview_is_a_dir(self) {
                self._create_file(path)
            } else {
                Err(FileSystemError::Custom(String::from("Parent is not a directory")))
            }
        } else {
            Err(FileSystemError::Custom(String::from(format!("Invalid path given {}", path.display()))))
        }
    }

    /**
     * Errors :
     * - User lacks permissions to create directory at `path`.
     * - A parent of the given path doesn't exist.
     * - `path` already exists.
     */
    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            self._has_to_not_exist(path, FileSystemError::Custom(String::from("Path already exists")))?;
            self._has_to_exist(parent, FileSystemError::Custom(String::from("Parent doesn't exists")))?;

            if parent.preview_is_a_dir(self) {
                self._create_dir(path)
            } else {
                Err(FileSystemError::Custom(String::from("Parent is not a directory")))
            }
        } else {
            Err(FileSystemError::Custom(String::from(format!("Invalid path given {}", path.display()))))
        }
    }

    fn create_dir_all<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        let mut ancestors : Vec<PathBuf> = path.ancestors().map(|p| p.to_path_buf()).collect();
        ancestors.reverse();
        for ancestor in ancestors.iter() {
            if ! ancestor.preview_exists(self) {
               self.create_dir(ancestor)?;
            }
        }
        Ok(())
    }

    /**
     * This function will overwrite the contents of to.
     * Errors :
     * - The `from` path is not a file.
     * - The `from` file does not exist.
     * - The current process does not have the permission rights to access `from` or write `to`.
     */
    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<u64> {
        let from = from.as_ref();
        let to = to.as_ref();
        self._has_to_exist(from, FileSystemError::Custom("From does not exists".into()))?;

        if from.preview_is_a_file(self) {
            self._copy(from, to)
        } else {
            Err(FileSystemError::Custom("From is not a file".into()))
        }
    }


    /// Because of this, the behavior when both `from` and `to` exist differs. On
    /// Unix, if `from` is a directory, `to` must also be an (empty) directory. If
    /// `from` is not a directory, `to` must also be not a directory. In contrast,
    /// on Windows, `from` can be anything, but `to` must *not* be a directory.
    /// Errors :
    /// * `from` does not exist.
    /// * The user lacks permissions to view contents.
    /// * `from` and `to` are on separate filesystems.
    #[cfg(target_family = "unix")]
    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<()> {
        let from = from.as_ref();
        let to = to.as_ref();
        self._has_to_exist(from, FileSystemError::Custom("From does not exists".into()))?;

        if self._are_on_same_filesystem(from, to) {
            if to.preview_exists(self) {
                let is_to_a_dir = to.preview_is_a_dir(self);
                if from.preview_is_a_dir(self) {
                    if is_to_a_dir && to.preview_read_dir(self)?.next().is_none() {
                        self._rename(from, to)
                    } else {
                        Err(FileSystemError::Custom("To has to be an empty dir".into()))
                    }
                } else if ! is_to_a_dir {
                    self._rename(from, to)
                } else {
                    Err(FileSystemError::Custom("To cannot be a directory".into()))
                }
            } else {
                self._rename(from, to)
            }
        } else {
            Err(FileSystemError::Custom("From and to are not on the same filesystem".into()))
        }
    }

    #[cfg(target_family = "windows")]
    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<()> {
        let from = from.as_ref();
        let to = to.as_ref();
        self._has_to_exists(from, FileSystemError::Custom("From does not exists".into()))?;

        if self._are_on_same_filesystem(from, to) {
            if to.preview_exists(self) {
                if to.preview_is_a_dir(self) {
                    return Err(FileSystemError::Custom("To cannot be a directory".into()));
                }
            }
            self._rename(from, to)
        } else {
            Err(FileSystemError::Custom("From and to are not on the same filesystem".into()))
        }
    }

    /// Errors :
    /// * `path` doesn't exist.
    /// * `path` isn't a directory.
    /// * The user lacks permissions to remove the directory at the provided `path`.
    /// * The directory isn't empty.
    fn remove_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        self._has_to_exist(path, FileSystemError::Custom("Path does not exists".into()))?;

        if path.preview_is_a_dir(self) {
            if path.preview_read_dir(self)?.next().is_none() {
                self._remove(path)
            } else {
                Err(FileSystemError::Custom("Path is not empty".into()))
            }
        } else {
            Err(FileSystemError::Custom("Path is not a directory".into()))
        }
    }

    /// Errors:  cf remove_file & remove_dir
    fn remove_dir_all<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        self._has_to_exist(path, FileSystemError::Custom("Path does not exists".into()))?;

        if path.preview_is_a_dir(self) {
            self._remove(path)
        } else {
            Err(FileSystemError::Custom("Path is not a directory".into()))
        }
    }

    /// Errors:
    /// * `path` points to a directory.
    /// * The file doesn't exist.
    /// * The user lacks permissions to remove the file.
    fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        self._has_to_exist(path, FileSystemError::Custom("Path does not exists".into()))?;

        if path.preview_is_a_dir(self) {
            Err(FileSystemError::Custom("Path is a directory".into()))
        } else {
            self._remove(path)
        }
    }
}