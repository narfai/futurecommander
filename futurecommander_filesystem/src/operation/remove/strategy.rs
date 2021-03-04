use crate::{
    Entry,
    ReadableFileSystem,
    DomainError,
    operation::{
        Strategist,
        remove::RemoveGenerator
    }
};

#[derive(Copy, Clone, Debug)]
pub enum RemoveStrategy {
    FileRemoval,
    EmptyDirectoryRemoval,
    RecursiveDirectoryRemoval
}

impl <E: Entry>Strategist for RemoveGenerator<'_, E> {
    type Strategy = RemoveStrategy;
    fn strategize<F: ReadableFileSystem>(&self, fs: &F) -> Result<Self::Strategy, DomainError> {
        use RemoveStrategy::*;
        let entry = fs.status(self.request.path())?;

        if !entry.exists() {
            return Err(DomainError::DoesNotExists(self.request.path().to_path_buf()))
        }

        if entry.is_file() {
            Ok(FileRemoval)
        } else if entry.is_dir() {
            if fs.is_directory_empty(entry.path())? {
                Ok(EmptyDirectoryRemoval)
            } else {
                Ok(RecursiveDirectoryRemoval)
            }
        } else {
            return Err(DomainError::Custom(String::from("Unknown node type")))
        }
    }
}