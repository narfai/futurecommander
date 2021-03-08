#[derive(Debug, Default)]
pub struct RealFileSystem {
    pub (in crate::infrastructure) read_buffer_size: usize,
    pub (in crate::infrastructure) write_buffer_size: usize
}

impl RealFileSystem {
    pub fn default() -> RealFileSystem {
        RealFileSystem {
            read_buffer_size: 10_485_760, //10 Mo,
            write_buffer_size: 2_097_152 //2 Mo
        }
    }
}