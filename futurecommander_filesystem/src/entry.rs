mod adapter;
mod collection;
mod serializable;

pub use self::{
    adapter::EntryAdapter,
    collection::EntryCollection
};

use std::{
    path::{ Path, PathBuf },
    ffi::{ OsStr },
    cmp::{ Ordering }
};

pub trait Entry {
    fn path(&self) -> &Path;
    fn to_path(&self) -> PathBuf;
    fn name(&self) -> Option<&OsStr>;
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
    fn exists(&self) -> bool;
    fn is_virtual(&self) -> bool;
    fn is_contained_by(&self, other: &dyn Entry) -> bool {
        for ancestor in self.path().ancestors() {
            if other.path() == ancestor {
                return true;
            }
        }
        false
    }
}

impl Eq for dyn Entry {}

impl Ord for dyn Entry {
    fn cmp(&self, other: &dyn Entry) -> Ordering {
        self.path().cmp(other.path())
    }
}

impl PartialOrd for dyn Entry {
    fn partial_cmp(&self, other: &dyn Entry) -> Option<Ordering> {
        Some(self.path().cmp(other.path()))
    }
}

impl PartialEq for dyn Entry {
    fn eq(&self, other: &dyn Entry) -> bool {
        self.path().eq(other.path())
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod entry_tests {
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
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod collection_tests {
    use std::path::Path;
    use crate::EntryAdapter;
    use super::*;

    #[test]
    fn entry_collection_contains() {
        let mut c = EntryCollection::new();
        c.add(EntryAdapter(Path::new("A")));

        assert!(c.contains(&EntryAdapter(Path::new("A"))));
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn entry_collection_is_empty() {
        let c = EntryCollection::<EntryAdapter<&Path>>::new();

        assert!(c.is_empty());
        assert_eq!(c.len(), 0);
    }
}



