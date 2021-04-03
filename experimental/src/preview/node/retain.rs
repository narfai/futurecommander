/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::path::{ Path, PathBuf };

use crate::{
    Result
};

use super::{
    PreviewNode
};

impl PreviewNode {
    pub fn retain(&mut self, predicate: impl Fn(&Path, &PreviewNode) -> bool) -> Result<()> {
        unimplemented!()
    }
}