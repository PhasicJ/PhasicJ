load("@dwtj_rules_markdown//markdown:defs.bzl", "markdown_library")
load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_library")

markdown_library(
    name = "markdown",
    srcs = glob(["*.md"]),
)

dwtj_java_library(
    name = "instr",
    srcs = [
        "Instrumenter.java",
        "MonitorInsnInstrumenter.java",
    ],
    visibility = ["//phasicj/agent:__subpackages__"],
    deps = [
        #"//phasicj/agent/rt",
        "//third_party/asm",
    ],
    # We override the default toolchain to use a GraalVM toolchain in order to
    #  get support for `@CEntryPoint` and other definitions in `graal-sdk`.
    # TODO(dwtj): Investigate less intrusive ways to support these definitions
    #  from `graal-sdk`. Ideas:
    #
    #  - Just use the GraalVM toolchain globally.
    #  - Get a maven copy of `graal-sdk`. I didn't try this first because I
    #    suspected that I might run into problems with there
    #    being double definitions: one provided by Maven and one provided by the
    #    environment. Note that Maven actually has support for such situations:
    #    you can declare a depenedency to be "provided" in the sense that it is
    #    needed at compile time but will be provided by the runtime environment.
    #java_compiler_toolchain = "@graalvm_linux_x64//:_java_compiler_toolchain",
)
