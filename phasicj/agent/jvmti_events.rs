use ::std::convert::TryInto;
use ::std::os::raw;
use ::std::mem;
use ::std::ptr;
use ::std::mem::MaybeUninit;
use ::jvmti::jvmtiEventCallbacks;
use ::jvmti::{
    jvmtiEnv,
    JNIEnv,
    jint,
    jclass,
    jobject,
};
use ::std::ffi;
use crate::jvmti_env;
use ::svm::{
    graal_isolatethread_t,
};

pub fn get_initial_agent_callbacks(env: &mut jvmtiEnv) -> jvmtiEventCallbacks {
    return jvmtiEventCallbacks {
        VMDeath: Some(vm_death),
        ClassFileLoadHook: Some(class_file_load_hook),
        ..Default::default()
    };
}

// https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#VMDeath
#[no_mangle]
unsafe extern "C" fn vm_death(_env: *mut jvmtiEnv, _jni_env: *mut JNIEnv) {
    println!("`jvmti_event_callbacks::vm_death()`, implemented in Rust.");
}

// https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#ClassFileLoadHook
#[no_mangle]
unsafe extern "C" fn class_file_load_hook(
        jvmti_env: *mut jvmtiEnv,
        _jni_env: *mut JNIEnv,
        _class_being_redefined: jclass,
        _loader: jobject,
        name: *const raw::c_char,
        _protection_domain: jobject,
        class_data_len: jint,
        class_data: *const raw::c_uchar,
        new_class_data_len: *mut jint,
        new_class_data: *mut *mut raw::c_uchar) {

    // TODO(dwtj): The given `name` is *modified* UTF-8, but this conversion
    //  expects standard UTF-8. Thus, this conversion will fail for some
    //  class names.
    let class_name = ffi::CStr::from_ptr(name).to_str().ok().unwrap();

    // TODO(dwtj): Remove this once we can instrument the standard library
    //  without
    if class_name.starts_with("sun/")
    || class_name.starts_with("jdk/")
    || class_name.starts_with("java/") {
        return;
    }

    // TODO(dwtj): Creating a new Graal Isolate for each instrumentation seems
    //  wasteful. Consider reusing them. Either have one global one with all
    //  threads attached or have one isolate per thread.

    // First create a Graal Isolate for our call to the SVM.
    let create_isolate_params = ptr::null_mut();
    let isolate_ptr = ptr::null_mut();
    let mut isolate_thread_ptr: MaybeUninit<*mut graal_isolatethread_t> = MaybeUninit::uninit();
    let err = ::svm::graal_create_isolate(
        create_isolate_params,
        isolate_ptr,
        isolate_thread_ptr.as_mut_ptr(),
    );
    if err != 0 {
        panic!("Failed to create Graal isolate during PhasicJ Agent `ClassFileLoadHook` event.");
    }
    let isolate_thread_ptr = isolate_thread_ptr.assume_init();

    // NOTE(dwtj): During our call to `instr_instrument()` SVM allocates memory
    //  for the `svmBuf` using `UnmanagedMemory.malloc()`. We are responsible
    //  for freeing it by calling `instr_free()`.
    let inBufSize: raw::c_int = class_data_len.try_into().unwrap();
    let inBuf: *mut raw::c_char = mem::transmute(class_data);

    // TODO(dwtj): Could/should I make this simpler by just initializing these
    //  to 0 and `ptr::null()`, respectively?
    let mut svmBufSize: MaybeUninit<raw::c_int> = MaybeUninit::uninit();
    let mut svmBuf: MaybeUninit<*mut raw::c_char> = MaybeUninit::uninit();

    let err = ::svm::svm_instr_instrument(
        isolate_thread_ptr,
        inBufSize,
        inBuf,
        svmBufSize.as_mut_ptr(),
        svmBuf.as_mut_ptr(),
    );
    if err != 0 {
        panic!("Class instrumentation by PhasicJ agent failed for an unknown reason: {}", class_name);
    }

    let svmBufSize: jint = svmBufSize.assume_init();

    // NOTE(dwtj): According to [this callback's documentation][1], this JVMTI
    //  callback's out-pointer, `new_class_data`, needs to be allocated via the
    //  JVMTI interface. To quote the spec,
    //
    //      The agent must allocate the space for the modified class file data
    //      buffer using the memory allocation function `Allocate` because the
    //      VM is responsible for freeing the new class file data buffer using
    //      `Deallocate`.
    //
    //  [1]: https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#ClassFileLoadHook

    // Allocate some JVMTI-managed memory that can be returned via
    // the `new_class_data` pointer.
    let mut jvmtiBuf: MaybeUninit<*mut raw::c_uchar> = MaybeUninit::uninit();
    jvmti_env::allocate(&mut *jvmti_env, svmBufSize.into(), jvmtiBuf.as_mut_ptr());

    // Copy binary data from the SVM-managed memory into our newly allocated
    // JVMTI-managed memory. (To please the Rust compiler, we use transmute to
    // cast these bytes from `c_uchar` to `c_char` and back to `c_uchar`.)
    let svmBuf: *mut raw::c_uchar = mem::transmute(svmBuf.assume_init());
    let jvmtiBuf: *mut raw::c_uchar = jvmtiBuf.assume_init();
    ptr::copy_nonoverlapping(svmBuf, jvmtiBuf, svmBufSize.try_into().unwrap());

    // Free the SVM-allocated memory that we got from our call to
    // `svm_instr_instrument()`.
    let svmBuf: *mut raw::c_char = mem::transmute(svmBuf);
    ::svm::svm_instr_free(isolate_thread_ptr, svmBuf);

    // Tear down the Graal isolate which we created just to handle this event.
    let err = ::svm::graal_tear_down_isolate(isolate_thread_ptr);
    if err != 0 {
        panic!("During PhasicJ class instrumentation, failed to tear down a Graal isolate for an unknown reason.");
    }

    // Return the SVM-instrumented class data which we copied into a JVMTI-
    // allocated buffer via out-pointers.
    *new_class_data_len = svmBufSize;
    *new_class_data = jvmtiBuf;
}
