// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VirtualState {
    Exists, //Exists only in real FS
    ExistsVirtually, //Directly added
    ExistsThroughVirtualParent, //Indirectly added
    Replaced,
    NotExists, //Does not exists in virtual fs or real, indirectly or not
    Removed, //Does exists in real fs and should be deleted
    RemovedVirtually, //Does exists in virtual but is also virtually deleted
}
