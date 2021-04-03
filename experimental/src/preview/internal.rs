use std::{
    path::Path,
};

use super::{
    Preview,
    node::{ PreviewNode }
};

use crate::{
    Result,
    FileSystemError,
    filesystem::PathExt
};

impl Preview {
    pub (in super) fn _create_file(&mut self, path: &Path) -> Result<()> {
        let file_name = path.file_name().ok_or_else(|| FileSystemError::Custom(String::from("Cannot obtain file name")))?;

        self.root.retain(|parent_path, child| parent_path.join(child.name()) != path)?;
        self.root.insert_at_path(path, &PreviewNode::new_file(&file_name.to_string_lossy(), None))?;

        Ok(())
    }

    pub (in super) fn _create_dir(&mut self, path: &Path) -> Result<()> {
        let file_name = path.file_name().ok_or_else(|| FileSystemError::Custom(String::from("Cannot obtain file name")))?;

        self.root.retain(|parent_path, child| parent_path.join(child.name()) != path)?;
        self.root.insert_at_path(path, &PreviewNode::new_directory(&file_name.to_string_lossy()))?;

        Ok(())
    }

    pub (in super) fn _rename_file(&mut self, from: &Path, to: &Path) -> Result<()> {
        let source = self.root.find_at_path(from)
            .and_then(|node| node.source())
            .map(|src| src.to_path_buf())
            .or_else(|| Some(from.to_path_buf()));

        self.root.retain(|parent_path, child| parent_path.join(child.name()) != from || parent_path.join(child.name()) != to)?;
        self.root.insert_at_path(
                to.parent().unwrap(),
                &PreviewNode::new_deleted(&from.file_name().unwrap().to_string_lossy())
            )?;
        self.root.insert_at_path(
                to.parent().unwrap(),
                &PreviewNode::new_file(&to.file_name().unwrap().to_string_lossy(), source)
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
        self.root.retain(|parent_path, child| parent_path.join(child.name()) != to)?;
        self.root.insert_at_path(
                to.parent().unwrap(),
                &PreviewNode::new_file(&to.file_name().unwrap().to_string_lossy(), Some(from.to_path_buf()))
            )?;
        Ok(0)
    }

    //TODO
    pub (in super) fn _are_on_same_filesystem(&self, _left: &Path, _right: &Path) -> bool {
        true
    }

    pub (in super) fn _remove(&mut self, path: &Path) -> Result<()> {
        self.root.retain(|parent_path, child| parent_path.join(child.name()) != path)?;
        self.root.insert_at_path(path, &PreviewNode::new_deleted(&path.file_name().unwrap().to_string_lossy()))?;

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