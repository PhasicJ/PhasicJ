sh_test(
    name = "test",
    srcs = ["test.sh"],
    args = [
        "$(location //pt/my_server)",
        "$(location //pt/my_client)",
    ],
    data = [
        "//pt/my_client",
        "//pt/my_server",
        "@dwtj_local_openjdk//:java",
    ],
)
