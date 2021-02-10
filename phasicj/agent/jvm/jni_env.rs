use ::std::os::raw::c_char;
use ::jvmti::{
    jmethodID,
    jclass,
    jint,
    JNIEnv,
    JNI_OK,
};
use ::std::ptr;
use ::std::convert::TryInto;

pub unsafe fn find_class(env: *mut JNIEnv, name: *const c_char) -> jclass {
    let f = (**env).FindClass.unwrap();
    let cls: jclass = f(env, name);
    if ptr::eq(cls, ptr::null()) {
        panic!();
    } else {
        return cls;
    }
}

pub unsafe fn get_static_method_id(env: *mut JNIEnv, cls: jclass, name: *const c_char, sig: *const c_char) -> jmethodID {
    let f = (**env).GetStaticMethodID.unwrap();
    let method_id = f(env, cls, name, sig);
    if ptr::eq(cls, ptr::null()) {
        panic!();
    } else {
        return method_id;
    }
}

pub unsafe fn call_static_void_method(env: *mut JNIEnv, cls: jclass, method_id: jmethodID) {
    let f = (**env).CallStaticVoidMethod.unwrap();
    f(env, cls, method_id);
}

fn jni_check(err: jint) {
    if err != JNI_OK.try_into().unwrap() {
        panic!();
    }
}
