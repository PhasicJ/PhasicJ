mod svm_function_types;

use ::std::slice;
use ::std::path::Path;
use ::std::mem;
use ::std::ptr;
use ::std::convert::TryInto;
use ::std::os::raw::{
    c_int,
    c_char,
};

use ::libloading as lib;

use svm_function_types::{
    GraalCreateIsolateFn,
    GraalTearDownIsolateFn,
    SvmInstrInstrumentFn,
    SvmInstrFreeFn,
};

use ::svm_raw::{
    graal_isolatethread_t,
};

/// Instances of this struct are thread safe. An internal mutex guards access to
/// its state.
pub struct SvmIsolateThread {
    // TODO(dwtj): Add a mutex.

    isolate_thread: *mut graal_isolatethread_t,

    svm_instr_instrument_fn: SvmInstrInstrumentFn,
    svm_instr_free_fn: SvmInstrFreeFn,

    svm_library: lib::Library,
}

// NOTE(dwtj): We do not drop this. Rather, we rely on the user to call
//  `Svm::free_svm_class` to prevent memory leaks.
// TODO(dwtj): Consider safer strategies.
pub struct SvmInstrBuffer {
    ptr: *mut u8,
    size: usize,
}

impl SvmInstrBuffer {
    pub fn size(&self) -> usize {
        self.size
    }

    pub unsafe fn as_slice(&self) -> &[u8] {
        return slice::from_raw_parts(self.ptr, self.size);
    }
}

impl SvmIsolateThread {
    /// This function relies upon `libloading::Library::new()` to load the
    /// library. That function [is not thread safe][1] on all platforms. Thus,
    /// this function inherits this unsafety.
    ///
    /// [1]: https://docs.rs/libloading/0.6.5/libloading/struct.Library.html#thread-safety
    pub unsafe fn new_from_library_path(path: &Path) -> Result<SvmIsolateThread, ()> {
        // TODO(dwtj): Consider creating more informative error reporting.
        let svm_library = lib::Library::new(path).or(Err(()))?;

        return Ok(SvmIsolateThread {
            isolate_thread: graal_create_isolate(&svm_library)?,
            svm_instr_instrument_fn: *get_svm_symbol(&svm_library, b"svm_instr_instrument")?,
            svm_instr_free_fn: *get_svm_symbol(&svm_library, b"svm_instr_free")?,
            svm_library,
        });
    }

    pub unsafe fn svm_instr_instrument(&self, in_buf: &mut [u8]) -> Result<SvmInstrBuffer, ()> {
        let in_buf_size: c_int = in_buf.len().try_into().unwrap();
        let in_buf = mem::transmute::<*mut u8, *mut i8>(in_buf.as_mut_ptr());

        let mut out_buf_size = mem::MaybeUninit::<c_int>::uninit();
        let mut out_buf = mem::MaybeUninit::<*mut i8>::uninit();

        let err = (self.svm_instr_instrument_fn)(
            self.isolate_thread,
            in_buf_size,
            in_buf,
            out_buf_size.as_mut_ptr(),
            out_buf.as_mut_ptr(),
        );

        let out_buf_size = unsafe { out_buf_size.assume_init() };
        let out_buf = unsafe { out_buf.assume_init() };
        let out_buf = unsafe { mem::transmute::<*mut i8, *mut u8>(out_buf) };
        let out_buf = SvmInstrBuffer {
            ptr: out_buf,
            size: out_buf_size.try_into().unwrap()
        };

        return if err == 0 {
            Ok(out_buf)
        } else {
            Err(())
        }
    }

    // TODO(dwtj): Consider adding an error code to this function.
    pub unsafe fn svm_instr_free(&self, buf: SvmInstrBuffer) -> Result<(), ()>{
        let buf = mem::transmute::<*mut u8, *mut i8>(buf.ptr);
        (self.svm_instr_free_fn)(self.isolate_thread, buf);
        return Ok(());
    }
}

impl Drop for SvmIsolateThread {
    fn drop(&mut self) {
        unsafe {
            graal_tear_down_isolate(&self.svm_library, self.isolate_thread).unwrap()
        }
    }
}

unsafe fn get_svm_symbol<'a, T>(svm_library: &'a lib::Library, symbol_name: &[u8]) -> Result<lib::Symbol<'a, T>, ()> {
    // TODO(dwtj): Consider creating more informative error reporting.
    let res = svm_library.get(symbol_name);
    return if res.is_err() {
        Err(())
    } else {
        Ok(res.unwrap())
    }
}

unsafe fn graal_create_isolate(svm_library: &lib::Library) -> Result<*mut graal_isolatethread_t, ()> {
    let mut params = ptr::null_mut();
    let mut isolate = ptr::null_mut();
    let mut isolate_thread = mem::MaybeUninit::<*mut graal_isolatethread_t>::uninit();

    let func: lib::Symbol<GraalCreateIsolateFn> = get_svm_symbol(&svm_library, b"graal_create_isolate")?;
    let err = (*func)(params, isolate, isolate_thread.as_mut_ptr());
    let isolate_thread = isolate_thread.assume_init();

    return if err == 0 {
        Ok(isolate_thread)
    } else {
        Err(())
    }
}

unsafe fn graal_tear_down_isolate(svm_library: &lib::Library, isolate_thread: *mut graal_isolatethread_t) -> Result<(), ()> {
    let func: lib::Symbol<GraalTearDownIsolateFn> = get_svm_symbol(&svm_library, b"graal_tear_down_isolate")?;
    let err = (*func)(isolate_thread);
    return if err == 0 {
        Ok(())
    } else {
        Err(())
    }
}
