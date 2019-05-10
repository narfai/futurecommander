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
    path::{ Path },
    fmt::{ Debug }
};


use serde::{ Serialize, Deserialize };

use crate::{
    DomainError,
    capability::{
        Capability
    }
};

#[typetag::serde(tag = "type")]
pub trait Guard : Debug {
    fn authorize(&mut self, capability: Capability, default: bool, target: &Path) -> Result<bool, DomainError>;
}

#[derive(Serialize, Deserialize,Debug, Clone)]
pub struct BlindGuard;

#[typetag::serde]
impl Guard for BlindGuard {
    fn authorize(&mut self, _capability: Capability, _default: bool, _target: &Path) -> Result<bool, DomainError> {
        Ok(true)
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuietGuard;

#[typetag::serde]
impl Guard for QuietGuard {
    fn authorize(&mut self, _capability: Capability, default: bool, _target: &Path) -> Result<bool, DomainError> {
        Ok(default)
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZealedGuard;

#[typetag::serde]
impl Guard for ZealedGuard {
    fn authorize(&mut self, capability: Capability, default: bool, target: &Path) -> Result<bool, DomainError> {
        match capability {
            Capability::Merge => {
                if default {
                    Ok(true)
                } else {
                    Err(
                        DomainError::MergeNotAllowed(
                            target.to_path_buf()
                        )
                    )
                }
            },
            Capability::Overwrite => {
                if default {
                    Ok(true)
                } else {
                    Err(
                        DomainError::OverwriteNotAllowed(
                            target.to_path_buf()
                        )
                    )
                }
            },
            Capability::Recursive => {
                if default {
                    Ok(true)
                } else {
                    Err(
                        DomainError::RecursiveNotAllowed(
                            target.to_path_buf()
                        )
                    )
                }
            }
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        error
    };

    fn assert_two_errors_equals(left: &impl error::Error, right: &impl error::Error) {
        assert_eq!(format!("{}", left), format!("{}", right))
    }

    #[test]
    fn all_guards_let_default_through(){
        let target = Path::new("/virtual/directory");
        let default = true;

        let mut zealed = ZealedGuard;
        assert!(zealed.authorize(Capability::Overwrite, default, target).unwrap());
        assert!(zealed.authorize(Capability::Merge, default, target).unwrap());
        assert!(zealed.authorize(Capability::Recursive, default, target).unwrap());

        let mut blind = BlindGuard;
        assert!(blind.authorize(Capability::Overwrite, default, target).unwrap());
        assert!(blind.authorize(Capability::Merge, default, target).unwrap());
        assert!(blind.authorize(Capability::Recursive, default, target).unwrap());

        let mut quiet = QuietGuard;
        assert!(quiet.authorize(Capability::Overwrite, default, target).unwrap());
        assert!(quiet.authorize(Capability::Merge, default, target).unwrap());
        assert!(quiet.authorize(Capability::Recursive, default, target).unwrap());
    }

    #[test]
    fn zealed_guard_block_sensible_operation(){
        let mut guard = ZealedGuard;
        let target = Path::new("/virtual/directory");
        let default = false;

        assert_two_errors_equals(
            &guard.authorize(Capability::Overwrite, default, target).err().unwrap(),
            &DomainError::OverwriteNotAllowed(target.to_path_buf())
        );
        assert_two_errors_equals(
            &guard.authorize(Capability::Merge, default, target).err().unwrap(),
            &DomainError::MergeNotAllowed(target.to_path_buf())
        );
        assert_two_errors_equals(
            &guard.authorize(Capability::Recursive, default, target).err().unwrap(),
            &DomainError::RecursiveNotAllowed(target.to_path_buf())
        );
    }

    #[test]
    fn quiet_guard_skip_sensible_operation(){
        let mut guard = QuietGuard;
        let target = Path::new("/virtual/directory");
        let default = false;

        assert_eq!(guard.authorize(Capability::Overwrite, default, target).unwrap(), false);
        assert_eq!(guard.authorize(Capability::Merge, default, target).unwrap(), false);
        assert_eq!(guard.authorize(Capability::Recursive, default, target).unwrap(), false);
    }

    #[test]
    fn blind_guard_let_any_sensible_operation_through(){
        let mut guard = BlindGuard;
        let target = Path::new("/virtual/directory");
        let default = false;

        assert_eq!(guard.authorize(Capability::Overwrite, default, target).unwrap(), true);
        assert_eq!(guard.authorize(Capability::Merge, default, target).unwrap(), true);
        assert_eq!(guard.authorize(Capability::Recursive, default, target).unwrap(), true);
    }
}
