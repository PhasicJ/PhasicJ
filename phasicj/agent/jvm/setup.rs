use ::std::os::raw;
use ::std::ptr;
use ::std::mem;
use ::jvmti::{
    JavaVM,
    jvmtiEnv,
    jvmtiEvent_JVMTI_EVENT_CLASS_FILE_LOAD_HOOK as JVMTI_EVENT_CLASS_FILE_LOAD_HOOK,
    jvmtiEvent_JVMTI_EVENT_VM_START as JVMTI_EVENT_VM_START,
    jvmtiEvent_JVMTI_EVENT_VM_INIT as JVMTI_EVENT_VM_INIT,
    jvmtiEvent_JVMTI_EVENT_VM_DEATH as JVMTI_EVENT_VM_DEATH,
    jvmtiEventMode_JVMTI_ENABLE as JVMTI_ENABLE,
    jvmtiCapabilities,
};
use ::std::ffi;

use crate::jvm::events;

use crate::jvm::jvmti_env::{
    get_potential_capabilities,
    get_capabilities,
    add_capabilities,
    set_event_callbacks,
    set_event_notification_mode,
    add_to_bootstrap_class_loader_search,
    get_env,
    set_environment_local_storage,
    allocate as jvmti_allocate,
};

use ::phasicj_agent_rt_jar_embed::{
    rt_jar_default_temp_file_path,
    write_rt_jar_file_if_missing,
};

use ::phasicj_agent_conf::PjAgentConf;
use crate::jvm::env_storage::EnvStorage;

pub fn setup_agent(jvm: &mut JavaVM, conf: PjAgentConf) {
    unsafe{
        let env: *mut jvmtiEnv = get_env(jvm);
        let env = &mut *env;
        configure_jvmti_env(env, &conf);
    }
    crate::recorder::start_forwarding_worker_thread();
}

pub fn configure_jvmti_env(env: &mut jvmtiEnv, conf: &PjAgentConf) {
    unsafe { EnvStorage::init(env, conf); }
    expect_jvmti_environment_provides_all_required_capabilities(env);
    add_all_required_capabilities(env);
    set_all_event_callbacks(env);
    set_all_event_notification_modes(env);
    add_embedded_jar_to_bootstrap_class_loader_search(env);
}

fn expect_jvmti_environment_provides_all_required_capabilities(env: &mut jvmtiEnv){
    let capa: jvmtiCapabilities = get_potential_capabilities(env);
    expect_capability(capa.can_generate_all_class_hook_events());
    expect_capability(capa.can_generate_early_class_hook_events());
}

fn add_all_required_capabilities(env: &mut jvmtiEnv) {
    let mut capa: jvmtiCapabilities = get_capabilities(env);

    // NOTE(dwtj): [ClassFileLoadHook][1] events will be sent during the
    //  primordial phase because both of these events are set. However, it seems
    //  that during the primordial phase, the VM may not able to load classes
    //  outside of `java.base`. Thus, instrumentation of classes loaded during
    //  the primordial phase is limited.
    //
    //  [1]: https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#ClassFileLoadHook
    capa.set_can_generate_all_class_hook_events(1);
    capa.set_can_generate_early_class_hook_events(1);

    add_capabilities(env, &capa);
}

fn set_all_event_callbacks(env: &mut jvmtiEnv) {
    let cb = events::get_initial_agent_callbacks();
    set_event_callbacks(env, &cb);
}

fn set_all_event_notification_modes(env: &mut jvmtiEnv) {
    let all_threads = ptr::null_mut();
    set_event_notification_mode(env, JVMTI_ENABLE, JVMTI_EVENT_CLASS_FILE_LOAD_HOOK, all_threads);
    set_event_notification_mode(env, JVMTI_ENABLE, JVMTI_EVENT_VM_START, all_threads);
    set_event_notification_mode(env, JVMTI_ENABLE, JVMTI_EVENT_VM_INIT, all_threads);
    set_event_notification_mode(env, JVMTI_ENABLE, JVMTI_EVENT_VM_DEATH, all_threads);
}

fn add_embedded_jar_to_bootstrap_class_loader_search(env: &mut jvmtiEnv) {
    let path = rt_jar_default_temp_file_path();
    write_rt_jar_file_if_missing(&path).unwrap();

    let path = ffi::CString::new(path.to_str().unwrap()).unwrap();
    let path = path.into_raw();
    unsafe {
        let path: *const i8 = mem::transmute(path);
        add_to_bootstrap_class_loader_search(env, path);
    }
}

fn expect_capability(capa: raw::c_uint) {
    if capa == 0 {
        panic!();
    }
}
