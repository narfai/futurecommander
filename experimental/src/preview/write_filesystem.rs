/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::{
    path::{ Path, PathBuf },
};

use super::Preview;

use crate::{
    Result,
    FileSystemError,
    WriteFileSystem,
    filesystem::PathExt
};

// TODO map same error behaviors according to real std::fs
impl WriteFileSystem for Preview {
    fn create_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        let parent = path.parent().ok_or_else(|| FileSystemError::PathTerminatesInARootOrPrefix(path.to_owned()))?;

        self._has_to_not_exist(path, |path|FileSystemError::PathAlreadyExists(path.to_owned()))?;
        self._has_to_exist(parent, |path| FileSystemError::ParentDoesNotExists(path.to_owned()))?;

        if parent.preview_is_a_dir(self) {
            self._create_file(path)
        } else {
            Err(FileSystemError::ParentIsNotADirectory(parent.to_owned()))
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
        let parent = path.parent().ok_or_else(|| FileSystemError::PathTerminatesInARootOrPrefix(path.to_owned()))?;

        self._has_to_not_exist(path, |path| FileSystemError::PathAlreadyExists(path.to_owned()))?;
        self._has_to_exist(parent, |path| FileSystemError::ParentDoesNotExists(path.to_owned()))?;

        if parent.preview_is_a_dir(self) {
            self._create_dir(path)
        } else {
            Err(FileSystemError::ParentIsNotADirectory(parent.to_owned()))
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
        self._has_to_exist(from, |path| FileSystemError::FromDoesNotExists(path.to_owned()))?;

        if from.preview_is_a_file(self) {
            let parent = to.parent().ok_or_else(|| FileSystemError::PathTerminatesInARootOrPrefix(to.to_owned()))?;
            if ! parent.preview_exists(self) {
                Err(FileSystemError::ToParentDoesNotExists(parent.to_owned()))
            } else if ! parent.preview_is_a_dir(self) {
                Err(FileSystemError::ToParentIsNotADirectory(parent.to_owned()))
            } else {
                self._copy(from, to)
            }
        } else {
            Err(FileSystemError::FromIsNotAFile(from.to_owned()))
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
        self._has_to_exist(from, |path| FileSystemError::FromDoesNotExists(path.to_owned()))?;

        if self._are_on_same_filesystem(from, to) {
            if to.preview_exists(self) {
                let is_to_a_dir = to.preview_is_a_dir(self);
                if from.preview_is_a_dir(self) {
                    if is_to_a_dir && to.preview_read_dir(self)?.next().is_none() {
                        self._rename(from, to)
                    } else {
                        Err(FileSystemError::ToDirectoryIsNotEmpty(to.to_owned()))
                    }
                } else if ! is_to_a_dir {
                    self._rename(from, to)
                } else {
                    Err(FileSystemError::ToCannotBeADirectory(to.to_owned()))
                }
            } else {
                self._rename(from, to)
            }
        } else {
            Err(FileSystemError::FromAndToAreNotOnTheSameFileSystem(from.to_owned(), to.to_owned()))
        }
    }

    // TODO THINK : a stored operation may not execute on the same system it was created initially and previewed
    // Therefor, it may create differences between preview and actual future processing of the wanted operation
    #[cfg(target_family = "windows")]
    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<()> {
        let from = from.as_ref();
        let to = to.as_ref();
        self._has_to_exists(from, |path| FileSystemError::FromDoesNotExists(path.to_owned()))?;

        if self._are_on_same_filesystem(from, to) {
            if to.preview_exists(self) && to.preview_is_a_dir(self) {
                Err(FileSystemError::ToCannotBeADirectory(to.to_owned()));
            } else {
                self._rename(from, to)
            }
        } else {
            Err(FileSystemError::FromAndToAreNotOnTheSameFileSystem(from.to_owned(), to.to_owned()))
        }
    }

    /// Errors :
    /// * `path` doesn't exist.
    /// * `path` isn't a directory.
    /// * The user lacks permissions to remove the directory at the provided `path`.
    /// * The directory isn't empty.
    fn remove_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        self._has_to_exist(path, |path| FileSystemError::PathDoesNotExists(path.to_owned()))?;

        if path.preview_is_a_dir(self) {
            if path.preview_read_dir(self)?.next().is_none() {
                self._remove(path)
            } else {
                Err(FileSystemError::DirectoryIsNotEmpty(path.to_owned()))
            }
        } else {
            Err(FileSystemError::PathIsNotADirectory(path.to_owned()))
        }
    }

    /// Errors:  cf remove_file & remove_dir
    fn remove_dir_all<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        self._has_to_exist(path, |path| FileSystemError::PathDoesNotExists(path.to_owned()))?;

        if path.preview_is_a_dir(self) {
            self._remove(path)
        } else {
            Err(FileSystemError::PathIsNotADirectory(path.to_owned()))
        }
    }

