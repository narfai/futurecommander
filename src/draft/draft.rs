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
