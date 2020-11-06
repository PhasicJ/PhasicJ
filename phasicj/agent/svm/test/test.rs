#![cfg(test)]

use ::svm::raw::{
    svm_instr_instrument,
    svm_instr_free,
    graal_isolatethread_t,
    graal_tear_down_isolate,
};

use svm::create_graal_isolate_thread;

#[test]
fn instrument_test_class() {
    // NOTE(dwtj): We go up four directories because this macro searches
    //  for the file to include relative to this source file.
    let test_class = include_bytes!(concat!("../../../../", env!("TEST_CLASS_EXEC_PATH")));
    assert!(test_class.len() > 0);

    unsafe {
        let isolate_thread_ptr = create_graal_isolate_thread();

        // TODO(dwtj): Call `svm_instr_instrument()` with `test_class`.
        // TODO(dwtj): Check that the returned instrumented class has more bytes
        //  than the original class.
        // TODO(dwtj): Consider other ways to validate the instrumented bytes.

        let err = graal_tear_down_isolate(isolate_thread_ptr);
        assert!(err == 0);
    }
}
