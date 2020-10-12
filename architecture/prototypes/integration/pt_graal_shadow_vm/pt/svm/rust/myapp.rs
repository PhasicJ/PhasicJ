use std::ffi::CString;
use analysis::run_main;

pub fn main() {
    unsafe {
        let arg0 = CString::new("myapp").expect("CString::new failed");
        let mut argv: [*mut i8; 1] = [arg0.into_raw()];
        run_main(1, argv.as_mut_ptr());
    }
}
