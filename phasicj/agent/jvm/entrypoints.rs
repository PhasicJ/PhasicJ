use ::jvmti::{
    JNIEnv,
    JavaVM,
    jint,
    jclass,
};
use ::std::ffi;

use crate::agent::{
    on_load,
    on_attach,
    on_unload,
};

#[no_mangle]
pub extern fn Agent_OnLoad(
    jvm: *mut JavaVM,
    options: *mut ::std::os::raw::c_char,
    _reserved: *mut ::std::os::raw::c_void,
) -> jint {
    // NOTE(dwtj): By my reading of [the JVMTI manual][1], a zero-length string
    // should be passed here when there are no options. Howerver, I am instead
    // seeing null pointers.
    //
    //  [1]: https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#onload
    if options.is_null() {
        let options = ffi::CString::new("").unwrap();
        unsafe { on_load(&mut *jvm, options.as_c_str()); }
    } else {
        unsafe {
            let options = ffi::CStr::from_ptr(options);
            on_load(&mut *jvm, &options);
        }
    }

    return 0;
}

#[no_mangle]
pub extern fn Agent_OnAttach(
    jvm: *mut JavaVM,
    _options: *mut ::std::os::raw::c_char,
    _reserved: *mut ::std::os::raw::c_void,
) -> jint {
    unsafe { on_attach(&mut *jvm); }
    return 0;
}

#[no_mangle]
pub extern fn Agent_OnUnload(jvm: *mut JavaVM) {
    unsafe { on_unload(&mut *jvm); }
}

use crate::recorder::EVENT_FORWARDING_CHANNEL;

// TODO(dwtj): Consider putting these in the `rt` subdirectory. I tried putting
//  them in their own crate in `//phasicj/agent/rt/`, but they did not show up
//  in the final agent shared library (possibly because of some link-time
//  optimization which filtered them out because they didn't appear to be used.)
#[no_mangle]
pub extern fn Java_phasicj_agent_rt_ApplicationEvents_monitorEnter(_env: *mut JNIEnv, _cls: jclass) {
    log::trace!("`monitorEnter()` in Rust");
    EVENT_FORWARDING_CHANNEL.lock().unwrap().as_ref().unwrap().send(()).unwrap();
}

#[no_mangle]
pub extern fn Java_phasicj_agent_rt_ApplicationEvents_monitorExit(_env: *mut JNIEnv, _cls: jclass) {
    log::trace!("`monitorExit()` in Rust");
}
