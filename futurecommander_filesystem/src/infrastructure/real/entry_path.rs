// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{
    path::{ Path, PathBuf },
    ffi::{ OsStr }
};
use crate::{
    Entry,
    EntryAdapter
};

impl Entry for EntryAdapter<&Path> {
    fn path(&self) -> &Path {
        self.0
    }

    fn to_path(&self) -> PathBuf {
        self.0.to_path_buf()
    }

    fn name(&self) -> Option<&OsStr> {
        self.0.file_name()
    }

    fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    fn is_file(&self) -> bool {
        self.0.is_file()
    }

    fn exists(&self) -> bool { self.0.exists() }

    fn is_virtual(&self) -> bool { false }
}

impl Entry for EntryAdapter<PathBuf> {
    fn path(&self) -> &Path { self.0.as_path() }

    fn to_path(&self) -> PathBuf { self.0.clone() }

    fn name(&self) -> Option<&OsStr> { self.0.file_name() }

    fn is_dir(&self) -> bool { self.0.is_dir() }

    fn is_file(&self) -> bool { self.0.is_file() }

    fn exists(&self) -> bool { self.0.exists() }

    fn is_virtual(&self) -> bool { false }
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{ sample::Samples };

    #[test]
    fn entry_adapter_path_directory() {
        let static_samples = Samples::static_samples_path();

        let a_path = static_samples.join("A");
        let a = EntryAdapter(a_path.as_path());
        assert!(a.exists());
        assert!(a.is_dir());
        assert!(!a.is_file());
        assert!(!a.is_virtual());
        assert_eq!(a.to_path(), a_path.clone());
        assert_eq!(a.path(), a_path.as_path());
        assert_eq!(a.name(), Some(OsStr::new("A")));
        assert_eq!(a.as_inner(), &a_path.as_path());
        assert_eq!(a.into_inner(), a_path.as_path());
    }


    #[test]
    fn entry_adapter_path_file() {
        let static_samples = Samples::static_samples_path();

        let f_path = static_samples.join("F");
        let f = EntryAdapter(f_path.as_path());
        assert!(f.exists());
        assert!(!f.is_dir());
        assert!(f.is_file());
        assert!(!f.is_virtual());
        assert_eq!(f.to_path(), f_path.clone());
        assert_eq!(f.path(), f_path.as_path());
        assert_eq!(f.name(), Some(OsStr::new("F")));
        assert_eq!(f.as_inner(), &f_path.as_path());
        assert_eq!(f.into_inner(), f_path.as_path());
    }


    #[test]
    fn entry_adapter_path_not_exists() {
        let static_samples = Samples::static_samples_path();

        let z_path = static_samples.join("Z");
        let z = EntryAdapter(z_path.as_path());
        assert!(!z.exists());
        assert!(!z.is_dir());
        assert!(!z.is_file());
        assert!(!z.is_virtual());
        assert_eq!(z.to_path(), z_path.clone());
        assert_eq!(z.path(), z_path.as_path());
        assert_eq!(z.name(), Some(OsStr::new("Z")));
        assert_eq!(z.as_inner(), &z_path.as_path());
        assert_eq!(z.into_inner(), z_path.as_path());
    }

    #[test]
    fn entry_adapter_pathbuf_directory() {
        let static_samples = Samples::static_samples_path();

        let a_path = static_samples.join("A");
        let a = EntryAdapter(a_path.clone());
        assert!(a.exists());
        assert!(a.is_dir());
        assert!(!a.is_file());
        assert!(!a.is_virtual());
        assert_eq!(a.to_path(), a_path);
        assert_eq!(a.path(), a_path.as_path());
        assert_eq!(a.name(), Some(OsStr::new("A")));
        assert_eq!(a.as_inner(), &a_path.as_path());
        assert_eq!(a.into_inner(), a_path.as_path());
    }


    #[test]
    fn entry_adapter_pathbuf_file() {
        let static_samples = Samples::static_samples_path();

        let f_path = static_samples.join("F");
        let f = EntryAdapter(f_path.clone());
        assert!(f.exists());
        assert!(!f.is_dir());
        assert!(f.is_file());
        assert!(!f.is_virtual());
        assert_eq!(f.to_path(), f_path);
        assert_eq!(f.path(), f_path.as_path());
        assert_eq!(f.name(), Some(OsStr::new("F")));
        assert_eq!(f.as_inner(), &f_path.as_path());
        assert_eq!(f.into_inner(), f_path.as_path());
    }


    #[test]
    fn entry_adapter_pathbuf_not_exists() {
        let static_samples = Samples::static_samples_path();

        let z_path = static_samples.join("Z");
        let z = EntryAdapter(z_path.clone());
        assert!(!z.exists());
        assert!(!z.is_dir());
        assert!(!z.is_file());
        assert!(!z.is_virtual());
        assert_eq!(z.to_path(), z_path);
        assert_eq!(z.path(), z_path.as_path());
        assert_eq!(z.name(), Some(OsStr::new("Z")));
        assert_eq!(z.as_inner(), &z_path.as_path());
        assert_eq!(z.into_inner(), z_path.as_path());
    }
}
