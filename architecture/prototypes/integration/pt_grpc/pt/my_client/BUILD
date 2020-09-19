load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_binary")
load("@dwtj_rules_java//java:defs.bzl", "dwtj_legacy_java_import")

dwtj_java_binary(
    name = "my_client",
    srcs = ["MyClient.java"],
    main_class = "pt.my_client.MyClient",
    deps = [":client_grpc_dep_imports"],
)

dwtj_legacy_java_import(
    name = "client_grpc_dep_imports",
    imports = [
        "//pt/my_services:greeter_java_grpc",
        "//pt/my_services:greeter_java_proto",
        "@com_github_grpc_grpc_java//api",
        "@com_github_grpc_grpc_java//okhttp",
        "@com_github_grpc_grpc_java//protobuf",
        "@com_github_grpc_grpc_java//stub",
    ],
)
