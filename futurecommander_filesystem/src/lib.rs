// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

mod errors;
mod capability;
mod entry;
mod infrastructure;
mod operation;
mod guard;
mod container;


pub use futurecommander_representation::{
    errors::RepresentationError,
    Kind
};

//TODO sub mod internal

pub use self::{
    errors::{ DomainError, QueryError },
    capability::{
        Capability,
        Capabilities
    },
    entry::{
        Entry,
        EntryCollection,
        EntryAdapter
    },
    infrastructure::{
        InfrastructureError,
        ReadableFileSystem,
        WriteableFileSystem
    },
    operation::{
        Request
    },
    guard::{
        ZealousGuard,
        SkipGuard,
        PresetGuard,
        BlindGuard
    },
    container::Container
};


#[cfg(not(tarpaulin_include))]
pub mod sample;
