use std::path::PathBuf;
use std::fs::File;

use std::io::prelude::*;
use std::io::{ BufReader, BufWriter };

const READ_BUFFER_SIZE: usize = 8;
const WRITE_BUFFER_SIZE: usize = 8;

pub fn copy(src: &PathBuf, dst: &PathBuf, on_read: &Fn(usize)) -> Result<usize, String> {
    File::open(src)
        .map_err(|err| err.to_string())
        .and_then(|src_file| Ok(BufReader::with_capacity(READ_BUFFER_SIZE,src_file)))
        .and_then(|reader|
            File::create(dst)
                .map_err(|err| err.to_string())
                .and_then(|dst_file| Ok((reader, BufWriter::with_capacity(WRITE_BUFFER_SIZE,dst_file) ) ) )
        )
        .and_then(|(mut reader, mut writer)| {
            let mut read = 0;
            loop {
                match {
                    reader.fill_buf()
                        .map_err(|err| err.to_string())
                        .and_then(|buffer| {
                            writer.write(&buffer)
                                .map_err(|err| err.to_string())
                                .and(Ok(buffer.len()))
                        })
                } {
                    Ok(length) => {
                        if length == 0 {
                            break;
                        }
                        read += length;
                        on_read(read);
                        reader.consume(length);
                    }
                    Err(why) => return Err(why)
                }
            }
            writer.flush()
                .map_err(|err| err.to_string())
                .and(Ok(read))
        })
}
