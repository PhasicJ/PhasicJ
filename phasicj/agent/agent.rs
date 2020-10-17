use jvmti::{
    JavaVM,
    jint,
};
use crate::jvmti_setup::setup;

const ERROR_AGENT_ATTACH_NOT_SUPPORTED: jint = 1;

pub fn on_load(jvm: &mut JavaVM) {
    setup(jvm);
}

pub fn on_attach(_jvm: &mut JavaVM) -> jint {
    println!("The PhasicJ Agent does not support being attached to a running JVM. This Agent only works if it is available at JVM startup.");
    return ERROR_AGENT_ATTACH_NOT_SUPPORTED;
}

pub fn on_unload(_jvm: &mut JavaVM) {
    // TODO(dwtj): Is there any cleanup needed here yet?
}
