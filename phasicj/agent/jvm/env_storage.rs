/// Implements storage of configuration data in JVMTI-provided "environment-
/// local storage". Between storage `init()` and `free()`, this storage is meant
/// to be readable from the PhasicJ Agent Rust code implementing any JVMTI event
/// callback via `EnvStorage::of(&mut jvmtiEnv)`.

use ::std::convert::TryInto;
use ::std::mem;
use ::std::slice;
use ::std::str;
use ::std::os::raw;
use ::std::ptr;
use ::phasicj_agent_conf::PjAgentConf;
use crate::jvm::jvmti_env;
use ::jvmti::jvmtiEnv;

#[derive(Debug)]
#[derive(Clone)]
pub struct EnvStorage {
    pub conf: PjAgentConf,
}

impl EnvStorage {
    pub unsafe fn init(env: &mut jvmtiEnv, conf: &PjAgentConf) {
        if EnvStorage::of(env).is_some() {
            panic!("The environment-local storage of this `jvmtiEnv` appears to already be initialized.")
        }

        // Allocate some JVMTI-managed memory for a `RawEnvStorage` struct to
        // be put in environment-local storage.
        let storage = RawEnvStorage::allocate(env);
        storage.write(RawEnvStorage::from_conf(env, conf));
        let storage: *mut raw::c_void = mem::transmute(storage);
        jvmti_env::set_environment_local_storage(env, storage);
    }

    pub fn of(env: &mut jvmtiEnv) -> Option<EnvStorage> {
        let ptr = unsafe { RawEnvStorage::get_ptr(env) };
        return match ptr {
            None => None,
            Some(ptr) => unsafe { Some((*ptr).to_env_storage()) }
        };
    }

    pub unsafe fn free(env: &mut jvmtiEnv) {
        let storage = RawEnvStorage::get_ptr(env).expect(
            "The JVMTI environment-local storage of this `jvmtiEnv` does not appear initialized."
        );

        // Free any inner allocations.
        if !(*storage).phasicj_exec_ptr.is_null() {
            let mem: *mut raw::c_uchar = mem::transmute(
                (*storage).phasicj_exec_ptr
            );
            jvmti_env::deallocate(env, mem);
        }

        // Deallocate the outer-most `RawEnvStorage` object.
        jvmti_env::set_environment_local_storage(env, ptr::null_mut());
    }

}

/// An object of this type is what is actually stored into the JVMTI
/// environment-local storage.
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[repr(C)]
struct RawEnvStorage {
    verbose: bool,
    phasicj_exec_ptr: *const u8,
    phasicj_exec_len: usize,
    debug_dump_classes_before_instr: bool,
    debug_dump_classes_after_instr: bool,
}

impl RawEnvStorage {
    fn to_env_storage(&self) -> EnvStorage {
        let phasicj_exec = if self.phasicj_exec_ptr.is_null() {
            None
        } else {
            let buf = unsafe {
                slice::from_raw_parts(
                    self.phasicj_exec_ptr,
                    self.phasicj_exec_len
                )
            };
            let buf = str::from_utf8(buf).unwrap();
            Some(buf.to_string())
        };
        EnvStorage {
            conf: PjAgentConf {
                verbose: self.verbose,
                phasicj_exec: phasicj_exec,
                debug_dump_classes_before_instr: self.debug_dump_classes_before_instr,
                debug_dump_classes_after_instr: self.debug_dump_classes_after_instr,
            }
        }
    }

    fn from_conf(env: &mut jvmtiEnv, given: &PjAgentConf) -> RawEnvStorage {
        let phasicj_exec_ptr: *mut u8;
        let phasicj_exec_len: usize;

        match &given.phasicj_exec {
            None => {
                phasicj_exec_len = 0;
                phasicj_exec_ptr = ptr::null_mut();
            },
            Some(s) => {
                let bytes: &[u8] = s.as_bytes();
                phasicj_exec_len = bytes.len();
                phasicj_exec_ptr = unsafe { allocate(env, phasicj_exec_len) };
                unsafe {
                    ptr::copy_nonoverlapping(
                        bytes.as_ptr(),
                        phasicj_exec_ptr,
                        phasicj_exec_len
                    );
                }
            },
        };
        RawEnvStorage {
            verbose: given.verbose,
            phasicj_exec_ptr: phasicj_exec_ptr,
            phasicj_exec_len: phasicj_exec_len,
            debug_dump_classes_before_instr: given.debug_dump_classes_before_instr,
            debug_dump_classes_after_instr: given.debug_dump_classes_after_instr,
        }
    }

    unsafe fn allocate(env: &mut jvmtiEnv) -> *mut RawEnvStorage {
        allocate_type(env)
    }

    unsafe fn get_ptr(env: &mut jvmtiEnv) -> Option<*mut RawEnvStorage> {
        let mut ptr: mem::MaybeUninit<*mut raw::c_void> = mem::MaybeUninit::uninit();
        let ptr: *mut RawEnvStorage = {
            jvmti_env::get_environment_local_storage(
                env,
                ptr.as_mut_ptr()
            );
            let ptr: *mut raw::c_void = ptr.assume_init();
            mem::transmute(ptr)
        };

        return if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    }
}

unsafe fn allocate_type<T: Sized>(env: &mut jvmtiEnv) -> *mut T {
    let buf: *mut T = mem::transmute(allocate(env, mem::size_of::<T>()));
    if !is_jvmti_allocation_aligned(buf) {
        panic!("Memory allocated by JVMTI for environment-local storage is not correctly aligned.");
    } else {
        return buf;
    }
}

unsafe fn allocate(env: &mut jvmtiEnv, size: usize) -> *mut u8 {
    if size == 0 {
        panic!("Attempted to use JVMTI environment to allocate a 0-sized object.");
    }
    let mut buf: mem::MaybeUninit<*mut raw::c_uchar> = mem::MaybeUninit::uninit();
    let buf: *mut u8 = {
        jvmti_env::allocate(
            env,
            size.try_into().unwrap(),
            buf.as_mut_ptr()
        );
        let buf: *mut raw::c_uchar = buf.assume_init();
        mem::transmute(buf)
    };
    return buf;
}

fn is_jvmti_allocation_aligned<T>(env_storage: *const T) -> bool {
    ((env_storage as usize) % mem::align_of::<T>()) == 0
}
