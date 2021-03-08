// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

#[derive(Debug)]
pub struct FileSystemAdapter<F>(pub F);
impl <F>FileSystemAdapter<F> {
    pub fn as_inner(&self) -> &F {
        &self.0
    }

    pub fn as_inner_mut(&mut self) -> &mut F {
        &mut self.0
    }
}