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

use std::{
    io::{ stdin, stdout, Read, Write },
    path::{ Path }
};

use serde::{ Serialize, Deserialize };

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
