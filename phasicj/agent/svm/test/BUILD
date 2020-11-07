load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_library")

dwtj_java_library(
    name = "svm_test_class_jar",
    srcs = ["SvmTestClass.java"],
    output_jar = "svm_test_class.jar",
)

genrule(
    name = "svm_test_class_unjar",
    srcs = ["svm_test_class.jar"],
    outs = ["SvmTestClass.class"],
    cmd = '''"$(location @graalvm_linux_x64//jdk:bin/jar)" -xf "$<" && mv "phasicj/agent/svm/test/SvmTestClass.class" "$@"''',
    tools = ["@graalvm_linux_x64//jdk:bin/jar"],
    visibility = [
        "//phasicj/agent/svm:__subpackages__",
    ],
)

# TODO(dwtj): Consider adding a test where `java.lang.Object` is instrumented.
#  This would be useful because instrumentation of this class is rather more
#  complicated than other classes. I spent a minute trying this, but I don't
#  know how to invoke `javac` such that I can compile a fake `java.lang.Object`
#  class. I could use ASM to rename a class.
