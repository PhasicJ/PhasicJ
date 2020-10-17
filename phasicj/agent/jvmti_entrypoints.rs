use ::jvmti::{
    JavaVM,
    jint,
};

use crate::agent::{
    on_load,
    on_attach,
    on_unload,
};

#[no_mangle]
pub extern fn Agent_OnLoad(
    jvm: *mut JavaVM,
    _options: *mut ::std::os::raw::c_char,
    _reserved: *mut ::std::os::raw::c_void,
) -> jint {
    unsafe { on_load(&mut *jvm); }
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
