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
    cmp ::{ Ordering },
    ffi ::{ OsStr },
    path::{ Path, PathBuf },
};

#[derive(Debug)]
pub struct EntryAdapter<T>(pub T);
impl <T>EntryAdapter<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
    pub fn as_inner(&self) -> &T {
        &self.0
    }
}

pub trait Entry {
    fn path(&self) -> &Path;
    fn to_path(&self) -> PathBuf;
    fn name(&self) -> Option<&OsStr>;
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
    fn exists(&self) -> bool;
    fn is_virtual(&self) -> bool;
    fn is_contained_by(&self, other: &Entry) -> bool {
        for ancestor in self.path().ancestors() {
            if other.path() == ancestor {
                return true;
            }
        }
        false
    }
}

impl Eq for Entry {}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        self.path().cmp(other.path())
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.path().cmp(other.path()))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.path().eq(other.path())
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests_entry {
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
        assert_eq!(z.to_path(), z_path.clone());
        assert_eq!(z.path(), z_path.as_path());
        assert_eq!(z.name(), Some(OsStr::new("Z")));
        assert_eq!(z.as_inner(), &z_path.as_path());
        assert_eq!(z.into_inner(), z_path.as_path());
    }
}
