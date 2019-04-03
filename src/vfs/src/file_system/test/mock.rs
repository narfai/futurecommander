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
use std::fs::{ File, create_dir, remove_dir_all, remove_file };
use std::io::Write;

use std::env::current_exe;
use std::path::{ PathBuf, Path };

pub struct VfsMock;

impl VfsMock {
    pub fn mock_file(chroot: &Path, path: &Path) {
        let mut file = File::create(chroot.join(path)).unwrap();
        file.write_all(b"Gummies candy biscuit jelly cheesecake. Liquorice gingerbread oat cake marzipan gummies muffin. Sweet liquorice dessert. Caramels chupa chups lollipop dragee gummies sesame snaps. Tootsie roll lollipop chocolate cake chocolate jelly jelly-o sesame snaps gummies. Topping topping bear claw candy canes bonbon muffin cupcake. Tart croissant liquorice croissant tootsie roll cupcake powder icing. Dessert souffle cake ice cream pie cookie. Brownie cotton candy pudding ice cream pudding cotton candy gingerbread gummi bears. Dragee biscuit croissant chocolate bar cheesecake marshmallow wafer macaroon. Sweet roll chupa chups gummi bears oat cake halvah marshmallow souffle pie. Jujubes pastry fruitcake macaroon jelly lemon drops chocolate cake chocolate cake."
        ).unwrap();
        assert!(chroot.join(path).exists());
    }

    pub fn init_real_samples_idempotently(arbitrary_identifier: &str) -> PathBuf {
        let chroot = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap()
            .join(Path::new("samples").join(format!("real_tests_{}", arbitrary_identifier)));

        if chroot.exists() {
            remove_dir_all(chroot.as_path()).unwrap();
        }

        create_dir(chroot.as_path()).unwrap();
        assert!(chroot.exists());

        create_dir(chroot.join("RDIR")).unwrap();
        assert!(chroot.join("RDIR").exists());

        Self::mock_file(chroot.as_path(), Path::new("RDIR").join("RFILEA").as_path());
        Self::mock_file(chroot.as_path(), Path::new("RDIR").join("RFILEB").as_path());

        chroot
    }

    pub fn init_real_to_virtual_samples_idempotently(arbitrary_identifier: &str) -> PathBuf {
        let chroot = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap()
            .join(Path::new("samples").join(format!("real_to_virtual_tests_{}", arbitrary_identifier)));

        if chroot.exists() {
            remove_dir_all(chroot.as_path()).unwrap();
        }

        create_dir(chroot.as_path()).unwrap();
        assert!(chroot.exists());

        Self::mock_file(chroot.as_path(), Path::new("F"));

        create_dir(chroot.join("A")).unwrap();
        assert!(chroot.join("A").exists());

        Self::mock_file(chroot.as_path(), Path::new("A").join("C").as_path());

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
