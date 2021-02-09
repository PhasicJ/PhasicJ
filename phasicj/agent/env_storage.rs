use ::std::mem;
use ::std::os::raw;
use ::phasicj_agent_conf::PjAgentConf;
use crate::jvmti_env;
use ::jvmti::jvmtiEnv;

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
pub struct EnvStorage {
    pub conf: PjAgentConf,
}

impl EnvStorage {
    pub unsafe fn new_from_env_global_storage(env: &mut jvmtiEnv) -> EnvStorage {
        // NOTE(dwtj): JVMTI writes to this locally-allocated pointer. JVMTI
        // finds this locally-allocated by following the pointer which we pass.
        let mut ptr: mem::MaybeUninit<*mut raw::c_void> = mem::MaybeUninit::uninit();
        jvmti_env::get_environment_local_storage(
            env,
            ptr.as_mut_ptr()
        );
        let ptr: *mut raw::c_void = ptr.assume_init();
        let ptr: *mut EnvStorage = mem::transmute(ptr);
        let rv: EnvStorage = *ptr;
        return rv;
    }

    fn new_with_defaults() -> EnvStorage {
        return EnvStorage {
            conf: PjAgentConf::new_with_defaults(),
        };
    }
}
