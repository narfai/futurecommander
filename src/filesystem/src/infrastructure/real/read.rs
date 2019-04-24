
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
    type Result = EntryAdapter<PathBuf>;
    //Read real specialization
    fn read_dir(&self, path: &Path) -> Result<EntryCollection<Self::Result>,QueryError> {
        if ! path.exists() {
            return Err(QueryError::ReadTargetDoesNotExists(path.to_path_buf()))
        }

        if ! path.is_dir() {
            return Err(QueryError::IsNotADirectory(path.to_path_buf()))
        }

        let mut entry_collection = EntryCollection::new();

        match path.read_dir() {
            Ok(results) => {
                for result in results {
                    match result {
                        Ok(result) => entry_collection.add(EntryAdapter(result.path())),
                        Err(error) => return Err(QueryError::from(error))
                    };
                }
                Ok(entry_collection)
            },
            Err(error) => Err(QueryError::from(error))
        }
    }

    fn status(&self, path: &Path) -> Result<Self::Result, QueryError> {
        Ok(EntryAdapter(path.to_path_buf()))
    }
}
