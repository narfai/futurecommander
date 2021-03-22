use std::{
    path::Path,
};

use super::{
    Preview,
    node::Node
};

use crate::{
    Result,
    FileSystemError,
    filesystem::PathExt
};

impl Preview {
    pub (in super) fn _create_file(&mut self, path: &Path) -> Result<()> {
        path.file_name().map(|file_name| {
            self.root
                .filter(|parent_path, child| &parent_path.join(child.name()) != path)?
                .insert_at(path, &Node::new_file(&file_name.to_string_lossy(), None))?;
        }).ok_or(FileSystemError::Custom(String::from("Cannot obtain file name")))
    }

    pub (in super) fn _create_dir(&mut self, path: &Path) -> Result<()> {
        path.file_name().map(|file_name| {
            self.root
                .filter(|parent_path, child| &parent_path.join(child.name()) != path)?
                .insert_at(path, &Node::new_directory(&file_name.to_string_lossy()))?;
        }).ok_or(FileSystemError::Custom(String::from("Cannot obtain file name")))
    }

    pub (in super) fn _rename_file(&mut self, from: &Path, to: &Path) -> Result<()> {
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

    pub (in super) fn _rename(&mut self, from: &Path, to: &Path) -> Result<()> {
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

    pub (in super) fn _copy(&mut self, from: &Path, to: &Path) -> Result<u64> {
        self.root
            .filter(|parent_path, child| &parent_path.join(child.name()) != to)?
            .insert_at(
                to.parent().unwrap(),
                &Node::new_file(&to.file_name().unwrap().to_string_lossy(), Some(from.to_path_buf()))
            )?;
        Ok(0)
    }

    //TODO
    pub (in super) fn _are_on_same_filesystem(&self, _left: &Path, _right: &Path) -> bool {
        true
    }

    pub (in super) fn _remove(&mut self, path: &Path) -> Result<()> {
        self.root
            .filter(|parent_path, child| &parent_path.join(child.name()) != path)?
            .insert_at(path, &Node::new_deleted(&path.file_name().unwrap().to_string_lossy()))?;

        Ok(())
    }

    pub (in super) fn _has_to_exist(&self, path: &Path, error: FileSystemError) -> Result<()> {
        if path.preview_exists(self) {
            Ok(())
        } else {
            Err(error)
        }
    }

    pub (in super) fn _has_to_not_exist(&self, path: &Path, error: FileSystemError) -> Result<()> {
        if path.preview_exists(self) {
            Err(error)
        } else {
            Ok(())
        }
    }
}
