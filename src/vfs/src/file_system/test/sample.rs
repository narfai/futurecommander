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
use std::fs::{ File, create_dir, remove_dir_all };
use std::io::Write;

use std::env::current_exe;
use std::path::{ PathBuf, Path };

pub struct Samples;

impl Samples {
    pub fn sample_path() -> PathBuf {
        current_exe().unwrap()
            .parent().unwrap() //project root
            .parent().unwrap() //target
            .parent().unwrap() //debug
            .parent().unwrap() //deps
            .join("samples")
            .to_path_buf()
    }

    pub fn static_samples_path() -> PathBuf {
        let sample_path = Self::sample_path().join("static_samples");
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

    pub fn init_real_chroot(arbitrary_identifier: &str) -> PathBuf {
        let chroot = Self::sample_path().join(format!("real_chroot_{}", arbitrary_identifier));

        if chroot.exists() {
            remove_dir_all(chroot.as_path()).unwrap();
        }

        create_dir(chroot.as_path()).unwrap();
        assert!(chroot.exists());

        create_dir(chroot.join("RDIR")).unwrap();
        assert!(chroot.join("RDIR").exists());

        Self::create_sample_file(chroot.as_path(), Path::new("RDIR").join("RFILEA").as_path());
        Self::create_sample_file(chroot.as_path(), Path::new("RDIR").join("RFILEB").as_path());

        chroot
    }

    pub fn init_virtual_chroot(arbitrary_identifier: &str) -> PathBuf {
        let chroot = Self::sample_path().join(format!("virtual_chroot_{}", arbitrary_identifier));

        if chroot.exists() {
            remove_dir_all(chroot.as_path()).unwrap();
        }

        create_dir(chroot.as_path()).unwrap();
        assert!(chroot.exists());

        Self::create_sample_file(chroot.as_path(), Path::new("F"));

        create_dir(chroot.join("A")).unwrap();
        assert!(chroot.join("A").exists());

        Self::create_sample_file(chroot.as_path(), Path::new("A").join("C").as_path());

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

}
