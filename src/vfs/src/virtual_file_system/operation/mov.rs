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

/* I EXPECT THE PATH destination TO EXISTS WITH SOURCE source */
use crate::{ VirtualFileSystem, VfsError };
use crate::operation::{Operation, MoveOperation, CopyOperation, RemoveOperation };

impl Operation<VirtualFileSystem> for MoveOperation {
    fn execute(&self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        CopyOperation::new(
            self.source(),
            self.destination(),
            self.merge(),
            self.overwrite()
        ).execute(fs)?;

        RemoveOperation::new(self.source(), true).execute(fs)
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        Samples,
        Kind
    };

    #[test]
    fn virtual_move_operation_directory(){
        let samples_path = Samples::static_samples_path();
        let mut fs = VirtualFileSystem::default();

        let operation = MoveOperation::new(
            samples_path.join("A").as_path(),
            samples_path.join("Z").as_path(),
            false,
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(!fs.virtual_state().unwrap().is_virtual(samples_path.join("A").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_virtual(samples_path.join("Z").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_directory(samples_path.join("Z").as_path()).unwrap());
    }

    #[test]
    fn virtual_move_operation_directory_merge(){
        let samples_path = Samples::static_samples_path();
        let mut fs = VirtualFileSystem::default();

        //Avoid need of override because of .gitkeep file present in both directory
        let gitkeep = samples_path.join("B/.gitkeep");
        fs.mut_sub_state().attach(gitkeep.as_path(),Some(gitkeep.as_path()), Kind::File).unwrap();

        let operation = MoveOperation::new(
            samples_path.join("B").as_path(),
            samples_path.join("A").as_path(),
            true,
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(!fs.virtual_state().unwrap().is_virtual(samples_path.join("B").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_virtual(samples_path.join("A/D").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_directory(samples_path.join("A/D").as_path()).unwrap());
    }

    #[test]
    fn virtual_move_operation_file(){
        let samples_path = Samples::static_samples_path();
        let mut fs = VirtualFileSystem::default();

        let operation = MoveOperation::new(
            samples_path.join("F").as_path(),
            samples_path.join("Z").as_path(),
            false,
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(!fs.virtual_state().unwrap().is_virtual(samples_path.join("F").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_virtual(samples_path.join("Z").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_file(samples_path.join("Z").as_path()).unwrap());
    }

    #[test]
    fn virtual_move_operation_file_overwrite(){
        let samples_path = Samples::static_samples_path();
        let mut fs = VirtualFileSystem::default();

        let operation = MoveOperation::new(
            samples_path.join("F").as_path(),
            samples_path.join("A/C").as_path(),
            false,
            true
        );

        operation.execute(&mut fs).unwrap();

        assert!(!fs.virtual_state().unwrap().is_virtual(samples_path.join("F").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_virtual(samples_path.join("A/C").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_file(samples_path.join("A/C").as_path()).unwrap());
    }
}
