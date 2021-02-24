#include <cassert>
#include <vector>
#include <filesystem>
#include <iostream>
#include <fstream>
#include <dlfcn.h>
#include "phasicj/agent/experimental/svm/svm_dynamic.h"

namespace fs = std::filesystem;

using ::std::cerr;
using ::std::endl;
using ::std::vector;
using ::std::ifstream;
using ::std::ios;
using ::std::streamsize;

constexpr int MAGIC_NUMBER_SIZE = 4;
constexpr uint8_t MAGIC_NUMBER[MAGIC_NUMBER_SIZE] = {0xCA, 0xFE, 0xBA, 0xBE};

bool starts_with_magic_number(int size, char* class_data) {
    uint8_t* unsigned_class_data = (uint8_t*) class_data;
    if (size < MAGIC_NUMBER_SIZE) {
        return false;
    }
    for (size_t idx = 0; idx < MAGIC_NUMBER_SIZE; idx++) {
        if (unsigned_class_data[idx] != MAGIC_NUMBER[idx]) {
            return false;
        }
    }
    return true;
}

void* non_null(void* ptr) {
    assert(ptr != nullptr);
    return ptr;
}

// NOTE(dwtj): `argv[1]` is the file path to the SVM lib; `argv[2]` is the file
//  path to the Java test class to instrument.
int main(int argc, char** argv) {
    assert(argc == 3);

    // Open the svm library using `dlopen()` and get function pointers to its
    //  relevant symbols using `dlsym()`.
    char* svm_lib_path_str = argv[1];

    fs::path svm_lib_path(svm_lib_path_str);
    cerr << "Loading svm library (relative path): " << svm_lib_path << endl;
    assert(fs::exists(svm_lib_path));

    void* svm_lib = dlopen(svm_lib_path.c_str(), RTLD_NOW);
    if (svm_lib == nullptr) {
        cerr << "`dlopen()` failed: " << dlerror() << endl;
        return 1;
    }

    auto graal_create_isolate_fn = (graal_create_isolate_fn_t) non_null(dlsym(svm_lib, "graal_create_isolate"));
    auto graal_detach_all_threads_and_tear_down_isolate_fn = (graal_detach_all_threads_and_tear_down_isolate_fn_t) non_null(dlsym(svm_lib, "graal_detach_all_threads_and_tear_down_isolate"));
    auto svm_instr_instrument_fn = (svm_instr_instrument_fn_t) non_null(dlsym(svm_lib, "svm_instr_instrument"));
    auto svm_instr_free_fn = (svm_instr_free_fn_t) non_null(dlsym(svm_lib, "svm_instr_free"));

    // Create a path to the class file.
    char* class_file_str = argv[2];
    fs::path class_file_path(class_file_str);
    cerr << "Instrumenting test class file: " << class_file_path << endl;
    assert(fs::exists(class_file_path));

    // Load the test class from the file system.
    // NOTE(dwtj): We get the size of the input file by opening it as a stream
    //  at the end of the file and checking the stream position.
    ifstream class_data_stream(class_file_path, ios::binary | ios::ate);
    assert(class_data_stream.is_open());
    std::streamsize stream_size = class_data_stream.tellg();
    class_data_stream.seekg(0, ios::beg);

    vector<char> class_data(stream_size);
    class_data_stream.read(class_data.data(), stream_size);
    assert(class_data_stream.good());
    assert(class_data.size() == stream_size);

    int in_buf_size = class_data.size();
    char* in_buf = class_data.data();
    assert(starts_with_magic_number(in_buf_size, in_buf));

    // Create a Graal isolate & isolate thread.
    graal_create_isolate_params_t* params = nullptr;
    graal_isolate_t* isolate = nullptr;
    graal_isolatethread_t* thread = nullptr;

    assert((*graal_create_isolate_fn)(params, &isolate, &thread) == 0);
    assert(isolate != nullptr);
    assert(thread != nullptr);

    // Use this Graal isolate & thread to instrument the test class.
    int out_buf_size = 0;
    char* out_buf = nullptr;
    (*svm_instr_instrument_fn)(thread, in_buf_size, (char* ) in_buf, &out_buf_size, &out_buf);
    assert(starts_with_magic_number(out_buf_size, out_buf));

    // Free SVM memory.
    (*svm_instr_free_fn)(thread, out_buf);

    // Destroy the Graal isolate & isolate thread.
    assert((*graal_detach_all_threads_and_tear_down_isolate_fn)(thread) == 0);

    cerr << "Test passed." << endl;
}
