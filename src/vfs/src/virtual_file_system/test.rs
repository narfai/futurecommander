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
use crate::shared::sample::Samples;

use crate::*;
use crate::operation::*;
use crate::query::*;

#[cfg(test)]
mod virtual_file_system {
    use super::*;

    #[test]
    fn resolve() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::default();

        let b = sample_path.join("B");
        let ab = sample_path.join("A/B");
        let abcdef = sample_path.join("A/B/C/D/E/F");

        CopyOperation::new(
            b.as_path(),
            ab.as_path()
        ).execute(&mut vfs).unwrap();

        let virtual_state = vfs.virtual_state().unwrap();

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(ab.as_path()).unwrap().unwrap()
        );
        assert_eq!(
            b.join("C/D/E/F").as_path(),
            virtual_state.resolve(abcdef.as_path()).unwrap().unwrap()
        );
    }

    #[test]
    fn resolve_through() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::default();

        let b = sample_path.join("B");

        let ab = sample_path.join("A/B");
        let bd = sample_path.join("B/D");

        CopyOperation::new(
            b.as_path(),
            ab.as_path()
        ).execute(&mut vfs).unwrap();

        CopyOperation::new(
            ab.as_path(),
            bd.join("B").as_path()
        ).execute(&mut vfs).unwrap();

        let virtual_state = vfs.virtual_state().unwrap();

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(ab.as_path()).unwrap().unwrap()
        );

        assert_eq!(
            b.as_path(),
            virtual_state.resolve(bd.join("B").as_path()).unwrap().unwrap()
        );
    }

    #[test]
    fn stat_none_if_deleted() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::default();
        let a = sample_path.join("A");

        assert!(
            StatusQuery::new(a.as_path())
                .retrieve(&vfs)
                .unwrap()
                .exists()
        );

        RemoveOperation::new(a.as_path())
            .execute(&mut vfs).unwrap();


        assert!(
            ! StatusQuery::new(a.as_path())
                .retrieve(&vfs)
                .unwrap()
                .exists()
        );
    }

    #[test]
    fn stat_virtual() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::default();
        let z = sample_path.join("Z");

        CreateOperation::new(
            z.as_path(),
            Kind::Directory
        ).execute(&mut vfs).unwrap();

        let stated = StatusQuery::new(z.as_path())
            .retrieve(&vfs)
            .unwrap()
            .into_inner()
            .into_existing_virtual()
            .unwrap();

        assert_eq!(stated.to_kind(), Kind::Directory);
        assert_eq!(stated.as_identity(), z);
        assert!(stated.as_source().is_none())
    }

    #[test]
    fn stat_real() {
        let sample_path = Samples::static_samples_path();
        let vfs = VirtualFileSystem::default();
        let a = sample_path.join("A");

        let stated = StatusQuery::new(a.as_path())
            .retrieve(&vfs)
            .unwrap()
            .into_inner()
            .into_existing_virtual()
            .unwrap();

        assert_eq!(stated.to_kind(), Kind::Directory);
        assert_eq!(stated.as_identity(), a.as_path());
        assert_eq!(stated.as_source(), Some(a.as_path()))
    }

    #[test]
    fn stat_related() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::default();
        let abdg = sample_path.join("A/B/D/G");//Note : should exists in samples

        CopyOperation::new(
            sample_path.join("B").as_path(),
            sample_path.join("A/B").as_path()
        ).execute(&mut vfs).unwrap();

        let stated = StatusQuery::new(abdg.as_path())
            .retrieve(&vfs)
            .unwrap()
            .into_inner()
            .into_existing_virtual()
            .unwrap();

        assert_eq!(stated.to_kind(), Kind::Directory);
        assert_eq!(stated.as_identity(), abdg.as_path());
        assert_eq!(stated.as_source(), Some(sample_path.join("B/D/G").as_path()))
    }

    //Error testing
    #[test]
    fn copy_or_move_directory_into_itself_must_not_be_allowed() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::default();

        let source = sample_path.join("B");
        let destination = sample_path.join("B/D/B");

        match CopyOperation::new(
            source.as_path(),
            destination.as_path()
        ).execute(&mut vfs) {
            Err(VfsError::CopyIntoItSelf(err_source, err_destination)) => {
                assert_eq!(source.as_path(), err_source.as_path());
                assert_eq!(destination.as_path(), err_destination.as_path());
            }
            Err(error) => panic!("{}", error),
            Ok(_) => panic!("Should not be able to copy into itself")
        };
    }

    // No-Backwards tests
    #[test]
    fn reset_empty() {
        let sample_path = Samples::static_samples_path();
        let mut vfs = VirtualFileSystem::default();

        CreateOperation::new(
            sample_path.join("VIRTUALA").as_path(),
            Kind::File
        ).execute(&mut vfs).unwrap();

        CreateOperation::new(
            sample_path.join("VIRTUALB").as_path(),
            Kind::Directory
        ).execute(&mut vfs).unwrap();

        RemoveOperation::new(
            sample_path.join("A").as_path()
        ).execute(&mut vfs).unwrap();

        assert!(vfs.has_addition());
        assert!(vfs.has_subtraction());

        vfs.reset();

        assert!(!vfs.has_addition());
        assert!(!vfs.has_subtraction());
    }
}
