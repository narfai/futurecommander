use crate::VirtualFileSystem;
use std::path::Path;

pub fn tree(vfs: &mut VirtualFileSystem, identity: &Path) {
    _tree(vfs, identity, None, false, true);
}

fn _tree(vfs: &mut VirtualFileSystem, identity: &Path, depth_list: Option<Vec<(bool,bool)>>, parent_first: bool, parent_last: bool) {
    vfs.read_virtual(identity);

    let file_name = match identity.file_name() {
        Some(file_name) => file_name.to_string_lossy().to_string(),
        None => "/".to_string()
    };


    if let Some(depth_list) = &depth_list {
        let mut depth_delimiter = "".to_string();
        for (first, last) in depth_list {
            if *last {
                depth_delimiter += "    ";
            } else {
                depth_delimiter += "│   ";
            }
        }
        println!(
            "{}{}── {}",
            depth_delimiter,
            match parent_last {
                false => "├",
                true => "└"
            },
            file_name
        );
    } else {
        println!("{}", file_name);
        println!("│");
    }

    match vfs.get_state().children(identity){
        Some(children) => {
            let new_depth_list = match depth_list {
                Some(depth_list) => {
                    let mut new = depth_list.clone();
                    new.push((parent_first, parent_last));
                    new
                },
                None => vec![]
            };

            let length = children.len();

            for (index, virtual_child) in children.iter().enumerate() {
                _tree(
                    vfs,
                    virtual_child.as_identity(),
                    Some(new_depth_list.clone()),
                    index == 0,
                    index == (length - 1)
                );
            }
        },
        None => {}
    };
}
