pub mod raw {
    // TODO(dwtj): Remove these re-exports.
    pub use ::svm_raw::{
        graal_isolatethread_t,
        graal_isolate_t,
        svm_instr_instrument,
        svm_instr_free,
        graal_create_isolate,
        graal_detach_thread,
        graal_tear_down_isolate,
    };
}
use ::svm_raw::{
    graal_create_isolate,
    graal_tear_down_isolate,
};
use ::std::convert::TryInto;
use ::std::ptr;
use ::std::mem::MaybeUninit;
use ::std::os;
use ::std::mem;
use ::std::slice;

// NOTE(dwtj): We do not drop this. Rather, we rely on the user to call
//  `Svm::free_svm_class` to prevent memory leaks.
// TODO(dwtj): Consider safer strategies. Can I maybe:
//  - Make the instrument class return a borrowed reference whose lifetime is
//    shorter than the Svm itself.
//  - Put this `&Svm` into the `SvmClass`.
//  - Drop the `&SvmClass` by calling to this `&Svm`'s free method.
pub struct SvmClass {
    ptr: *mut u8,
    size: usize,
}

impl SvmClass {
    pub fn size(&self) -> usize {
        self.size
    }

    pub unsafe fn as_slice(&self) -> &[u8] {
        return slice::from_raw_parts(self.ptr, self.size);
    }
}

pub struct Svm {
    isolate_ptr: *mut raw::graal_isolate_t,
    isolate_thread_ptr: *mut raw::graal_isolatethread_t,
}

impl Svm {
    pub fn new() -> Result<Svm, ()> {
        unsafe {
            let (isolate_ptr, isolate_thread_ptr) = new_graal_isolate_thread()?;
            return Ok(Svm {
                isolate_ptr,
                isolate_thread_ptr,
            });
        }
    }

    // TODO(dwtj): Figure out how to make the `class` slice immutable. We need
    //  a guarantee from the SVM interface that it won't modify these bytes.
    pub unsafe fn instrument(&mut self, class: &mut [u8]) -> Result<SvmClass, ()> {
        let inBufSize: os::raw::c_int = class.len().try_into().unwrap();
        let inBuf: *mut os::raw::c_char = mem::transmute(class.as_mut_ptr());

        // TODO(dwtj): Could/should I make this simpler by just initializing
        //  these to 0 and `ptr::null()`, respectively?
        let mut svmBufSize: MaybeUninit<os::raw::c_int> = MaybeUninit::uninit();
        let mut svmBuf: MaybeUninit<*mut os::raw::c_char> = MaybeUninit::uninit();

        let err = raw::svm_instr_instrument(
            self.isolate_thread_ptr,
            inBufSize,
            inBuf,
            svmBufSize.as_mut_ptr(),
            svmBuf.as_mut_ptr(),
        );
        if err != 0 {
            return Err(()); // TODO(dwtj): Consider including an error message.
        }

        return Ok(SvmClass {
            ptr: mem::transmute(svmBuf.assume_init()),
            size: svmBufSize.assume_init().try_into().unwrap(),
        });
    }

    pub unsafe fn free_svm_class(&mut self, svm_class: SvmClass) -> Result<(), ()>{
        // TODO(dwtj): Consider changing the interface of `svm_instr_free()` to
        //  return an error code and forwarding this error.
        let ptr: *mut i8 = mem::transmute(svm_class.ptr);
        raw::svm_instr_free(self.isolate_thread_ptr, ptr);
        Ok(())
    }
}

// TODO(dwtj): Remove `pub` once this is no longer used by `jvmti_events.rs`.
pub unsafe fn new_graal_isolate_thread() -> Result<(*mut raw::graal_isolate_t, *mut raw::graal_isolatethread_t), ()> {
    let create_isolate_params = ptr::null_mut();
    let mut isolate_ptr: MaybeUninit<*mut raw::graal_isolate_t> = MaybeUninit::uninit();
    let mut isolate_thread_ptr: MaybeUninit<*mut raw::graal_isolatethread_t> = MaybeUninit::uninit();
    let err = graal_create_isolate(
        create_isolate_params,
        isolate_ptr.as_mut_ptr(),
        isolate_thread_ptr.as_mut_ptr(),
    );

    if err == 0 {
        return Ok((isolate_ptr.assume_init(), isolate_thread_ptr.assume_init()));
    } else {
        return Err(());
    }
}

impl Drop for Svm {
    fn drop(&mut self) {
        unsafe {
            let err = graal_tear_down_isolate(self.isolate_thread_ptr);
            if err != 0 {
                panic!("While dropping a `GraalIsolateThread`, a call to the SVM's `graal_tear_down_isolate()` function failed.");
            }
        }
    }
}
