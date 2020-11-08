use ::std::os::raw::c_uint;
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

use crate::jvmti_events;

use crate::jvmti_env::{
    get_potential_capabilities,
    get_capabilities,
    add_capabilities,
    set_event_callbacks,
    set_event_notification_mode,
    add_to_bootstrap_class_loader_search,
    get_env,
};

use ::phasicj_agent_rt_jar_embed::{
    rt_jar_default_temp_file_path,
    write_rt_jar,
};

pub fn setup(jvm: &mut JavaVM) {
    unsafe{
        let env: *mut jvmtiEnv = get_env(jvm);
        configure_jvmti_env(&mut *env);
    }
}

pub fn configure_jvmti_env(env: &mut jvmtiEnv) {
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
    let cb = jvmti_events::get_initial_agent_callbacks();
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
    write_rt_jar(&path).unwrap();

    let path = ffi::CString::new(path.to_str().unwrap()).unwrap();
    let path = path.into_raw();
    unsafe {
        let path: *const i8 = mem::transmute(path);
        add_to_bootstrap_class_loader_search(env, path);
    }
}

fn write_embedded_svm_lib_to_default_temp_path() {
    let path = ::phasicj_agent_svm_embed::svm_default_temp_file_path();
    ::phasicj_agent_svm_embed::write_svm_file_if_missing(&path).unwrap();
}

fn expect_capability(capa: c_uint) {
    if capa == 0 {
        panic!();
    }
}
