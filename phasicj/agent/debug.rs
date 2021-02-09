use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::io::prelude::*;
use std::env;

pub fn dump_class_to_file(prefix: &Path, class_name: &str, class_data: &[u8]) -> io::Result<()> {
    let mut path_buf = PathBuf::new();
    path_buf.push(prefix);
    path_buf.push(format!("{}.class", class_name));
    fs::create_dir_all((&path_buf).parent().unwrap());
    let mut file = fs::File::create(&path_buf)?;
    file.write_all(class_data)?;
    return Ok(());
}
