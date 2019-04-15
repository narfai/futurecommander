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

extern crate futurecommander_vfs;

#[cfg_attr(tarpaulin, skip)]
mod hybrid_fs_integration {
    use std::{
        path:: { Path }
    };

    use futurecommander_vfs::{
        HybridFileSystem,
        operation::{
            Operation,
            CopyOperation,
            RemoveOperation
        },
        query::{
            Query,
            Entry,
            StatusQuery
        },
        representation::{
            VirtualState
        },
        Samples,
        Kind
    };

    #[test]
    pub fn no_dangling() {
        let mut fs = HybridFileSystem::default();
        _no_dangling(
            &mut fs,
            Samples::init_advanced_chroot("hybrid_no_dangling").as_path()
        );
    }

    /*
    dumb operations which should be resolved
    cp A APRIME
    rm A

    cp APRIME A
    rm APRIME
    */
    pub fn _no_dangling(fs: &mut HybridFileSystem, chroot: &Path) {
        let cp_a_aprime = CopyOperation::new(
            chroot.join("A").as_path(),
            chroot.join("APRIME").as_path()
        );

        fs.mut_transaction().add_operation(Box::new(cp_a_aprime.clone()));
        cp_a_aprime.execute(fs.mut_vfs()).unwrap();


        let rm_a = RemoveOperation::new(
            chroot.join("A").as_path()
        );

        fs.mut_transaction().add_operation(Box::new(rm_a.clone()));
        rm_a.execute(fs.mut_vfs()).unwrap();


        let cp_aprime_chroot = CopyOperation::new(
            chroot.join("APRIME").as_path(),
            chroot.join("A").as_path()
        );

        fs.mut_transaction().add_operation(Box::new(cp_aprime_chroot.clone()));
        cp_aprime_chroot.execute(fs.mut_vfs()).unwrap();


        let rm_aprime = RemoveOperation::new(
            chroot.join("APRIME").as_path()
        );

        fs.mut_transaction().add_operation(Box::new(rm_aprime.clone()));
        rm_aprime.execute(fs.mut_vfs()).unwrap();


        let stated_a = StatusQuery::new(chroot.join("A").as_path())
            .retrieve(fs.vfs())
            .unwrap()
            .into_inner();

        assert!(match stated_a.state() {
            VirtualState::Exists => true,
            _ => false
        });

        let virtual_identity = stated_a.into_existing_virtual().unwrap();

        assert_eq!(virtual_identity.as_identity(), chroot.join("A"));
        assert_eq!(virtual_identity.to_kind(), Kind::Directory);
        assert_eq!(virtual_identity.as_source().unwrap(), chroot.join("A"));

        let stated_aprime = StatusQuery::new(chroot.join("APRIME").as_path())
            .retrieve(fs.vfs())
            .unwrap();

        assert!(!stated_aprime.exists());
    }

    #[test]
    pub fn copy_file_dir_interversion() {
        let mut fs = HybridFileSystem::default();
        _copy_file_dir_interversion(
            &mut fs,
            Samples::init_advanced_chroot("hybrid_file_dir_interversion").as_path()
        );
    }


