use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::io::prelude::*;
use std::env;
use phasicj_agent_conf::PjAgentConf;

pub fn print_verbose_startup_info(conf: &PjAgentConf) {
    log::info!("PjAgent: startup: current directory: {}", env::current_dir().unwrap().display());
    log::info!("PjAgent: startup: configuration: {:?}", conf);
}

pub fn dump_class_to_file(prefix: &Path, class_name: &str, class_data: &[u8]) -> io::Result<()> {
    log::info!("Dumping class `{}` to file", class_name);
    let mut path_buf = PathBuf::new();
    path_buf.push(prefix);
    path_buf.push(format!("{}.class", class_name));
    fs::create_dir_all((&path_buf).parent().unwrap())?;
    let mut file = fs::File::create(&path_buf)?;
    file.write_all(class_data)?;
    return Ok(());
}
