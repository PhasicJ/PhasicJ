load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_test")

dwtj_java_test(
    name = "hello_world",
    srcs = ["HelloWorld.java"],
    java_agents = {"//phasicj/agent/instr/test/java_agent": ""},
    main_class = "phasicj.agent.instr.test.java_agent.smoke_test_simple.HelloWorld",
)
