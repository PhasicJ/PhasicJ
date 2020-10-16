load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_test")

JAVA_TEST_PACKAGE = "phasicj.agent.instr.test.java_agent.smoke_test_simple"

def test(name):
    dwtj_java_test(
        name = name,
        srcs = [name + ".java"],
        deps = ["//phasicj/agent/rt"],
        java_agents = {"//phasicj/agent/instr/test/java_agent": ""},
        main_class = JAVA_TEST_PACKAGE + "." + name,
    )
