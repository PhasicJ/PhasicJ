#include <iostream>

#include "jni.h"
#include "jvmti.h"

namespace pt::monitorenter::count::agent {

// TODO(dwtj): Everything!

}  // namespace pt::monitorenter::count::agent

JNIEXPORT jint JNICALL
Agent_OnLoad(JavaVM *vm, char *options, void *reserved) {
  std::cout << "Hello, from JVMTI." << std::endl;
  return 0;
}
