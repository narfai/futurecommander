
use std::{
    path::{ Path, PathBuf }
};

use crate::{
    errors::{
        QueryError
    },
    port::{
        ReadableFileSystem,
        FileSystemAdapter,
        EntryAdapter,
        EntryCollection
    },
    infrastructure::real::{
        RealFileSystem
    }
};
impl ReadableFileSystem for FileSystemAdapter<RealFileSystem> {
    type Item = EntryAdapter<PathBuf>;

    //Read real specialization
    fn read_dir(&self, path: &Path) -> Result<EntryCollection<Self::Item>,QueryError> {
        if ! path.exists() {
            return Err(QueryError::ReadTargetDoesNotExists(path.to_path_buf()))
        }

        if ! path.is_dir() {
            return Err(QueryError::IsNotADirectory(path.to_path_buf()))
        }

        let mut entry_collection = EntryCollection::new();

        for result in path.read_dir()? {
            entry_collection.add(EntryAdapter(result?.path()));
        }

        Ok(entry_collection)
    }

    fn status(&self, path: &Path) -> Result<Self::Item, QueryError> {
        Ok(EntryAdapter(path.to_path_buf()))
    }

    fn is_directory_empty(&self, path: &Path) -> Result<bool, QueryError> {
        Ok(path.is_dir() && path.read_dir()?.next().is_none())
    }
}
