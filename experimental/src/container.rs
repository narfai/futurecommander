/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

mod operation;
mod read_filesystem;
mod write_filesystem;

use crate::{ Result, Preview };

#[derive(Default)]
pub struct Container {
    operation_list: Vec<operation::Operation>,
    preview: Preview
}

impl Container {
    pub fn apply(&mut self) -> Result<()> {
        for op in &self.operation_list {
            op.apply()?
        }
        self.operation_list = Vec::new();
        Ok(())
    }
}

//TODO apply
//TODO to_json
//TODO apply_json

//TODO guards & so on ...