    /// Errors:
    /// * `path` points to a directory.
    /// * The file doesn't exist.
    /// * The user lacks permissions to remove the file.
    fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        self._has_to_exist(path, |path| FileSystemError::PathDoesNotExists(path.to_owned()))?;

        if path.preview_is_a_dir(self) {
            Err(FileSystemError::PathIsADirectory(path.to_owned()))
        } else {
            self._remove(path)
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        ffi::OsStr,
        path::PathBuf,
        collections::HashSet,
        io::{stdout}
    };
    use super::*;
    use crate::{
        sample::*,
        filesystem::{PathExt, FileTypeExt},
        Node
    };


    #[test]
    fn created_file_exists_virtually() {
        let chroot_path = static_samples_path();
        let target_path = chroot_path.join("HAS_TO_EXISTS");

        let mut preview = Preview::default();

        preview.create_file(&target_path).unwrap();
        assert!(target_path.preview_exists(&preview));
        assert!(target_path.preview_is_a_file(&preview));
        assert!(!target_path.exists());
    }

    #[test]
    fn created_dir_exists_virtually() {
        let chroot_path = static_samples_path();
        let target_path = chroot_path.join("HAS_TO_EXISTS");

        let mut preview = Preview::default();
        preview.create_dir(&target_path).unwrap();
        assert!(target_path.preview_exists(&preview));
        assert!(target_path.preview_is_a_dir(&preview));
        assert!(!target_path.exists());
    }

    #[test]
    fn create_dir_recursively() {
        let chroot_path = static_samples_path();
        let sub_a = chroot_path.join("SUBA");
        let sub_b = sub_a.join("SUBB");
        let target_path = sub_b.join("HAS_TO_EXISTS");

        let mut preview = Preview::default();
        preview.create_dir_all(&target_path).unwrap();
        assert!(target_path.preview_exists(&preview));
        assert!(sub_b.preview_exists(&preview));
        assert!(sub_a.preview_exists(&preview));

        assert!(target_path.preview_is_a_dir(&preview));
        assert!(sub_b.preview_is_a_dir(&preview));
        assert!(sub_a.preview_is_a_dir(&preview));

        assert!(!target_path.exists());
        assert!(!sub_b.exists());
        assert!(!sub_a.exists());
    }

    #[test]
    fn copied_file_exists_virtually_and_keep_track_of_source() {
        let chroot_path = static_samples_path();
        let source = chroot_path.join("F");
        let target_path = chroot_path.join("HAS_TO_EXISTS");

        let mut preview = Preview::default();
        preview.copy(&source, &target_path).unwrap();
        assert!(target_path.preview_exists(&preview));
        assert!(target_path.preview_is_a_file(&preview));
        assert!(!target_path.exists());

        assert!(source.preview_exists(&preview));
        assert_eq!(&source, preview.root.find_at_path(&target_path).unwrap().source().unwrap());
    }

    #[test]
    fn renamed_file_exists_virtually_and_keep_track_of_source() {
        let chroot_path = static_samples_path();
        let source = chroot_path.join("F");
        let target_path = chroot_path.join("HAS_TO_EXISTS");

        let mut preview = Preview::default();
        preview.rename(&source, &target_path).unwrap();
        assert!(target_path.preview_exists(&preview));
        assert!(target_path.preview_is_a_file(&preview));
        assert!(!target_path.exists());

        assert!(!source.preview_exists(&preview));
        assert_eq!(&source, preview.root.find_at_path(&target_path).unwrap().source().unwrap());
    }

    #[test]
    fn removed_empty_dir_does_not_exists_virtually() {
        let chroot = Chroot::new("preview_removed_empty_dir_does_not_exists_virtually");
        chroot.init_empty();
        let a = chroot.create_dir("A");

        let mut preview = Preview::default();
        preview.remove_dir(&a).unwrap();
        assert!(!a.preview_exists(&preview));
        assert!(a.exists());

        chroot.clean();
    }

    #[test]
    fn removed_dir_and_children_does_not_exists_virtually() {
        let chroot_path = static_samples_path();
        let a = chroot_path.join("A");
        let a_c = a.join("C");
        let a_gitkeep = a.join(".gitkeep");

        let mut preview = Preview::default();

        preview.remove_dir_all(&a).unwrap();
        assert!(!a.preview_exists(&preview));
        assert!(!a_c.preview_exists(&preview));
        assert!(!a_gitkeep.preview_exists(&preview));

        assert!(a.exists());
        assert!(a_c.exists());
        assert!(a_gitkeep.exists());
    }

    #[test]
    fn removed_file_does_not_exists_virtually() {
        let chroot_path = static_samples_path();
        let f = chroot_path.join("F");

        let mut preview = Preview::default();

        preview.remove_file(&f).unwrap();
        assert!(!f.preview_exists(&preview));
        assert!(f.exists());
    }
}