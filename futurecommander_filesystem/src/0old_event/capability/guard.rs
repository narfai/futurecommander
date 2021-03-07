// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{
    path::{ Path },
    fmt::{ Debug }
};

use crate::{
    DomainError,
    capability::{
        Capability
    }
};


// TODO : add an implementation of WriteGuard<W> and ReadGuard<R> for sharing W and R streams types respectively
// That implementation should by example, in the shell, allow the InteractiveGuard to Read from stdin and write in stdout
// Plus : guard may not be a trait with is subspecialization but much more a generic wrapper/adapter Guard<T> ( more idomatic )
// pub struct WriteGuard<W: Write>(Box<dyn Guard>, W);
// pub struct ReadGuard<R: Read>(Box<dyn Guard>, R);
// Note : it would probably make the methods emit and delay use generics

pub trait Guard {
    fn authorize(&mut self, capability: Capability, default: bool, target: &Path) -> Result<bool, DomainError>;
}

#[derive(Debug)]
pub struct BlindGuard;

impl Guard for BlindGuard {
    fn authorize(&mut self, _capability: Capability, _default: bool, _target: &Path) -> Result<bool, DomainError> {
        Ok(true)
    }
}

#[derive(Debug)]
pub struct QuietGuard;

impl Guard for QuietGuard {
    fn authorize(&mut self, _capability: Capability, default: bool, _target: &Path) -> Result<bool, DomainError> {
        Ok(default)
    }
}


#[derive(Debug)]
pub struct ZealousGuard;

impl Guard for ZealousGuard {
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

#[cfg(not(tarpaulin_include))]
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

        let mut zealed = ZealousGuard;
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
        let mut guard = ZealousGuard;
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
