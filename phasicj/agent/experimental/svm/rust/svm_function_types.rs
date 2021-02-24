use ::std::os::raw::{
    c_int,
    c_char,
};

use ::svm_raw::{
    graal_create_isolate_params_t,
    graal_isolate_t,
    graal_isolatethread_t,
};

pub type GraalCreateIsolateFn = unsafe extern fn(
    *mut graal_create_isolate_params_t,
    *mut *mut graal_isolate_t,
    *mut *mut graal_isolatethread_t,
) -> c_int;

pub type GraalTearDownIsolateFn = unsafe extern fn(
    *mut graal_isolatethread_t,
) -> c_int;

pub type SvmInstrInstrumentFn = unsafe extern fn(
    thread: *mut graal_isolatethread_t,
    in_buf_size: c_int,
    in_buf: *mut c_char,
    out_buf_size: *mut c_int,
    out_buf: *mut *mut c_char
) -> c_int;

pub type SvmInstrFreeFn = unsafe extern fn(
    thread: *mut graal_isolatethread_t,
    *mut c_char
);
