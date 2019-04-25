/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommander.
 *
 * FutureCommander is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommander is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommander.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::path::{ Path, PathBuf, Component };

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
