#include <iostream>
#include <string>
#include <filesystem>
#include <cassert>

#include "jni.h"
#include "jvmti.h"
#include "pt/monitorenter/count/agent/rt/extract_embedded_jar.h"

namespace pt::monitorenter::count::agent {

using std::string;

jvmtiEnv* GetNewJvmtiEnv(JavaVM& jvm) {
  jvmtiEnv* env;
  jvm.GetEnv((void**) &env, JVMTI_VERSION_11);
  return env;
}

void AddRuntimeJarToBootClassPath(jvmtiEnv& env) {
  jvmtiError err;
  err = env.AddToBootstrapClassLoaderSearch(rt::write_jar_to_temp().c_str());
  assert(err == JVMTI_ERROR_NONE);
}

const jint NO_ERROR = 0;

jint OnLoad(JavaVM& jvm) {
  jvmtiEnv* env(GetNewJvmtiEnv(jvm));
  AddRuntimeJarToBootClassPath(*env);
  return NO_ERROR;
}

}  // namespace pt::monitorenter::count::agent

JNIEXPORT jint JNICALL
Agent_OnLoad(JavaVM *vm, char *options, void *reserved) {
  return pt::monitorenter::count::agent::OnLoad(*vm);
}
