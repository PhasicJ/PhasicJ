load("@io_bazel_rules_rust//rust:rust.bzl", "rust_test")
load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_library")

dwtj_java_library(
    name = "test_class_jar",
    srcs = ["TestClass.java"],
    output_jar = "test_class.jar",
)

genrule(
    name = "test_class_unjar",
    srcs = ["test_class.jar"],
    outs = ["TestClass.class"],
    cmd = '''"$(location @graalvm_linux_x64//jdk:bin/jar)" -xf "$<" && mv "phasicj/agent/svm/test/TestClass.class" "$@"''',
    tools = ["@graalvm_linux_x64//jdk:bin/jar"],
)

rust_test(
    name = "test_svm",
    srcs = ["test.rs"],
    rustc_env = {
        "TEST_CLASS_EXEC_PATH": "$(execpath :TestClass.class)"
    },
    deps = ["//phasicj/agent/svm"],
    data = [":TestClass.class"],
)

# TODO(dwtj): Consider adding a test where `java.lang.Object` is instrumented.
#  This would be useful because instrumentation of this class is rather more
#  complicated than other classes. I spent a minute trying this, but I don't
#  know how to invoke `javac` such that I can compile a fake `java.lang.Object`
#  class. I could use ASM to rename a class.
