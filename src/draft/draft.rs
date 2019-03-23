//Generic dummy wrapper of iterator

pub struct DummyWrapperIterator<I> {
    iter: I
}

impl <I>DummyWrapperIterator<I> {
    pub fn new(iter: I) -> DummyWrapperIterator<I> {
        DummyWrapperIterator {
            iter
        }
    }
}

impl <I>Iterator for DummyWrapperIterator<I> where I: Iterator {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        match self.iter.next() {
            None => None,
            Some(v) => Some(v)
        }
    }
}
//
//#[derive(Debug)]
//pub struct LsIterator<'a> {
//    iter: HashSetIter<'a, VirtualPath>,
//    state: VirtualDelta
//}

//
//impl <'a>LsIterator<'a> {
//    pub fn new(state: VirtualDelta, iter: HashSetIter<'a, VirtualPath>) -> LsIterator<'a> {
//        LsIterator {
//            iter,
//            state
//        }
//    }
//}
//
//impl <'a>Iterator for LsIterator<'a> {
//    type Item = LsItem;
//
//    fn next(&mut self) -> Option<LsItem> {
//        match self.iter.next() {
//            None => None,
//            Some(child) => Some(LsItem::from(&child, self.state.is_directory(child.as_identity())))
//        }
//    }
//}
//
//#[derive(Eq, Hash)]
//pub struct ChildItem {
//    path: PathBuf,
//    is_directory: Option<bool>
//}
//
//impl ChildItem {
//    fn from(path: PathBuf, is_directory: Option<bool>) -> ChildItem {
//        ChildItem{
//            path,
//            is_directory
//        }
//    }
//
//    fn into_known_directory(self, is_directory: bool) -> ChildItem {
//        Self::from(self.path, Some(is_directory))
//    }
//}
//
//
//impl PartialEq for ChildItem {
//    fn eq(&self, other: &ChildItem) -> bool {
//        self.path.eq(&other.path) && self.is_directory.eq(&other.is_directory)
//    }
//}
//
//
//pub struct ChildrenIterator<'a> {
//    vfs: &'a mut VirtualFileSystem,
//    parent: PathBuf,
//    iter: Option<HashSetIter<'a, VirtualPath>>
//
//}

//impl <'a> ChildrenIterator <'a> {
//    pub fn new(vfs: &'a mut VirtualFileSystem, parent: &Path) -> ChildrenIterator<'a> {
//        ChildrenIterator {
//            vfs,
//            parent: parent.to_path_buf(),
//            iter: None
//        }
//    }
//    pub fn single(vfs: &'b mut vfs, identity: &'a VirtualPath) -> ChildrenIterator {
//        let tmp_set : HashSet<VirtualPath> = [identity.clone()].iter().cloned().collect();
//        ChildrenIterator {
//            iter: tmp_set.iter()
//        }
//    }
//}
//
//impl <'a>Iterator for ChildrenIterator<'a> {
//    type Item = ChildItem;
//
//    fn next(&mut self) -> Option<ChildItem> {
//        if self.iter.is_none() {
//            self.vfs.read_virtual(self.parent.as_path());
//            self.iter = match self.vfs.get_state().children(self.parent.as_path()) {
//                Some(children) => Some(children.iter()),
//                None => None
//            }
//        }
//
//        match self.iter.unwrap().next() {
//            None => None,
//            Some(child) => Some(ChildItem::from(child.to_identity(), None))
//        }
//    }
//}


//#[test]
//pub fn test_iterators() {
//    let mut base_it : HashSet<&VirtualPath> = HashSet::new();
//    let test1 = VirtualPath::from_str("/test1");
//    let test2 = VirtualPath::from_str("/test2");
//    let test3 = VirtualPath::from_str("/test3");
//    let test4 = VirtualPath::from_str("/test4");
//    base_it.insert(&test1);
//    base_it.insert(&test2);
//    base_it.insert(&test3);
//    base_it.insert(&test4);
//
////    for test in LsIterator::new(base_it.iter()) {
//////        println!("{:?}", test);
////    }
//}
//
//#[test]
//pub fn test_iterators_2() {
//    let sample_path = current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("examples");
//    let mut vfs = VirtualFileSystem::new();
//    vfs.read_virtual(sample_path.as_path());
//    vfs.read_virtual(sample_path.join(&Path::new("A")).as_path());
//    vfs.read_virtual(sample_path.join(&Path::new("B")).as_path());
//    vfs.read_virtual(sample_path.join(&Path::new("B/D")).as_path());
//    vfs.read_virtual(sample_path.join(&Path::new("B/D/E")).as_path());
//    vfs.read_virtual(sample_path.join(&Path::new("B/D/G")).as_path());
//
//    let state = vfs.get_state();
//    match state.children(sample_path.join(&Path::new("B")).as_path()) {
//        Some(children) => {
//            for test in LsIterator::new(state.clone(),children.iter()) {
//                println!("{:#?}", test);
//            }
//        },
//        None => panic!("None")
//    }
//}



