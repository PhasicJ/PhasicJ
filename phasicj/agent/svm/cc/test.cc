#include <cassert>
#include <vector>
#include <filesystem>
#include <iostream>
#include <fstream>
#include "phasicj/agent/svm/svm.h"

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

int main(int argc, char** argv) {

    // Create a path to the class file.
    assert(argc == 2);
    char* class_path_str = argv[1];
    cerr << "Instrumenting test class file: " << class_path_str << endl;
    fs::path class_file_path(class_path_str);
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

    assert(graal_create_isolate(params, &isolate, &thread) == 0);
    assert(isolate != nullptr);
    assert(thread != nullptr);

    // Use this Graal isolate & thread to instrument the test class.
    int out_buf_size = 0;
    char* out_buf = nullptr;
    svm_instr_instrument(thread, in_buf_size, (char* ) in_buf, &out_buf_size, &out_buf);
    assert(starts_with_magic_number(out_buf_size, out_buf));

    // Free SVM memory.
    svm_instr_free(thread, out_buf);

    // Destroy the Graal isolate & isolate thread.
    assert(graal_detach_all_threads_and_tear_down_isolate(thread) == 0);
}