    pub fn _copy_file_dir_interversion(fs: &mut HybridFileSystem, chroot: &Path) {
        /*
        file dir interversion ( C <-> B )
        cp A/C .
        rm A/C

        cp C Z
        rm C

        cp B C <- C should be dir & still a file, may there is smth to update or not take data from fs but stat insteda
        rm B

        cp Z B
        rm Z
        */

        let cp_ac_chroot = CopyOperation::new(
            chroot.join("A/C").as_path(),
            chroot.join("C").as_path(),
        );

        fs.mut_transaction().add_operation(Box::new(cp_ac_chroot.clone()));
        cp_ac_chroot.execute(fs.mut_vfs()).unwrap();

        let rm_ac = RemoveOperation::new(
            chroot.join("A/C").as_path()
        );

        fs.mut_transaction().add_operation(Box::new(rm_ac.clone()));
        rm_ac.execute(fs.mut_vfs()).unwrap();


        let cp_c_z = CopyOperation::new(
            chroot.join("C").as_path(),
            chroot.join("Z").as_path()
        );

        fs.mut_transaction().add_operation(Box::new(cp_c_z.clone()));
        cp_c_z.execute(fs.mut_vfs()).unwrap();


        let rm_c = RemoveOperation::new(
            chroot.join("C").as_path()
        );

        fs.mut_transaction().add_operation(Box::new(rm_c.clone()));
        rm_c.execute(fs.mut_vfs()).unwrap();


        let cp_b_c = CopyOperation::new(
            chroot.join("B").as_path(),
            chroot.join("C").as_path()
        );

        fs.mut_transaction().add_operation(Box::new(cp_b_c.clone()));
        cp_b_c.execute(fs.mut_vfs()).unwrap();


        let rm_b = RemoveOperation::new(
            chroot.join("B").as_path()
        );

        fs.mut_transaction().add_operation(Box::new(rm_b.clone()));
        rm_b.execute(fs.mut_vfs()).unwrap();

        let cp_z_b = CopyOperation::new(
            chroot.join("Z").as_path(),
            chroot.join("B").as_path()
        );

        fs.mut_transaction().add_operation(Box::new(cp_z_b.clone()));
        cp_z_b.execute(fs.mut_vfs()).unwrap();


        let rm_z = RemoveOperation::new(
            chroot.join("Z").as_path()
        );

        fs.mut_transaction().add_operation(Box::new(rm_z.clone()));
        rm_z.execute(fs.mut_vfs()).unwrap();


        let stated_b = StatusQuery::new(chroot.join("B").as_path())
            .retrieve(fs.vfs())
            .unwrap();

        assert!(stated_b.exists());
        assert!(stated_b.is_file());
        assert_eq!(stated_b.as_inner().as_virtual().as_identity(), chroot.join("B"));
        assert_eq!(stated_b.as_inner().as_virtual().to_kind(), Kind::File);
        assert_eq!(stated_b.as_inner().as_virtual().as_source().unwrap(), chroot.join("A/C"));

        let stated_c = StatusQuery::new(chroot.join("C").as_path())
            .retrieve(fs.vfs())
            .unwrap()
            .into_inner()
            .into_existing_virtual()
            .unwrap();

        assert_eq!(stated_c.as_identity(), chroot.join("C"));
        assert_eq!(stated_c.to_kind(), Kind::Directory);
        assert_eq!(stated_c.as_source().unwrap(), chroot.join("B"));

        let stated_z = StatusQuery::new(chroot.join("Z").as_path())
            .retrieve(fs.vfs())
            .unwrap();

        assert!(!stated_z.exists());
    }

    /*
        nesting
        cp C A/
        cp A/C/D A/
        rm A/D/G //<- should no appear
    */
    pub fn _some_nesting(fs: &mut HybridFileSystem, chroot: &Path) {
        let cp_c_a = CopyOperation::new(
            chroot.join("C").as_path(),
            chroot.join("A").join("C").as_path()
        );

        fs.mut_transaction().add_operation(Box::new(cp_c_a.clone()));
        cp_c_a.execute(fs.mut_vfs()).unwrap();

        let cp_acd_a = CopyOperation::new(
            chroot.join("A/C/D").as_path(),
            chroot.join("A").join("D").as_path()
        );

        fs.mut_transaction().add_operation(Box::new(cp_acd_a.clone()));
        cp_acd_a.execute(fs.mut_vfs()).unwrap();

        let rm_adg = RemoveOperation::new(
            chroot.join("A/D/G").as_path()
        );

        fs.mut_transaction().add_operation(Box::new(rm_adg.clone()));
        rm_adg.execute(fs.mut_vfs()).unwrap();

        let stated_ad = StatusQuery::new(chroot.join("A/D").as_path())
            .retrieve(fs.vfs())
            .unwrap()
            .into_inner()
            .into_existing_virtual()
            .unwrap();

        assert_eq!(stated_ad.as_identity(), chroot.join("A/D"));
        assert_eq!(stated_ad.to_kind(), Kind::Directory);
        assert_eq!(stated_ad.as_source().unwrap(), chroot.join("B/D"));

        let stated_adg = StatusQuery::new(chroot.join("A/D/G").as_path())
            .retrieve(fs.vfs())
            .unwrap();

        assert!(!stated_adg.exists());
    }

    #[test]
    pub fn apply_a_vfs_to_real_fs() {
        let chroot = Samples::init_advanced_chroot("hybrid_apply_a_vfs_to_real_fs");
        let mut fs = HybridFileSystem::default();

        _no_dangling(&mut fs, chroot.as_path());
        _copy_file_dir_interversion(&mut fs, chroot.as_path());
        _some_nesting(&mut fs, chroot.as_path());

        fs.apply().unwrap();

        let c = chroot.join("C");
        assert!(c.exists());
        assert!(c.is_dir());

        let b = chroot.join("B");
        assert!(b.exists());
        assert!(b.is_file());

        let ac = chroot.join("A/C");
        assert!(ac.exists());
        assert!(ac.is_dir());

        let ad = chroot.join("A/D");
        assert!(ad.exists());
        assert!(ad.is_dir());

        let ad = chroot.join("A/D/G");
        assert!(!ad.exists());
    }
}
