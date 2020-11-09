use ::std::io;
use ::std::fs;
use ::std::path;
use ::std::io::Write;

pub fn rt_jar() -> &'static [u8] {
    include_bytes!(concat!("../../../../", env!("RT_JAR_EXEC_PATH")))
}

pub fn rt_jar_sha1_hash() -> &'static str {
    include_str!(concat!("../../../../", env!("RT_JAR_SHA1_EXEC_PATH")))
}

pub fn rt_jar_default_temp_file_path() -> path::PathBuf {
    let mut path = std::env::temp_dir();
    // TODO(dwtj): Consider performing this reserve more rigorously. Then again,
    //  there is no unsafety from being sloppy here, just a very unimportant
    //  performance cost.
    path.reserve(80);
    path.push("phasicj/rt.jar.temp/sha1");
    path.push(rt_jar_sha1_hash());
    path.push("rt.jar");
    return path;
}

pub fn write_rt_jar_file_if_missing(file_path: &path::Path) -> Result<(), io::Error> {
    if !file_path.exists() {
        let mut parent_path = file_path.parent().expect("Parent directory doesn't exist.");
        fs::create_dir_all(parent_path)?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(rt_jar())?;
        file.sync_all()?;
    }
    Ok(())
}
