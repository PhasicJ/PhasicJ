#include <cassert>
#include <cstddef>
#include <iterator>
#include <filesystem>
#include <iostream>

#include "pt/monitorenter/count/agent/rt/extract_embedded_jar.h"

namespace fs = std::filesystem;

using std::cerr;
using std::endl;

namespace pt::monitorenter::count::agent::rt {

// The embedded JAR file should have the same contents as the given JAR file.
void test(fs::path jar_file) {
  // Sanity check the relationship between the three definitions describing the
  //  embedded JAR.
  assert((size_t)(jar_end_ptr() - jar_start_ptr()) == jar_size());

  assert(fs::exists(jar_file));
  assert(fs::file_size(jar_file) == jar_size());

  fs::path temp_jar_file(write_jar_to_temp());
  assert(fs::exists(temp_jar_file));
  assert(fs::file_size(temp_jar_file));

  // TODO(dwtj): Check the contents of the extracted file.

  // Clean up temporary files.
  // TODO(dwtj): Consider cleaning up the JAR file's ancestor directories (if
  //  they are empty).
  fs::remove(temp_jar_file);
}

}  // namespace pt::monitorenter::count::agent::rt

// We assume that `argv[1]` is a file path to a copy of the embedded JAR file.
// We check that this file is equivalent to the JAR file embedded in this
// library.
int main(int argc, char** argv) {
  assert(argc == 2);
  fs::path jar_file(argv[1]);
  pt::monitorenter::count::agent::rt::test(jar_file);
  cerr << "Test Passed." << endl;
}
