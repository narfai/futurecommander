/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

mod node;
mod read_filesystem;
mod write_filesystem;
mod internal;

pub use node::{Node, NodeFileType};

#[derive(Default)]
pub struct Preview {
    root: Node
}