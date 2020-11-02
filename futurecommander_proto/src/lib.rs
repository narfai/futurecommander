pub mod vfs;


#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        vfs::{ListDirectoryRequest}
    };

    #[test]
    fn dummy() {
        let request = ListDirectoryRequest {path: format!("hello")};

        assert_eq!(request.path, format!("hello"))
    }
}
