#![cfg(test)]

use ::std::path::Path;

use ::phasicj_agent_svm_rust as svm;

const SVM_LIBRARY_FILE_PATH: &'static str = env!("SVM_LIBRARY_FILE_PATH");

const SVM_TEST_CLASS_DATA: &'static [u8] = include_bytes!(env!("SVM_TEST_CLASS_FILE_PATH"));

// [JVMS 15 §4.1]https://docs.oracle.com/javase/specs/jvms/se15/html/jvms-4.html#jvms-4.1
const JVM_CLASS_FILE_MAGIC_NUMBER: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];

fn starts_with_magic_number(data: &[u8]) -> bool {
    JVM_CLASS_FILE_MAGIC_NUMBER == data[..4]
}

fn svm_test_class_data() -> Vec<u8> {
    let mut vec = vec!(0; SVM_TEST_CLASS_DATA.len());
    vec[..].copy_from_slice(SVM_TEST_CLASS_DATA);
    return vec;
}

#[test]
pub fn test() {
    let svm_library_path = Path::new(SVM_LIBRARY_FILE_PATH);
    let svm_library = unsafe { svm::SvmIsolateThread::new_from_library_path(svm_library_path).unwrap() };

    // Use our Graal isolate to instrument our test data.
    let mut in_buf = svm_test_class_data();
    assert!(starts_with_magic_number(&in_buf));
    let out_buf = unsafe { svm_library.svm_instr_instrument(&mut in_buf).unwrap() };
    assert!(starts_with_magic_number( unsafe { out_buf.as_slice() } ));

    // Free the SVM-allocated memory.
    unsafe { svm_library.svm_instr_free(out_buf).unwrap() }
}
