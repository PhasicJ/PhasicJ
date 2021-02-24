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
use crate::jvm::jvmti_env;
use crate::jvm::jni_env;
use crate::jvm::env_storage::EnvStorage;
use ::std::path::Path;
use ::phasicj_agent_conf::PjAgentConf;

use phasicj_agent_instr_rust::Instrumenter;

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
unsafe extern "C" fn pj_vm_death(env: *mut jvmtiEnv, _jni_env: *mut JNIEnv) {
    EnvStorage::free(&mut *env);
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

    let env_storage = EnvStorage::of(&mut *jvmti_env).expect(
        "Failed to get JVMTI environment-local storage."
    );
    let conf = &env_storage.conf;

    // TODO(dwtj): The given `name` is *modified* UTF-8, but this conversion
    //  expects standard UTF-8. Thus, this conversion will fail for some
    //  class names.
    let class_name = ffi::CStr::from_ptr(name).to_str().ok().unwrap();

    // Don't instrument our runtime classes.
    if class_name.starts_with("phasicj/agent/rt/") {
        return;
    }

    let class: &mut [u8] = slice::from_raw_parts_mut(
        mem::transmute(class_data),
        class_data_len.try_into().unwrap()
    );

    if conf.debug_dump_classes_before_instr {
        crate::debug::dump_class_to_file(Path::new("classes/before_instr"), class_name, class).unwrap();
    }
    let instr_result = match &conf.phasicj_exec {
        None => Instrumenter::using_system_path().instrument(class),
        Some(path) => Instrumenter::wrap(&path).instrument(class),
    };
    if instr_result.is_err() {
        // If our attempt to get an instrumented copy of the class data failed,
        // then we just don't instrument it, log this fact, and return early.
        // TODO(dwtj): Use a unified logging strategy.
        // TODO(dwtj): Log more information.
        eprintln!("Failed to instrument a class: {}", class_name);
        return;
    }
    let instrumented_class = instr_result.unwrap();

    if conf.debug_dump_classes_after_instr {
        crate::debug::dump_class_to_file(Path::new("classes/after_instr"), class_name, &instrumented_class).unwrap();
    }

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
    let buf_size: usize = instrumented_class.len();
    jvmti_env::allocate(
        &mut *jvmti_env,
        buf_size.try_into().unwrap(),
        jvmti_buf.as_mut_ptr()
    );
    let jvmti_buf: *mut raw::c_uchar = jvmti_buf.assume_init();

    ptr::copy_nonoverlapping(instrumented_class.as_ptr(), jvmti_buf, instrumented_class.len());

    *new_class_data_len = buf_size.try_into().unwrap();
    *new_class_data = jvmti_buf;
}
