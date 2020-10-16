mod agent;

use jvmti::{
    JavaVM,
    jint,
};
use svm::instr::instrument;

#[no_mangle]
pub extern fn Agent_OnLoad(
    _vm: *mut JavaVM,
    _options: *mut ::std::os::raw::c_char,
    _reserved: *mut ::std::os::raw::c_void,
) -> jint {
    agent::on_load();
    return 0;
}

#[no_mangle]
pub extern fn Agent_OnAttach(
    _vm: *mut JavaVM,
    _options: *mut ::std::os::raw::c_char,
    _reserved: *mut ::std::os::raw::c_void,
) -> jint {
    agent::on_attach();
    return 0;
}

#[no_mangle]
pub extern fn Agent_OnUnload(_vm: *mut JavaVM) {
    agent::on_unload();
}
