use ::std::convert::TryInto;
use ::std::os::raw;
use ::std::mem;
use ::std::ptr;
use ::std::slice;
use ::std::mem::MaybeUninit;
use ::jvmti::jvmtiEventCallbacks;
use ::jvmti::{
    jvmtiEnv,
    JNIEnv,
    jint,
    jclass,
    jobject,
    jthread,
};
use ::std::ffi;
use crate::jvmti_env;
use crate::jni_env;
use ::svm::Svm;

pub fn get_initial_agent_callbacks() -> jvmtiEventCallbacks {
    return jvmtiEventCallbacks {
        VMStart: Some(pj_vm_start),
        VMInit: Some(pj_vm_init),
        VMDeath: Some(pj_vm_death),
        ClassFileLoadHook: Some(pj_class_file_load_hook),
        ..Default::default()
    };
}

fn enable_instrumentation(env: *mut JNIEnv) {
    let class_name = ffi::CString::new("java/lang/Object").unwrap();
    let method_name = ffi::CString::new("phasicj$agent$rt$enableInstrumentation").unwrap();
    let method_signature = ffi::CString::new("()V").unwrap();
    unsafe {
        let java_lang_object_class = jni_env::find_class(env, class_name.as_ptr());
        let enable_instr_method = jni_env::get_static_method_id(
            env,
            java_lang_object_class,
            method_name.as_ptr(),
            method_signature.as_ptr(),
        );
        jni_env::call_static_void_method(env, java_lang_object_class, enable_instr_method);
    }
}

// https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#VMStart
#[no_mangle]
unsafe extern "C" fn pj_vm_start(_env: *mut jvmtiEnv, jni_env: *mut JNIEnv) {
    enable_instrumentation(jni_env);
}

// https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#VMDeath
#[no_mangle]
unsafe extern "C" fn pj_vm_init(_env: *mut jvmtiEnv, _jni_env: *mut JNIEnv, _: jthread) {
    // Nothing needed here yet.
}

// https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#VMDeath
#[no_mangle]
unsafe extern "C" fn pj_vm_death(_env: *mut jvmtiEnv, _jni_env: *mut JNIEnv) {
    // Nothing needed here yet.
}

// https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#ClassFileLoadHook
#[no_mangle]
unsafe extern "C" fn pj_class_file_load_hook(
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

    // Don't instrument our runtime classes.
    if class_name.starts_with("phasicj/agent/rt/") {
        return;
    }

    // TODO(dwtj): Creating a new Graal Isolate for each instrumentation seems
    //  wasteful. Consider reusing them. Either have one global one with all
    //  threads attached or have one isolate per thread.

    let mut svm = Svm::new().expect("Failed to create a new `Svm` instance.");

    // NOTE(dwtj): During our call to `instrument()` SVM allocates memory. This
    //  memory is wrapped by our `SvmClass`. We are reponsible for manually
    //  freeing this memory by calling `Svm::free_svm_class()`.
    // TODO(dwtj): Consider implementing a safer design.

    let class: &mut [u8] = slice::from_raw_parts_mut(mem::transmute(class_data), class_data_len.try_into().unwrap());
    let instrumented_class = svm.instrument(class).expect("Failed to instrument a class.");

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
    let mut jvmti_buf: MaybeUninit<*mut raw::c_uchar> = MaybeUninit::uninit();
    let buf_size: usize = instrumented_class.size();
    jvmti_env::allocate(
        &mut *jvmti_env,
        buf_size.try_into().unwrap(),
        jvmti_buf.as_mut_ptr()
    );
    let jvmti_buf: *mut raw::c_uchar = jvmti_buf.assume_init();

    // Copy binary data from the SVM-managed memory into our newly allocated
    // JVMTI-managed memory. (To please the Rust compiler, we use transmute to
    // cast these bytes to `c_uchar`.)
    {
        let svm_class_buf: *const raw::c_uchar = mem::transmute(instrumented_class.as_slice().as_ptr());
        ptr::copy_nonoverlapping(
            svm_class_buf,
            jvmti_buf,
            instrumented_class.size()
        );
    }

    svm.free_svm_class(instrumented_class).expect("Failed to free an `SvmClass`.");

    // Return the SVM-instrumented class data which we copied into a JVMTI-
    // allocated buffer via out-pointers.
    *new_class_data_len = buf_size.try_into().unwrap();
    *new_class_data = jvmti_buf;
}
