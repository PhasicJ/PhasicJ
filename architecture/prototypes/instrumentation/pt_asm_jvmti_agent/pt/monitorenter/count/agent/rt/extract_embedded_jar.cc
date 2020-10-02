#include <cstddef>
#include <cassert>
#include <cstdint>
#include <filesystem>
#include <iostream>
#include <fstream>
#include <sys/types.h>
#include <unistd.h>

#include "pt/monitorenter/count/agent/rt/embedded_jar.h"
#include "pt/monitorenter/count/agent/rt/extract_embedded_jar.h"

namespace pt::monitorenter::count::agent::rt {

namespace fs = std::filesystem;

using std::byte;
using std::string;
using std::to_string;
using std::ofstream;
using std::ios;

namespace {
  string pid_string() {
    pid_t pid(getpid());
    return to_string(pid);
  }
}  // namespace <anonymous>

const byte* jar_start_ptr() {
  return _binary_pt_monitorenter_count_agent_rt_start;
}

size_t jar_size() {
  return (size_t)&_binary_pt_monitorenter_count_agent_rt_size;
}

const byte* jar_end_ptr() {
  return &_binary_pt_monitorenter_count_agent_rt_end;
}

void write_jar_to_path(fs::path jar_output_path) {
  // Create the directory in which the JAR file will be written.
  if (jar_output_path.has_parent_path()) {
    fs::create_directories(jar_output_path.parent_path());
  }

  ofstream out(jar_output_path, ofstream::out & ofstream::binary & ofstream::trunc);
  out.write((const char*) jar_start_ptr(), jar_size());
  out.close();
}

fs::path write_jar_to_temp() {
  fs::path temp(fs::temp_directory_path());
  temp /= fs::path("phasicj");
  temp /= fs::path("testing");
  temp /= fs::path("pid-");
  temp += fs::path(pid_string());
  temp /= fs::path("agent_runtime.jar");

  write_jar_to_path(temp);

  return temp;
}

}  // namespace pt::monitorenter::count::agent::rt
