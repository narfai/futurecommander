// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

#[derive(Debug)]
pub struct EntryAdapter<T>(pub T);
impl <T>EntryAdapter<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
    pub fn as_inner(&self) -> &T {
        &self.0
    }
}