//    pub fn ls_iter(&mut self, identity: &Path) -> Option<ChildrenIterator> {
//        self.read_virtual(identity);
//
//        let state = self.get_state();
//
//        match state.is_directory(identity) {
//            true => Some(ChildrenIterator::new(self, identity)),
//            false => match identity.parent() {
//                Some(parent) => {
//                    self.read_virtual(parent);
//                    match self.get_state().get(identity) {
//                        Some(child) => Some(ChildrenIterator::new(self, child.as_identity())),
//                        None => None
//                    }
//                },
//                None => None
//            }
//        }
//    }
//pub fn get(identity: &Path) {
//    /*
//        Exists
//            in state
//            in add
//            in sub
//        Is directory
//            Is root
//                Is empty
//            and is in real fs
//                Is empty
//            and is virtually
//                Is empty
//        Is file
//            in real fs
//            virtually
//
//        TODO recursive method to attach directories from bottom to top till it match some existing dir in state
//        TODO / IDEA be able to slice a delta into a subtree
//
//
//        Cache the virtual_state
//        VIRTUALIZATION
//        1. maintain real tree
//
//        2. maintain virtual tree
//
//    */
//}


//
//pub fn exp_read_dir(&self, path: &Path) -> Result<VirtualChildren, VfsError> {
//    let virtual_state = self.get_virtual_state();
//    match self.exists_virtually(path) {
//        true => match virtual_state.is_directory(path) {
//            Some(true) => match virtual_state.get(path) {
//                Some(virtual_identity) =>
//                    match virtual_identity.as_source() {
//                        Some(source_path) =>
//                            match VirtualChildren::from_file_system(
//                                source_path,
//                                Some(virtual_identity.as_identity())
//                            ) {
//                                Ok(virtual_children) => Ok(
//                                    &(&virtual_children - &self.sub.children(path).unwrap())
//                                        + &self.add.children(path).unwrap()
//                                ),
//                                Err(error) => Err(VfsError::from(error))
//                            },
//                        None => Err(VfsError::HasNoSource(path.to_path_buf()))
//                    },
//                None => Err(VfsError::VirtuallyDoesNotExists(path.to_path_buf()))
//            },
//            Some(false) => Err(VfsError::IsNotADirectory(path.to_path_buf())),
//            None => Err(VfsError::VirtuallyDoesNotExists(path.to_path_buf()))
//        },
//        false => {
//            let mut real_children = match VirtualChildren::from_file_system(path, None) {
//                Ok(virtual_children) => virtual_children,
//                Err(error) => return Err(VfsError::from(error))
//            };
//
//            if let Some(to_del_children) = self.sub.children(path) {
//                println!("TO DEL CHILDREN {:?}", to_del_children);
//                real_children = &real_children - &to_del_children;
//            }
//
//            if let Some(to_add_children) = self.add.children(path) {
//                println!("TO ADD CHILDREN {:?}", to_add_children);
//                real_children = &real_children + &to_add_children;
//            }
//
//            Ok(real_children)
//        }
//    }
//}
//
//pub fn copy(&mut self, source: &Path, destination: &Path) -> Result<VirtualPath, VfsError>{
//    let virtual_state = self.get_virtual_state();
//    let referent_source = match virtual_state.get(source) {
//        Some(source_identity) => source_identity.as_referent_source(),
//        None => match source.exists() {
//            true => source,
//            false => return Err(VfsError::DoesNotExists(source.to_path_buf()))
//        }
//    };
//
//    if !self.exists(source) {
//        return Err(VfsError::DoesNotExists(source.to_path_buf()))
//    }
//
//    let kind = match self.is_directory_virtually(destination) {
//        Some(true) => VirtualKind::Directory,
//        Some(false) => return Err(VfsError::IsNotADirectory(destination.to_path_buf())),
//        None => match source.exists() {
//            true => match source.is_dir() {
//                true => VirtualKind::Directory,
//                false => return Err(VfsError::IsNotADirectory(destination.to_path_buf()))
//            }
//            false => return Err(VfsError::DoesNotExists(destination.to_path_buf()))
//        }
//    };
//
//    let new_identity = &VirtualPath::from_path(source)
//        .with_new_parent(destination)
//        .with_source(Some(referent_source))
//        .with_kind(kind);
//
//    if self.exists(new_identity.as_identity()) {
//        return Err(VfsError::AlreadyExists(new_identity.to_identity()))
//    }
//
//    self.add.attach_virtual(new_identity);
//
//    if self.sub.exists(new_identity.as_identity()) {
//        self.sub.detach(new_identity.as_identity())
//    }
//
//    Ok(new_identity.clone())
//}
//
//pub fn remove(&mut self, path: &Path) -> Result<VirtualPath, VfsError> {
//    let identity = match self.add.get(path) {
//        Some(identity) => {
//            let cloned = identity.clone();
//            self.add.detach(cloned.as_identity());
//            cloned
//        },
//        None => match path.exists() {
//            true => VirtualPath::from_path(path).with_kind(match path.is_dir() {
//                true => VirtualKind::Directory,
//                false => VirtualKind::File
//            }),
//            false => return Err(VfsError::DoesNotExists(path.to_path_buf()))
//        }
//    };
//
//    return match self.sub.get(path) {
//        Some(_) => Err(VfsError::DoesNotExists(path.to_path_buf())),
//        None => {
//            self.sub.attach_virtual(&identity);
//            Ok(identity.clone())
//        }
//    }
//}
//
//
//
//pub fn old_copy(&mut self, source: &Path, destination: &Path ) -> Result<VirtualPath, VfsError>{
//    let virtual_source = match self.get(source) {
//        Ok(virtual_source) => virtual_source,
//        Err(error) => return Err(error)
//    };
//
//    match self.get(destination) {
//        Ok(virtual_destination) =>
//            match virtual_destination.to_kind() {
//                VirtualKind::Directory => {},
//                _ => return Err(VfsError::IsNotADirectory(virtual_destination.to_identity()))
//            },
//        Err(error) => return Err(error)
//    }
//
//    let new_identity = &VirtualPath::from_path(source)
//        .with_new_parent(destination)
//        .with_source(Some(virtual_source.as_referent_source()))
//        .with_kind(virtual_source.to_kind());
//
//
//
//    if self.exists(new_identity.as_identity()) {
//        return Err(VfsError::AlreadyExists(new_identity.to_identity()))
//    }
//
//    self.add.attach_virtual(new_identity);
//
//    if self.sub.exists(new_identity.as_identity()) {
//        self.sub.detach(new_identity.as_identity())
//    }
//
//    Ok(new_identity.clone())
//}
//
//pub fn old_remove(&mut self, path: &Path) -> Result<VirtualPath, VfsError> {
//    let identity = match self.add.get(path) {
//        Some(identity) => {
//            let cloned = identity.clone();
//            self.add.detach(cloned.as_identity());
//            cloned
//        },
//        None => match self.get(path) {
//            Ok(virtual_path) => virtual_path,
//            Err(error) => return Err(error)
//        }
//    };
//
//    return match self.sub.get(path) {
//        Some(_) => Err(VfsError::DoesNotExists(path.to_path_buf())),
//        None => {
//            self.sub.attach_virtual(&identity);
//            Ok(identity.clone())
//        }
//    }
//}
//
//pub fn old_mkdir(&mut self, path: &Path) -> Result<(), VfsError>{
//    match self.exists(path) {
//        true => Err(VfsError::AlreadyExists(path.to_path_buf())),
//        false => {
//            self.add.attach(path, None, true);
//            Ok(())
//        }
//    }
//}
//
//pub fn old_touch(&mut self, path: &Path) -> Result<(), VfsError>{
//    match self.exists(path) {
//        true => Err(VfsError::AlreadyExists(path.to_path_buf())),
//        false => {
//            self.add.attach(path, None, false);
//            Ok(())
//        }
//    }
//}
//
//pub fn old_mv(&mut self, source: &Path, destination: &Path) -> Result<VirtualPath, VfsError>{
//    let result = self.copy(source, destination);
//    match self.remove(source) {
//        Ok(_) => result,
//        Err(error) => Err(error)
//    }
//}
//
//pub fn old_get(&self, path: &Path) -> Result<VirtualPath, VfsError> {
//    let state = self.get_virtual_state();
//    match state.first_virtual_ancestor(path) {
//        Some(_ancestor) =>
//            match state.get(path) {
//                Some(virtual_identity) => Ok(virtual_identity.clone()),
//                None => {
//                    let resolved = state.resolve(path);
//                    Ok(VirtualPath::from_path(path)
//                        .with_kind(match resolved.is_dir() {
//                            true => VirtualKind::Directory,
//                            false => VirtualKind::File
//                        })
//                        .with_source(Some(resolved.as_path()))
//                    )
//                }
//            },
//        None =>
//            match path.exists() {
//                true => Ok(VirtualPath::from_path(path)
//                    .with_kind(
//                        match path.is_dir() {
//                            true => VirtualKind::Directory,
//                            false => VirtualKind::File
//                        }
//                    )),
//                false => Err(VfsError::DoesNotExists(path.to_path_buf()))
//            }
//    }
//}
