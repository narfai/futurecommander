// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{
    io::{ stdin },
    path::{ Path }
};

use futurecommander_filesystem::{
    DomainError,
    Capabilities,
    Guard,
    Capability,
    ZealousGuard,
    BlindGuard,
    SkipGuard,
    PresetGuard
};

#[derive(Debug, Default)]
pub struct InteractiveGuard {
    skip_all: Capabilities,
    allow_all: Capabilities
}


impl Guard for InteractiveGuard {
    fn authorize(&mut self, target: &Path, capability: Option<Capability>) -> Result<bool, DomainError> {
        if let Some(capability) = capability {
            if self.skip_all.authorize(capability) {
                return Ok(false)
            }

            if ! self.allow_all.authorize(capability) {
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
        } else {
            Ok(true)
        }
    }
}

pub enum AvailableGuard {
    Zealed,
    Blind,
    Skip,
    Interactive
}

impl AvailableGuard {
    pub fn to_guard(&self, capabilities: Capabilities) -> Box<dyn Guard> {
        match self {
            AvailableGuard::Zealed => Box::new(PresetGuard::new(ZealousGuard, capabilities)),
            AvailableGuard::Blind => Box::new(PresetGuard::new(BlindGuard, capabilities)),
            AvailableGuard::Skip => Box::new(PresetGuard::new(SkipGuard, capabilities)),
            AvailableGuard::Interactive => Box::new(PresetGuard::new(InteractiveGuard::default(), capabilities)),
        }
    }

    pub fn available(s: &str) -> bool {
        vec!["interactive", "zealed", "skip", "blind"].contains(&s)
    }
}

impl From<&str> for AvailableGuard {
    fn from(s: &str) -> AvailableGuard {
        match s.trim() {
            "interactive" => AvailableGuard::Interactive,
            "zealed" => AvailableGuard::Zealed,
            "skip" => AvailableGuard::Skip,
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
