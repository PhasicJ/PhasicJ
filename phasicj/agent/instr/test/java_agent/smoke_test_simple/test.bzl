load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_test")

def test(name):
    dwtj_java_test(
        name = name,
        srcs = [name + ".java"],
        deps = [
            "//phasicj/agent/rt",
            # TODO(dwtj): Remove these once transitive dependencies from Java agents are fixed.
            "//phasicj/agent/instr",
            "//third_party/asm",
        ],
        java_agents = {"//phasicj/agent/instr/test/java_agent": ""},
        main_class = "phasicj.agent.instr.test.java_agent.smoke_test_simple." + name,
    )
