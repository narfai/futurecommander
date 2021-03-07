// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::path::{ PathBuf };
use std::{ error, fmt };

#[derive(Debug)]
pub enum RepresentationError {
    AlreadyExists(PathBuf),
    VirtualParentIsAFile(PathBuf),
    DoesNotExists(PathBuf),
    IsNotADirectory(PathBuf),
    IsRelativePath(PathBuf),
}
impl fmt::Display for RepresentationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepresentationError::AlreadyExists(identity) => write!(f, "Identity {} already exists", identity.as_os_str().to_string_lossy()),
            RepresentationError::VirtualParentIsAFile(identity) => write!(f, "Identity parent {} is a file", identity.as_os_str().to_string_lossy()),
            RepresentationError::DoesNotExists(identity) => write!(f, "Identity {} does not exists", identity.as_os_str().to_string_lossy()),
            RepresentationError::IsNotADirectory(identity) => write!(f, "Identity {} is not a directory", identity.as_os_str().to_string_lossy()),
            RepresentationError::IsRelativePath(identity) => write!(f, "Path {} is relative", identity.as_os_str().to_string_lossy()),
        }
    }
}

impl error::Error for RepresentationError {}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod errors_tests {
    use super::*;

    use std::{
        path::{ Path, PathBuf }
    };

    use crate::*;

    fn assert_two_errors_equals(left: &impl error::Error, right: &impl error::Error) {
        assert_eq!(format!("{}", left), format!("{}", right))
    }

    #[test]
    fn error_already_exists(){
        let mut delta = VirtualDelta::default();
        delta.attach(Path::new("/TEST"), None, Kind::File).unwrap();
        assert_two_errors_equals(
            &delta.attach(Path::new("/TEST"), None, Kind::File).err().unwrap(),
            &RepresentationError::AlreadyExists(PathBuf::from("/TEST"))
        );
    }

    #[test]
    fn error_virtual_parent_is_a_file(){
        let mut delta = VirtualDelta::default();
        delta.attach(Path::new("/PARENT"), None, Kind::File).unwrap();
        assert_two_errors_equals(
             &delta.attach(Path::new("/PARENT/CHILD"), None, Kind::File).err().unwrap(),
            &RepresentationError::VirtualParentIsAFile(PathBuf::from("/PARENT/CHILD"))
        );
    }

    #[test]
    fn error_does_not_exists(){
        let mut delta = VirtualDelta::default();
        assert_two_errors_equals(
            &delta.detach(Path::new("/DOES/NOT/EXISTS")).err().unwrap(),
            &RepresentationError::DoesNotExists(PathBuf::from("/DOES/NOT/EXISTS"))
        );
    }

    #[test]
    fn error_is_relative_path(){
        assert_two_errors_equals(
            &VirtualPath::from(PathBuf::from("RELATIVE"), None, Kind::Unknown).err().unwrap(),
            &RepresentationError::IsRelativePath(PathBuf::from("RELATIVE"))
        )
    }
}
