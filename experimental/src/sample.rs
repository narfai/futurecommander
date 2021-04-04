/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::{
    fs::{ File, create_dir, remove_dir_all },
    io::Write,
    env::current_exe,
    path::{ PathBuf, Path },
    time::{ SystemTime, UNIX_EPOCH }
};

pub fn sample_path() -> PathBuf {
    current_exe().unwrap()
        .parent().unwrap() //target
        .parent().unwrap() //debug
        .parent().unwrap() //deps
        .parent().unwrap() //project root
        .join("samples")
}

pub fn static_samples_path() -> PathBuf {
    let sample_path = sample_path().join("static");

    assert!(sample_path.join("A").exists());
    assert!(sample_path.join("B").exists());
    assert!(sample_path.join("F").exists());
    assert!(sample_path.join("B/D").exists());
    assert!(sample_path.join("B/D/E").exists());
    assert!(sample_path.join("B/D/G").exists());

    assert!(sample_path.join("A").is_dir());
    assert!(sample_path.join("B").is_dir());
    assert!(sample_path.join("F").is_file());
    assert!(sample_path.join("B/D").is_dir());
    assert!(sample_path.join("B/D/E").is_dir());
    assert!(sample_path.join("B/D/G").is_dir());
    sample_path
}

pub fn dynamic_samples_path() -> PathBuf {
    sample_path().join("dynamic")
}

pub fn create_sample_file(chroot: &Path, path: &Path) {
    let path = chroot.join(path);
    let mut file = File::create(path.as_path()).unwrap();
    file.write_all(
        format!(
            "{:?} Gummies candy biscuit jelly cheesecake. Liquorice gingerbread oat cake marzipan gummies muffin. Sweet liquorice dessert. Caramels chupa chups lollipop dragee gummies sesame snaps. Tootsie roll lollipop chocolate cake chocolate jelly jelly-o sesame snaps gummies. Topping topping bear claw candy canes bonbon muffin cupcake. Tart croissant liquorice croissant tootsie roll cupcake powder icing. Dessert souffle cake ice cream pie cookie. Brownie cotton candy pudding ice cream pudding cotton candy gingerbread gummi bears. Dragee biscuit croissant chocolate bar cheesecake marshmallow wafer macaroon. Sweet roll chupa chups gummi bears oat cake halvah marshmallow souffle pie. Jujubes pastry fruitcake macaroon jelly lemon drops chocolate cake chocolate cake.",
            path
        ).as_bytes()
    ).unwrap();
    assert!(chroot.join(path).exists());
}

pub struct Chroot {
    pub name: String,
    pub id: u32
}

impl Chroot {
    pub fn new(name: &str) -> Self {
        Chroot {
            name: name.to_owned(),
            id: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
        }
    }

    pub fn path(&self) -> PathBuf {
        dynamic_samples_path().join(format!("chroot_{}_{}", self.name, self.id))
    }

    pub fn init_empty(&self) -> PathBuf {
        let path = self.clean();

        create_dir(&path).unwrap();
        assert!(path.exists());

        path
    }

    pub fn create_file(&self, path: &str) -> PathBuf {
        create_sample_file(&self.path(), Path::new(path));
        self.path().join(path)
    }

    pub fn create_dir(&self, path: &str) -> PathBuf {
        let dir_path = self.path().join(path);
        create_dir(&dir_path).unwrap();
        assert!(dir_path.exists());
        dir_path
    }

    pub fn init_simple(&self) -> PathBuf {
        let chroot = self.init_empty();

        create_dir(chroot.join("RDIR")).unwrap();
        assert!(chroot.join("RDIR").exists());

        create_dir(chroot.join("RDIR2")).unwrap();
        assert!(chroot.join("RDIR2").exists());

        create_dir(chroot.join("RDIR3")).unwrap();
        assert!(chroot.join("RDIR3").exists());

        create_sample_file(chroot.as_path(), Path::new("RDIR").join("RFILEA").as_path());
        create_sample_file(chroot.as_path(), Path::new("RDIR").join("RFILEB").as_path());
        create_sample_file(chroot.as_path(), Path::new("RDIR2").join("RFILEA").as_path());
        create_sample_file(chroot.as_path(), Path::new("RDIR2").join("RFILEC").as_path());

        chroot
    }

    pub fn init_advanced_chroot(&self) -> PathBuf {
        let chroot = self.init_empty();

        create_sample_file(chroot.as_path(), Path::new("F"));

        create_dir(chroot.join("A")).unwrap();
        assert!(chroot.join("A").exists());

        create_sample_file(chroot.as_path(), Path::new("A").join("C").as_path());

        create_dir(chroot.join("B")).unwrap();
        assert!(chroot.join("B").exists());

        create_dir(chroot.join("B/D")).unwrap();
        assert!(chroot.join("B/D").exists());

        create_dir(chroot.join("B/D/E")).unwrap();
        assert!(chroot.join("B/D/E").exists());

        create_dir(chroot.join("B/D/G")).unwrap();
        assert!(chroot.join("B/D/G").exists());

        chroot
    }

    pub fn clean(&self) -> PathBuf {
        let path = self.path();

        if path.exists() {
            remove_dir_all(&path).unwrap();
        }

        path
    }
}