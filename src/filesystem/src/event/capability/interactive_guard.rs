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
    io::{ stdin },
    path::{ Path }
};

use serde::{ Serialize, Deserialize };

use crate::{
    DomainError,
    capability::{
        Capabilities,
        Guard,
        Capability
    }
};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct InteractiveGuard {
    skip_all: Capabilities,
    allow_all: Capabilities
}

#[typetag::serde]
impl Guard for InteractiveGuard {
    fn authorize(&mut self, capability: Capability, default: bool, target: &Path) -> Result<bool, DomainError> {
        if self.skip_all.authorize(capability) {
            return Ok(false)
        }

        if ! default && ! self.allow_all.authorize(capability) {
            //TODO put that into a callback and interface with an enum
            let mut input = String::new();
            println!("Allow {} for target {} ?([skip]/skip_all/allow/allow_all/cancel) : ", capability, target.to_string_lossy());
            stdin().read_line(&mut input)?;
            //

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
