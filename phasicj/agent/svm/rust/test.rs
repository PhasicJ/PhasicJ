#![cfg(test)]

use ::svm::Svm;
use ::phasicj_agent_svm_rust_embed::{
    svm_default_temp_file_path,
    write_svm_file_if_missing,
};

// [JVMS 15 §4.1]https://docs.oracle.com/javase/specs/jvms/se15/html/jvms-4.html#jvms-4.1
const JVM_CLASS_FILE_MAGIC_NUMBER: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];

#[test]
fn instrument_test_class() {
    write_svm_file_if_missing(&svm_default_temp_file_path()).expect("Failed to write SVM library to temp file path.");

    let mut isolate_thread = Svm::new().expect("Failed to create a new `Svm` instance.");
    let mut test_class = get_test_class();
    unsafe {
        let instrumented = isolate_thread.instrument(test_class.as_mut_slice()).expect("Failed to instrument a test class.");

        assert!(test_class.as_slice().len() < instrumented.size());
        assert_eq!(&JVM_CLASS_FILE_MAGIC_NUMBER, &instrumented.as_slice()[..4]);

        isolate_thread.free_svm_class(instrumented).expect("Failed to free an `SvmClass`.");
    }
}

fn get_test_class() -> Vec<u8> {
    // NOTE(dwtj): The `include_bytes!()` macro searches for a file relative to
    //  this source file. Thus, to find our desired file, we go up four
    //  directories to the Bazel execroot (i.e. the directory from which Bazel
    //  actions are executed). We then concatenate an environment variable's
    //  value to find the desired test class file.
    let test_class = include_bytes!(concat!("../../../../", env!("SVM_TEST_CLASS_EXEC_PATH")));
    let mut buffer = vec![0; test_class.len()];
    buffer.clone_from_slice(test_class);
    return buffer;
}
