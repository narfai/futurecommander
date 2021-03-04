use crate::{
    Kind,
    Entry,
    ReadableFileSystem,
    DomainError,
    operation::{
        Strategist,
        create::CreateGenerator
    }
};

#[derive(Copy, Clone, Debug)]
pub enum CreateStrategy {
    FileCreation,
    DirectoryCreation,
    FileCreationOverwrite,
    DirectoryCreationOverwrite,
}

impl Strategist for CreateGenerator {
    type Strategy = CreateStrategy;
    fn strategize<F: ReadableFileSystem>(&self, fs: &F) -> Result<Self::Strategy, DomainError> {
        use CreateStrategy::*;
        let entry = fs.status(self.request.path())?;

        if entry.exists() {
            if entry.is_dir() {
                return Err(DomainError::DirectoryOverwriteNotAllowed(entry.to_path()))
            } else if !entry.is_file() {
                return Err(DomainError::CreateUnknown(entry.to_path()))
            }
        }

        match self.request.kind().into() {
            Kind::Directory => {
                if entry.exists() && entry.is_file() {
                    Ok(DirectoryCreationOverwrite)
                } else {
                    Ok(DirectoryCreation)
                }
            },
            Kind::File => {
                if entry.exists() && entry.is_file() {
                    Ok(FileCreationOverwrite)
                } else {
                    Ok(FileCreation)
                }
            },
            Kind::Unknown => {
                Err(DomainError::CreateUnknown(entry.to_path()))
            }
        }
    }
}