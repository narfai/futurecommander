// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::path::{ PathBuf, Path, MAIN_SEPARATOR, Component };

pub fn root_identity() -> PathBuf {
    PathBuf::from(MAIN_SEPARATOR.to_string())
}

pub fn get_parent_or_root(identity: &Path) -> PathBuf {
    match identity.parent() {
        Some(parent) => parent.to_path_buf(),
        None => root_identity()
    }
}

pub fn absolute(cwd: &Path, path: &Path) -> PathBuf {
    normalize(&cwd.join(path) )
}

/**
* Thanks to ThatsGobbles ( https://github.com/ThatsGobbles ) for his solution : https://github.com/rust-lang/rfcs/issues/2208
* This code will be removed when os::make_absolute will be marked as stable
*/
pub fn normalize(p: &Path) -> PathBuf {
    let mut stack: Vec<Component<'_>> = vec![];

    for component in p.components() {
        match component {
            Component::CurDir => {},
            Component::ParentDir => {
                match stack.last().cloned() {
                    Some(c) => {
                        match c {
                            Component::Prefix(_) => { stack.push(component); },
                            Component::RootDir => {},
                            Component::CurDir => { unreachable!(); },
                            Component::ParentDir => { stack.push(component); },
                            Component::Normal(_) => { let _ = stack.pop(); }
                        }
                    },
                    None => { stack.push(component); }
                }
            },
            _ => { stack.push(component); },
        }
    }

    if stack.is_empty() {
        return PathBuf::from(".");
    }

    let mut norm_path = PathBuf::new();

    for item in &stack {
        norm_path.push(item);
    }

    norm_path
}