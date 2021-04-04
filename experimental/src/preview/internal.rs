/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::{
    path::Path,
};

use crate::{
    filesystem::PathExt,
    FileSystemError,
    Result
};

use super::{
    node::PreviewNode,
    Preview
};

impl Preview {
    pub (in super) fn _create_file(&mut self, path: &Path) -> Result<()> {
        let file_name = path.file_name().ok_or_else(|| FileSystemError::PathTerminatesInTwoDot(path.to_owned()))?;
        let parent = path.parent().ok_or_else(|| FileSystemError::PathTerminatesInARootOrPrefix(path.to_owned()))?;

        self.root.retain(&|parent_path, child| parent_path.join(child.name()) != path)?;
        self.root.insert_at_path(parent, PreviewNode::new_file(&file_name, None))?;

        Ok(())
    }

    pub (in super) fn _create_dir(&mut self, path: &Path) -> Result<()> {
        let file_name = path.file_name().ok_or_else(|| FileSystemError::PathTerminatesInTwoDot(path.to_owned()))?;
        let parent = path.parent().ok_or_else(|| FileSystemError::PathTerminatesInARootOrPrefix(path.to_owned()))?;

        self.root.retain(&|parent_path, child| parent_path.join(child.name()) != path)?;
        self.root.insert_at_path(parent, PreviewNode::new_directory(file_name))?;

        Ok(())
    }

    pub (in super) fn _rename_file(&mut self, from: &Path, to: &Path) -> Result<()> {
        let source = self.root.find_at_path(from)
            .and_then(|node| node.source())
            .map(|src| src.to_path_buf())
            .or_else(|| Some(from.to_path_buf()));

        let to_parent = to.parent().ok_or_else(|| FileSystemError::PathTerminatesInARootOrPrefix(to.to_owned()))?;
        let from_file_name = from.file_name().ok_or_else(|| FileSystemError::PathTerminatesInTwoDot(from.to_owned()))?;
        let to_file_name = to.file_name().ok_or_else(|| FileSystemError::PathTerminatesInTwoDot(to.to_owned()))?;

        self.root.retain(&|parent_path, child| parent_path.join(child.name()) != from || parent_path.join(child.name()) != to)?;
        self.root.insert_at_path(
            to_parent,
            PreviewNode::new_deleted(from_file_name)
        )?;
        self.root.insert_at_path(
            to_parent,
            PreviewNode::new_file(to_file_name, source)
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
        let to_parent = to.parent().ok_or_else(|| FileSystemError::PathTerminatesInARootOrPrefix(to.to_owned()))?;
        let to_file_name = to.file_name().ok_or_else(|| FileSystemError::PathTerminatesInTwoDot(to.to_owned()))?;

        self.root.retain(&|parent_path, child| parent_path.join(child.name()) != to)?;
        self.root.insert_at_path(
            to_parent,
            PreviewNode::new_file(to_file_name, Some(from.to_path_buf()))
        )?;
        Ok(0)
    }

    //TODO
    pub (in super) fn _are_on_same_filesystem(&self, _left: &Path, _right: &Path) -> bool {
        true
    }

    pub (in super) fn _remove(&mut self, path: &Path) -> Result<()> {
        let parent = path.parent().ok_or_else(|| FileSystemError::PathTerminatesInARootOrPrefix(path.to_owned()))?;

        self.root.retain(&|parent_path, child| parent_path.join(child.name()) != path)?;
        self.root.insert_at_path(parent, PreviewNode::new_deleted(path.file_name().unwrap()))?;

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