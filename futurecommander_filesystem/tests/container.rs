// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

extern crate futurecommander_filesystem;

#[cfg(not(tarpaulin_include))]
mod container_integration {
    use std::{
        path:: { Path }
    };

    use futurecommander_representation::{
        VirtualState
    };

    use futurecommander_filesystem::{
        Entry,
        Capabilities,
        Capability,
        Container,
        sample::Samples,
        Kind,
        PresetGuard,
        ZealousGuard,
        ReadableFileSystem
    };

    #[test]
    pub fn no_dangling() {
        let mut fs = Container::new();
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
    pub fn _no_dangling(fs: &mut Container, chroot: &Path) {
        fs.copy(
            &chroot.join("A"),
            &chroot.join("APRIME"),
            &mut PresetGuard::new(ZealousGuard, Capabilities::default() + Capability::Merge)
        ).unwrap();

        fs.remove(
            &chroot.join("A"),
            &mut PresetGuard::new(ZealousGuard, Capabilities::default() + Capability::Recursive)
        ).unwrap();

        fs.copy(
            &chroot.join("APRIME"),
            &chroot.join("A"),
            &mut PresetGuard::new(ZealousGuard, Capabilities::default() + Capability::Merge)
        ).unwrap();

        fs.remove(
            &chroot.join("APRIME"),
            &mut PresetGuard::new(ZealousGuard, Capabilities::default() + Capability::Recursive)
        ).unwrap();

        let stated_a = fs.status(chroot.join("A").as_path())
            .unwrap()
            .into_inner();

        assert!(matches!(stated_a.state(), VirtualState::Exists));

        let virtual_identity = stated_a.into_existing_virtual().unwrap();

        assert_eq!(virtual_identity.as_identity(), chroot.join("A"));
        assert_eq!(virtual_identity.to_kind(), Kind::Directory);
        assert_eq!(virtual_identity.as_source().unwrap(), chroot.join("A"));

        let stated_aprime = fs.status(chroot.join("APRIME").as_path()).unwrap();

        assert!(!stated_aprime.exists());
    }

    #[test]
    pub fn copy_file_dir_interversion() {
        let mut fs = Container::new();
        _copy_file_dir_interversion(
            &mut fs,
            Samples::init_advanced_chroot("hybrid_file_dir_interversion").as_path()
        );
    }


    pub fn _copy_file_dir_interversion(fs: &mut Container, chroot: &Path) {
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

        fs.copy(
            &chroot.join("A/C"),
            &chroot.join("C"),
            &mut PresetGuard::new(ZealousGuard, Capabilities::default() + Capability::Merge)
        ).unwrap();

        fs.remove(
            &chroot.join("A/C"),
            &mut PresetGuard::new(ZealousGuard, Capabilities::default() + Capability::Recursive)
        ).unwrap();

        fs.copy(
            &chroot.join("C"),
            &chroot.join("Z"),
            &mut PresetGuard::new(ZealousGuard, Capabilities::default() + Capability::Merge)
        ).unwrap();

        fs.remove(
            &chroot.join("C"),
            &mut PresetGuard::new(ZealousGuard, Capabilities::default() + Capability::Recursive)
        ).unwrap();

        fs.copy(
            &chroot.join("B"),
            &chroot.join("C"),
            &mut PresetGuard::new(ZealousGuard, Capabilities::default() + Capability::Merge)
        ).unwrap();

        fs.remove(
            &chroot.join("B"),
            &mut PresetGuard::new(ZealousGuard, Capabilities::default() + Capability::Recursive)
        ).unwrap();

        fs.copy(
            &chroot.join("Z"),
            &chroot.join("B"),
            &mut PresetGuard::new(ZealousGuard, Capabilities::default() + Capability::Merge)
        ).unwrap();

        fs.remove(
            &chroot.join("Z"),
            &mut PresetGuard::new(ZealousGuard, Capabilities::default() + Capability::Recursive)
        ).unwrap();

        let stated_b = fs.status(chroot.join("B").as_path())
            .unwrap();

        assert!(stated_b.exists());
        assert!(stated_b.is_file());
        assert_eq!(stated_b.as_inner().as_virtual().as_identity(), chroot.join("B"));
        assert_eq!(stated_b.as_inner().as_virtual().to_kind(), Kind::File);
        assert_eq!(stated_b.as_inner().as_virtual().as_source().unwrap(), chroot.join("A/C"));

        let stated_c = fs.status(chroot.join("C").as_path())
            .unwrap()
            .into_inner()
            .into_existing_virtual()
            .unwrap();

        assert_eq!(stated_c.as_identity(), chroot.join("C"));
        assert_eq!(stated_c.to_kind(), Kind::Directory);
        assert_eq!(stated_c.as_source().unwrap(), chroot.join("B"));

        let stated_z = fs.status(chroot.join("Z").as_path()).unwrap();

        assert!(!stated_z.exists());
    }

    /*
        nesting
        cp C A/
        cp A/C/D A/
        rm A/D/G //<- should no appear
    */
    pub fn _some_nesting(fs: &mut Container, chroot: &Path) {
        fs.copy(
            &chroot.join("C"),
            &chroot.join("A/C"),
            &mut PresetGuard::new(ZealousGuard, Capabilities::default())
        ).unwrap();

        fs.copy(
            &chroot.join("A/C/D"),
            &chroot.join("A/D"),
            &mut PresetGuard::new(ZealousGuard, Capabilities::default())
        ).unwrap();

        fs.remove(
            &chroot.join("A/D/G"),
            &mut PresetGuard::new(ZealousGuard, Capabilities::default() + Capability::Recursive)
        ).unwrap();

        let stated_ad = fs.status(chroot.join("A/D").as_path())
            .unwrap()
            .into_inner()
            .into_existing_virtual()
            .unwrap();

        assert_eq!(stated_ad.as_identity(), chroot.join("A/D"));
        assert_eq!(stated_ad.to_kind(), Kind::Directory);
        assert_eq!(stated_ad.as_source().unwrap(), chroot.join("B/D"));

        let stated_adg = fs.status(chroot.join("A/D/G").as_path())
            .unwrap();

        assert!(!stated_adg.exists());
    }

    // TODO
    #[test]
    pub fn apply_a_vfs_to_real_fs() {
        let chroot = Samples::init_advanced_chroot("hybrid_apply_a_vfs_to_real_fs");
        let mut fs = Container::new();

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

    /* TODO TEST
        cd samples/static
        mv A B/
        apply
        ERROR : error : Command error : container operation error: Source /home/narfai/current2/code_own_source/futurecommander/samples/static/A does not exists
        TODO : exists => exist
    */
}
