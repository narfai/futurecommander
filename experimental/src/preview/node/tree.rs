/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::{
    ffi::OsStr,
    io::Write,
    path::{Path, PathBuf}
};

use crate::{
    Preview,
    ReadFileSystem,
    Result
};
use crate::filesystem::PathExt;

fn display_tree_line<W: Write>(out: &mut W, depth_list: &Option<Vec<bool>>, parent_last: bool, file_name: String) -> Result<()> {
    if let Some(depth_list) = &depth_list {
        writeln!(
            out,
            "{}{}── {}",
            depth_list.
                iter().fold(
                "".to_string(),
                |depth_delimiter, last|
                    depth_delimiter + if *last { "    " } else { "│   " }
            ),
            if parent_last { '└' } else { '├' },
            file_name
        )?;
    } else {
        writeln!(out, "{}", file_name)?;
        writeln!(out, "│")?;
    }
    Ok(())
}


pub fn tree<W: Write, R: ReadFileSystem>(out: &mut W, preview: &R, identity: &Path, depth_list: Option<Vec<bool>>, parent_last: bool) -> Result<()>{
    let file_name = identity.file_name().unwrap_or_else(|| OsStr::new("ROOT"));

    display_tree_line(
        out,
        &depth_list,
        parent_last,
        file_name.to_string_lossy().to_string()
    )?;

    let new_depth_list = match depth_list {
        Some(depth_list) => {
            let mut new = depth_list;
            new.push(parent_last);
            new
        },
        None => vec![]
    };

    if identity.preview_is_a_dir(preview) {
        let collection: Vec<PathBuf> = identity.preview_read_dir(preview)?
            .filter_map(|entry| entry.ok())
            .map(|e| e.path())
            .collect();

        let length = collection.len();
        for (index, child) in collection.into_iter().enumerate() {
            tree(
                out,
                preview,
                &child,
                Some(new_depth_list.clone()),
                index == (length - 1)
            )?
        }
    }

    Ok(())
}

impl Preview {
    pub fn tree<W: Write>(&self, out: &mut W, path: &Path) -> Result<()>{
        tree(out, self, path, None, true)?;
        Ok(())
    }
}