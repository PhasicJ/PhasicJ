#include <string>
#include <cassert>
#include <filesystem>
#include <fstream>
#include <iostream>

#include <unistd.h>

#include "jni.h"
#include "jvmti.h"

#include "pt/svm/exec/embedded/analysis.h"

using std::cout;
using std::endl;
using std::string;
using std::ofstream;
using std::error_code;

namespace pt::agent {

  namespace {
    enum Error {
      None = 0,
      SvmFileMakeDirFailed = 1,
      SvmFileOpenFailed = 2,
      SvmFileWriteFailed = 3,
      SvmFileAddExecPermFailed = 4,
      SvmForkFailed = 5,
      SvmExecFailed = 6,
    };
  }

  namespace fs = std::filesystem;

  fs::path GetSvmTempPath() {
    fs::path svm(fs::temp_directory_path());
    svm /= "phasicj";
    svm /= "testing";
    svm /= "pt_graal_shadow_vm";
    svm /= "pt";
    svm /= "svm";
    return svm;
  }

  Error ExecSvm(const fs::path& svm) {
    int err = execl(svm.c_str(), "svm");
    if (err == -1) {
      return Error::SvmExecFailed;
    } else {
      // Otherwise, exec was successful and exec doesn't return here.
      assert(false);
    }
  }

  Error ForkSvm(const fs::path& svm) {
    pid_t pid;

    switch (pid = fork()) {
    case -1:
      return Error::SvmForkFailed;
    case 0:
      // Child process.
      return ExecSvm(svm);
    default:
      // Parent process.
      return Error::None;
    }
  }

  Error MakeSvmDir(const fs::path& svm) {
    fs::path svm_parent(svm.parent_path());

    if (fs::exists(svm_parent)) {
      return Error::None;
    }

    if (fs::create_directories(svm_parent)) {
      return Error::None;
    } else {
      return Error::SvmFileMakeDirFailed;
    }
  }

  Error WriteSvmFile(const fs::path& svm) {
    ofstream out(svm.c_str(), ofstream::trunc | ofstream::binary);
    if (!out.good()) {
      return Error::SvmFileOpenFailed;
    }

    out.write(reinterpret_cast<const char*>(::pt::svm::exec::embedded::start()),
              ::pt::svm::exec::embedded::size());
    if (!out.good()) {
      return Error::SvmFileWriteFailed;
    }

    out.close();
    return Error::None;
  }

  Error AddExecPermToSvmFile(const fs::path& svm) {
    error_code err;
    fs::permissions(svm, fs::perms::owner_all, fs::perm_options::add, err);
    if (bool(err)) {
      return Error::SvmFileAddExecPermFailed;
    } else {
      return Error::None;
    }
  }

  Error CreateSvmExecutableFile(const fs::path& svm) {
    Error err;

    err = MakeSvmDir(svm);
    if (err != None) {
      return err;
    }

    err = WriteSvmFile(svm);
    if (err != None) {
      return err;
    }

    err = AddExecPermToSvmFile(svm);
    if (err != None) {
      return err;
    }

    return Error::None;
  }

  jint OnLoad(JavaVM& jvm) {
    Error err = Error::None;
    fs::path svm(GetSvmTempPath());

    err = CreateSvmExecutableFile(svm);
    if (err != None) {
      return err;
    }

    err = ForkSvm(svm);
    if (err != None) {
      return err;
    }

    return Error::None;
  }

}  // namespace pt::agent

JNIEXPORT jint JNICALL
Agent_OnLoad(JavaVM *jvm, char *options_c_str, void *reserved) {
  jint err = pt::agent::OnLoad(*jvm);
  if (err != 0) {
    cout << "Failed to initialize `pt::agent`. Error code: " << err << endl;
  }
  return err;
}
