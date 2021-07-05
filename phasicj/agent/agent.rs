use jvmti::{
    JavaVM,
    jint,
};
use crate::jvm::setup::setup_agent;
use phasicj_agent_conf::PjAgentConf;
use std::ffi;

const ERROR_AGENT_ATTACH_NOT_SUPPORTED: jint = 1;

pub fn on_load(jvm: &mut JavaVM, options: &ffi::CStr) {
    // NOTE(dwtj): According to the [JVMTI docs][1], options should be passed
    // to an agent as a [modified UTF-8 string][2]. However, Rust's
    // [CStr::to_str][3] expects valid UTF-8 data.
    //
    // [1]: https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#onload
    // [2]: https://docs.oracle.com/en/java/javase/15/docs/specs/jvmti.html#mUTF
    // [3]: https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.to_str

    // TODO(dwtj): Learn how modified UTF-8 works and about how this distinction
    // might cause problems here (e.g. when the options encode a NUL byte).
    env_logger::init();
    let options = options.to_str().expect("Failed parsing the PhasicJ agent options as valid UTF-8.");
    let conf = PjAgentConf::new_from_agent_options_list_str(options);
    if conf.verbose {
        crate::debug::print_verbose_startup_info(&conf);
    }
    setup_agent(jvm, conf);
}

pub fn on_attach(_jvm: &mut JavaVM) -> jint {
    log::warn!("The PhasicJ Agent does not support being attached to a running JVM. This Agent only works if it is available at JVM startup.");
    return ERROR_AGENT_ATTACH_NOT_SUPPORTED;
}

pub fn on_unload(_jvm: &mut JavaVM) {
    log::trace!("`on_unload()` called");
    // TODO(dwtj): Is there any cleanup needed here yet?
}
