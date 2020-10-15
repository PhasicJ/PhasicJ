use jvmti::{
    JavaVM,
    jint,
};
use svm::svm_instrument;

#[no_mangle]
pub extern fn Agent_OnLoad(
    _vm: *mut JavaVM,
    _options: *mut ::std::os::raw::c_char,
    _reserved: *mut ::std::os::raw::c_void,
) -> jint {
    println!("Hello, from `Agent_OnLoad()`, implemented in Rust.");
    return 0;
}

#[no_mangle]
pub extern fn Agent_OnAttach(
    _vm: *mut JavaVM,
    _options: *mut ::std::os::raw::c_char,
    _reserved: *mut ::std::os::raw::c_void,
) -> jint {
    println!("Hello, from `Agent_OnAttach()`, implemented in Rust.");
    return 0;
}

#[no_mangle]
pub extern fn Agent_OnUnload(_vm: *mut JavaVM) {
    println!("Hello, from `Agent_OnUnload()`, implemented in Rust.");
}
