// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{
    io::{ stdin },
    path::{ Path }
};

use futurecommander_filesystem::{
    DomainError,
    capability::{
        Capabilities,
        Guard,
        Capability,
        ZealousGuard,
        BlindGuard,
        QuietGuard
    }
};

#[derive(Debug, Default)]
pub struct InteractiveGuard {
    skip_all: Capabilities,
    allow_all: Capabilities
}


impl Guard for InteractiveGuard {
    fn authorize(&mut self, capability: Capability, default: bool, target: &Path) -> Result<bool, DomainError> {
        if self.skip_all.authorize(capability) {
            return Ok(false)
        }

        if ! default && ! self.allow_all.authorize(capability) {
            let mut input = String::new();
            println!("Allow {} for target {} ?([skip]/skip_all/allow/allow_all/cancel) : ", capability, target.to_string_lossy());
            stdin().read_line(&mut input)?;

            match input.trim() {
                "skip" => Ok(false),
                "allow" => Ok(true),
                "skip_all" => {
                    self.skip_all = self.skip_all + capability;
                    Ok(false)
                },
                "allow_all" => {
                    self.allow_all = self.allow_all + capability;
                    Ok(true)
                },
                "cancel" =>
                    Err(DomainError::UserCancelled)
                ,
                _ => Ok(false)
            }
        } else {
            Ok(true)
        }
    }
}

pub enum AvailableGuard {
    Zealed,
    Blind,
    Quiet,
    Interactive
}

impl AvailableGuard {
    pub fn to_guard(&self) -> Box<dyn Guard> {
        match self {
            AvailableGuard::Zealed => Box::new(ZealousGuard),
            AvailableGuard::Blind => Box::new(BlindGuard),
            AvailableGuard::Quiet => Box::new(QuietGuard),
            AvailableGuard::Interactive => Box::new(InteractiveGuard::default()),
        }
    }

    pub fn available(s: &str) -> bool {
        vec!["interactive", "zealed", "quiet", "blind"].contains(&s)
    }
}

impl From<&str> for AvailableGuard {
    fn from(s: &str) -> AvailableGuard {
        match s.trim() {
            "interactive" => AvailableGuard::Interactive,
            "zealed" => AvailableGuard::Zealed,
            "quiet" => AvailableGuard::Quiet,
            "blind" => AvailableGuard::Blind,
            _ => Self::default()
        }
    }
}

impl Default for AvailableGuard {
    fn default() -> AvailableGuard {
        AvailableGuard::Interactive
    }
}
