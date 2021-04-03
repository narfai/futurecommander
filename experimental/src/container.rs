mod operation;
mod read_filesystem;
mod write_filesystem;

use crate::{ Preview };

#[derive(Default)]
pub struct Container {
    operation_list: Vec<operation::Operation>,
    preview: Preview
}