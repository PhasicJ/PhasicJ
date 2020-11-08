use ::std::io;
use ::std::fs;
use ::std::path;
use ::std::io::Write;

// TODO(dwtj): Make this portable.
macro_rules! svm_file_name {
    () => { "libsvm.so" };
}

pub fn svm() -> &'static [u8] {
    include_bytes!(concat!("../../../../", env!("SVM_EXEC_PATH")))
}

pub fn svm_sha1_hash() -> &'static str {
    include_str!(concat!("../../../../", env!("SVM_SHA1_EXEC_PATH")))
}

pub fn svm_default_temp_file_path() -> path::PathBuf {
    let mut path = std::env::temp_dir();
    // TODO(dwtj): Consider performing this reserve more rigorously. Then again,
    //  there is no unsafety from being sloppy here, just a very unimportant
    //  performance cost.
    path.reserve(80);
    path.push(format!(
        "phasicj/{svm}.temp/sha1/{sha1}/{svm}",
        sha1 = svm_sha1_hash(),
        svm = svm_file_name!(),
    ));
    return path;
}

pub fn write_svm_file_if_missing(file_path: &path::Path) -> Result<(), io::Error> {
    if !file_path.exists() {
        let mut parent_path = file_path.parent().expect("Parent directory doesn't exist.");
        fs::create_dir_all(parent_path)?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(svm())?;
        file.sync_all()?;
    }
    Ok(())
}
