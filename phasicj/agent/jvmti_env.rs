use ::std::mem::MaybeUninit;
use ::std::ffi;
use ::std::ptr;
use ::std::mem;
use ::std::os::raw;
use ::jvmti::{
    jvmtiPhase,
    JavaVM,
    jvmtiEnv,
    jvmtiError,
    jvmtiCapabilities,
    jthread,
    jvmtiEvent,
    jvmtiEventMode,
    JVMTI_VERSION_11,
    JNI_OK,
    jint,
    jlong,
    jvmtiError_JVMTI_ERROR_NONE as JVMTI_ERROR_NONE,
    jvmtiEventCallbacks,
    jvmtiPhase_JVMTI_PHASE_ONLOAD as JVMTI_PHASE_ONLOAD,
    jvmtiPhase_JVMTI_PHASE_PRIMORDIAL as JVMTI_PHASE_PRIMORDIAL,
    jvmtiPhase_JVMTI_PHASE_START as JVMTI_PHASE_START,
    jvmtiPhase_JVMTI_PHASE_LIVE as JVMTI_PHASE_LIVE,
    jvmtiPhase_JVMTI_PHASE_DEAD as JVMTI_PHASE_DEAD,
};
use std::convert::TryInto;

// [JNI#GetEnv](https://docs.oracle.com/en/java/javase/15/docs/specs/jni/invocation.html#getenv)
pub fn get_env(jvm: &mut JavaVM) -> *mut jvmtiEnv {
    let version = JVMTI_VERSION_11.try_into().unwrap();
    unsafe {
        let f = (**jvm).GetEnv.unwrap();
        let mut env: *mut ffi::c_void = ptr::null_mut();
        jni_check(f(jvm, &mut env, version));
        return mem::transmute(env);
    }
}

pub fn get_phase(env: &mut jvmtiEnv) -> jvmtiPhase {
    let mut phase: MaybeUninit<jvmtiPhase> = MaybeUninit::uninit();
    unsafe {
        let f = (**env).GetPhase.unwrap();
        jvmti_check(f(env, phase.as_mut_ptr()));
        return phase.assume_init();
    }
}

pub fn get_phase_str(env: &mut jvmtiEnv) -> &'static str {
    match get_phase(env) {
        JVMTI_PHASE_ONLOAD => "OnLoad",
        JVMTI_PHASE_PRIMORDIAL => "Primordial",
        JVMTI_PHASE_LIVE => "Live",
        JVMTI_PHASE_START => "Start",
        JVMTI_PHASE_DEAD => "Dead",
        _ => panic!("Unfamiliar JVMTI phase.")
    }
}

pub unsafe fn allocate(env: &mut jvmtiEnv, size: jlong, mem_ptr: *mut *mut raw::c_uchar) {
    let f = (**env).Allocate.unwrap();
    jvmti_check(f(env, size, mem_ptr));
}

// [JVMTI#GetPotentialCapabilities](https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#GetPotentialCapabilities)
pub fn get_potential_capabilities(env: &mut jvmtiEnv) -> jvmtiCapabilities {
    let mut capa: MaybeUninit<jvmtiCapabilities> = MaybeUninit::uninit();
    unsafe {
        let f = (**env).GetPotentialCapabilities.unwrap();
        jvmti_check(f(env, capa.as_mut_ptr()));
        return capa.assume_init();
    }
}

// [JVMTI#GetCapabilities](https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#GetCapabilities)
pub fn get_capabilities(env: &mut jvmtiEnv) -> jvmtiCapabilities {
    let mut capa = MaybeUninit::uninit();
    unsafe {
        let f = (**env).GetCapabilities.unwrap();
        jvmti_check(f(env, capa.as_mut_ptr()));
        return capa.assume_init();
    }
}

// [JVMTI#AddCapabilities](https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#AddCapabilities)
pub fn add_capabilities(env: &mut jvmtiEnv, capa: &jvmtiCapabilities) {
    unsafe {
        let f = (**env).AddCapabilities.unwrap();
        jvmti_check(f(env, capa));
    }
}


// [JVMTI#SetEventCallbacks](https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#SetEventCallbacks)
pub fn set_event_callbacks(env: &mut jvmtiEnv, cb: &jvmtiEventCallbacks) {
    unsafe {
        let f = (**env).SetEventCallbacks.unwrap();
        jvmti_check(f(env, cb, mem::size_of::<jvmtiEventCallbacks>().try_into().unwrap()));
    }
}

// [JVMTI#SetEventNotificationMode](https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#SetEventNotificationMode)
pub fn set_event_notification_mode(env: &mut jvmtiEnv, mode: jvmtiEventMode, event_type: jvmtiEvent, event_thread: jthread) {
    unsafe {
        let f = (**env).SetEventNotificationMode.unwrap();
        jvmti_check(f(env, mode, event_type, event_thread));
    }
}

pub fn add_to_bootstrap_class_loader_search(env: &mut jvmtiEnv, segment: *const i8) {
    unsafe {
        let f = (**env).AddToBootstrapClassLoaderSearch.unwrap();
        jvmti_check(f(env, segment));
    }
}

fn jvmti_check(err: jvmtiError) {
    if err != JVMTI_ERROR_NONE {
        panic!("JVMTI error value check failed. Error code: {}", err);
    }
}

fn jni_check(err: jint) {
    if err != JNI_OK.try_into().unwrap() {
        panic!();
    }
}
