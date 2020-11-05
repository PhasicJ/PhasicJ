pub use ::svm_raw::{
    graal_isolatethread_t,
    svm_instr_instrument,
    svm_instr_free,
    graal_create_isolate,
    graal_tear_down_isolate,
};

use ::std::ptr;

use ::std::mem::MaybeUninit;

pub unsafe fn create_graal_isolate_thread() -> *mut graal_isolatethread_t{
    let create_isolate_params = ptr::null_mut();
    let isolate_ptr = ptr::null_mut();
    let mut isolate_thread_ptr: MaybeUninit<*mut graal_isolatethread_t> = MaybeUninit::uninit();
    let err = ::svm_raw::graal_create_isolate(
        create_isolate_params,
        isolate_ptr,
        isolate_thread_ptr.as_mut_ptr(),
    );
    if err != 0 {
        panic!("Failed to create Graal isolate.");
    }
    return isolate_thread_ptr.assume_init();
}
