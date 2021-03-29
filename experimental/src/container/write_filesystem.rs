use std::path::Path;

use crate::{
    Result,
    WriteFileSystem
};

use super::{
    Container,
    operation::Operation
};


impl WriteFileSystem for Container {
    fn create_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.preview.create_file(&path)?;
        self.operation_list.push(Operation::CreateFile(path.as_ref().to_path_buf()));
        Ok(())
    }

    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.preview.create_dir(&path)?;
        self.operation_list.push(Operation::CreateDir(path.as_ref().to_path_buf()));
        Ok(())
    }

    fn create_dir_all<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.preview.create_dir_all(&path)?;
        self.operation_list.push(Operation::CreateDirAll(path.as_ref().to_path_buf()));
        Ok(())
    }

    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<u64> {
        self.preview.copy(&from, &to)?;
        self.operation_list.push(Operation::Copy(from.as_ref().to_path_buf(), to.as_ref().to_path_buf()));
        Ok(0)
    }

    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<()> {
        self.preview.rename(&from, &to)?;
        self.operation_list.push(Operation::Rename(from.as_ref().to_path_buf(), to.as_ref().to_path_buf()));
        Ok(())
    }

    fn remove_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.preview.remove_dir(&path)?;
        self.operation_list.push(Operation::RemoveDir(path.as_ref().to_path_buf()));
        Ok(())
    }

    fn remove_dir_all<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.preview.remove_dir_all(&path)?;
        self.operation_list.push(Operation::RemoveDirAll(path.as_ref().to_path_buf()));
        Ok(())
    }

    fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.preview.remove_file(&path)?;
        self.operation_list.push(Operation::RemoveFile(path.as_ref().to_path_buf()));
        Ok(())
    }
}