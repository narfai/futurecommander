pub mod node;
pub mod kind;

mod read_filesystem;
mod write_filesystem;
mod internal;

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
pub mod tests;

#[derive(Default)]
pub struct Preview {
    root: node::Node
}