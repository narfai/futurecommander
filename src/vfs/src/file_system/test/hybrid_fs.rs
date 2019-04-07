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

use std::path::{ Path };
use std::ffi::{ OsString };

use crate::*;
use crate::file_system::test::sample::Samples;

#[cfg(test)]
mod tests {
    use super::*;


//    #[test]
    pub fn no_dangling() {
        let mut fs = HybridFileSystem::new();
        _no_dangling(
            &mut fs,
            Samples::init_advanced_chroot("no_dangling").as_path()
        );
    }

    /*
    dumb operations which should be resolved
    cp A APRIME
    rm A

    cp APRIME A
    rm APRIME
    */
    pub fn _no_dangling(mut fs: &mut HybridFileSystem, chroot: &Path) {
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
            .unwrap();

        assert!(match stated_a {
            IdentityStatus::Exists(_) => true,
            _ => false
        });

        let virtual_identity = stated_a.as_virtual_identity().unwrap();

        assert_eq!(virtual_identity.as_identity(), chroot.join("A"));
        assert_eq!(virtual_identity.to_kind(), VirtualKind::Directory);
        assert_eq!(virtual_identity.as_source().unwrap(), chroot.join("A"));

        let stated_aprime = StatusQuery::new(chroot.join("APRIME").as_path())
            .retrieve(fs.vfs())
            .unwrap();

        assert!(!stated_aprime.exists());
    }

//    #[test]
    pub fn file_dir_interversion() {
        let mut fs = HybridFileSystem::new();
        _file_dir_interversion(
            &mut fs,
            Samples::init_advanced_chroot("file_dir_interversion").as_path()
        );
    }


    pub fn _file_dir_interversion(mut fs: &mut HybridFileSystem, chroot: &Path) {
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
            .unwrap()
            .into_virtual_identity()
            .unwrap();

        assert_eq!(stated_b.as_identity(), chroot.join("B"));
        assert_eq!(stated_b.to_kind(), VirtualKind::File);
        assert_eq!(stated_b.as_source().unwrap(), chroot.join("A/C"));

        let stated_c = StatusQuery::new(chroot.join("C").as_path())
            .retrieve(fs.vfs())
            .unwrap()
            .into_virtual_identity()
            .unwrap();

        assert_eq!(stated_c.as_identity(), chroot.join("C"));
        assert_eq!(stated_c.to_kind(), VirtualKind::Directory);
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
    pub fn _some_nesting(mut fs: &mut HybridFileSystem, chroot: &Path) {
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
            .into_virtual_identity()
            .unwrap();

        assert_eq!(stated_ad.as_identity(), chroot.join("A/D"));
        assert_eq!(stated_ad.to_kind(), VirtualKind::Directory);
        assert_eq!(stated_ad.as_source().unwrap(), chroot.join("B/D"));

        let stated_adg = StatusQuery::new(chroot.join("A/D/G").as_path())
            .retrieve(fs.vfs())
            .unwrap();

        assert!(!stated_adg.exists());
    }

    pub fn _directory_to_directory(mut fs: &mut HybridFileSystem, chroot: &Path) {
        let cp_c_a = CopyOperation::new(
            chroot.join("B").as_path(),
            chroot.join("A/B").as_path()
        );

        fs.mut_transaction().add_operation(Box::new(cp_c_a.clone()));
        cp_c_a.execute(fs.mut_vfs()).unwrap();
        assert!(chroot.join("A").as_path().is_dir());
    }

//    #[test]
    pub fn apply_a_vfs_to_real_fs() {
        let chroot = Samples::init_advanced_chroot("apply_a_vfs_to_real_fs");
        let mut fs = HybridFileSystem::new();


        _no_dangling(&mut fs, chroot.as_path());
        _file_dir_interversion(&mut fs, chroot.as_path());
        _some_nesting(&mut fs, chroot.as_path());
//        _directory_to_directory(&mut fs, chroot.as_path());
        println!("{:#?}", fs);
        fs.apply().unwrap();
        //@TODO Better to / into real_fs API / and maybe virtual ?



//        let mut apply: ApplyOperation<Box<WriteOperation<RealFileSystem>>> = ApplyOperation::from_virtual_filesystem(&vfs).unwrap();
//        println!("VFS {:#?} {:#?}", vfs, apply);
//        apply.execute(&mut real_fs).unwrap();
//        println!("REAL VERSION : {}", RealVersion::get());
//        println!("{:?}", apply)
    }
}
