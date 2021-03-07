// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

mod zealous;
mod preset;
mod skip;
mod blind;

use std::path::Path;
use crate::{
    DomainError,
    Capability
};
pub use self::{
    zealous::ZealousGuard,
    preset::PresetGuard,
    blind::BlindGuard,
    skip::SkipGuard
};

pub trait Guard {
    fn authorize(&mut self, capability: Capability, target: &Path) -> Result<bool, DomainError>;
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    use std::{ error };

    use crate::capability::{ Capability, Capabilities };

    fn assert_two_errors_equals(left: &impl error::Error, right: &impl error::Error) {
        assert_eq!(format!("{}", left), format!("{}", right))
    }

    #[test]
    fn preset_guard_let_any_sensible_operation_through(){
        let target = Path::new("/virtual/directory");
        use Capability::*;

        let mut preset = PresetGuard::new(
            ZealousGuard,
            Capabilities::default() + Overwrite + Merge + Recursive
        );
        assert!(preset.authorize(Capability::Overwrite, target).unwrap());
        assert!(preset.authorize(Capability::Merge, target).unwrap());
        assert!(preset.authorize(Capability::Recursive, target).unwrap());
    }

    #[test]
    fn zealous_guard_block_sensible_operation(){
        let mut guard = ZealousGuard;
        let target = Path::new("/virtual/directory");

        assert_two_errors_equals(
            &guard.authorize(Capability::Overwrite, target).err().unwrap(),
            &DomainError::OverwriteNotAllowed(target.to_path_buf())
        );
        assert_two_errors_equals(
            &guard.authorize(Capability::Merge, target).err().unwrap(),
            &DomainError::MergeNotAllowed(target.to_path_buf())
        );
        assert_two_errors_equals(
            &guard.authorize(Capability::Recursive, target).err().unwrap(),
            &DomainError::RecursiveNotAllowed(target.to_path_buf())
        );
    }

    #[test]
    fn skip_guard_skip_sensible_operation(){
        let mut guard = SkipGuard;
        let target = Path::new("/virtual/directory");

        assert_eq!(guard.authorize(Capability::Overwrite, target).unwrap(), false);
        assert_eq!(guard.authorize(Capability::Merge, target).unwrap(), false);
        assert_eq!(guard.authorize(Capability::Recursive, target).unwrap(), false);
    }

    #[test]
    fn blind_guard_let_any_sensible_operation_through(){
        let mut guard = BlindGuard;
        let target = Path::new("/virtual/directory");
        let default = false;

        assert_eq!(guard.authorize(Capability::Overwrite, target).unwrap(), true);
        assert_eq!(guard.authorize(Capability::Merge, target).unwrap(), true);
        assert_eq!(guard.authorize(Capability::Recursive, target).unwrap(), true);
    }
}
