load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_library")
load(
    "@rules_cc//cc:defs.bzl",
    "cc_import",
    "cc_library",
    "cc_test",
)

dwtj_java_library(
    name = "agent_runtime",
    srcs = ["AgentRuntime.java"],
    output_jar = "agent_runtime.jar",
    visibility = ["//visibility:public"],
)

AGENT_RUNTIME_SYMBOLS_NAME_BASE = "pt_monitorenter_count_agent_rt"

# Convert this JAR to a native object file that can be embedded in the `agent`.
# This generates a native object file with three symbols:
#
# - _binary_pt_monitorenter_count_agent_rt_end
# - _binary_pt_monitorenter_count_agent_rt_size
# - _binary_pt_monitorenter_count_agent_rt_start
#
# NOTE(dwtj): By default, the `ld` command uses the input file's relative path
#  to set the symbols' names. We copy the JAR file control these symbols' names.
#  (The `nm` command can be used to list the symbols in an object file.)
genrule(
    name = "agent_runtime_jar_obj",
    srcs = [":agent_runtime.jar"],
    outs = ["agent_runtime.jar.o"],
    cmd_bash = ' \
        "./$(location :make_agent_runtime_jar_obj.sh)" \
            "{}" \
            "$(location :agent_runtime.jar)" \
            "$(location :agent_runtime.jar.o)" \
    '.format(AGENT_RUNTIME_SYMBOLS_NAME_BASE),
    tools = [":make_agent_runtime_jar_obj.sh"],
)

genrule(
    name = "agent_runtime_jar_static_library",
    srcs = ["agent_runtime.jar.o"],
    outs = ["agent_runtime.jar.a"],
    cmd_bash = 'ar rcs "$(location :agent_runtime.jar.a)" "$(location :agent_runtime.jar.o)"',
)

cc_import(
    name = "agent_runtime_jar_import",
    hdrs = ["embedded_jar.h"],
    static_library = "agent_runtime.jar.a",
)

cc_library(
    name = "extract_embedded_jar",
    srcs = ["extract_embedded_jar.cc"],
    hdrs = ["extract_embedded_jar.h"],
    visibility = ["//visibility:public"],
    deps = [":agent_runtime_jar_import"],
)

cc_test(
    name = "extract_embedded_jar_test",
    srcs = ["extract_embedded_jar_test.cc"],
    args = ['"$(location :agent_runtime.jar)"'],
    data = [":agent_runtime.jar"],
    deps = ["extract_embedded_jar"],
)
