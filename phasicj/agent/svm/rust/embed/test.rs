#![cfg(test)]

use ::phasicj_agent_svm_rust_embed::{
    svm,
    svm_sha1_hash,
    svm_default_temp_file_path,
    write_svm_file_if_missing,
};

use ::phasicj_agent_svm_rust::SvmIsolateThread;

#[test]
fn test_svm_is_non_empty() {
    assert!(svm().len() > 0);
}

#[test]
fn test_svm_sha1_hash_length() {
    assert!(svm_sha1_hash().len() == 40);
}

// [JVMS 15 §4.1]https://docs.oracle.com/javase/specs/jvms/se15/html/jvms-4.html#jvms-4.1
const JVM_CLASS_FILE_MAGIC_NUMBER: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];

#[test]
fn test_svm_instrumentation() {
    let svm_library_path = svm_default_temp_file_path();
    write_svm_file_if_missing(&svm_library_path).expect("Failed to write SVM library to temp file path.");
    let mut test_class = get_test_class();

    unsafe {
        let isolate_thread = SvmIsolateThread::new_from_library_path(&svm_library_path).expect("Failed to create a new `Svm` instance.");
        let instrumented = isolate_thread.svm_instr_instrument(test_class.as_mut_slice()).expect("Failed to instrument a test class.");

        assert!(test_class.as_slice().len() < instrumented.size());
        assert_eq!(&JVM_CLASS_FILE_MAGIC_NUMBER, &instrumented.as_slice()[..4]);

        isolate_thread.svm_instr_free(instrumented).expect("Failed to free an `SvmClass`.");
    }
}

fn get_test_class() -> Vec<u8> {
    // NOTE(dwtj): The `include_bytes!()` macro searches for a file relative to
    //  this source file. Thus, to find our desired file, we go up four
    //  directories to the Bazel execroot (i.e. the directory from which Bazel
    //  actions are executed). We then concatenate an environment variable's
    //  value to find the desired test class file.
    let test_class = include_bytes!(env!("SVM_TEST_CLASS_EXEC_PATH"));
    let mut buffer = vec![0; test_class.len()];
    buffer.clone_from_slice(test_class);
    return buffer;
}
