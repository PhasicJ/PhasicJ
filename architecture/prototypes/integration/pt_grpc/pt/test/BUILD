sh_test(
    name = "test",
    srcs = ["test.sh"],
    args = [
        "$(location //pt/my_server)",
        "$(location //pt/my_client)",
    ],
    data = [
        "@dwtj_local_openjdk//:java",
        "//pt/my_server",
        "//pt/my_client",
    ],
)
