sh_test(
    name = "test",
    srcs = ["test.sh"],
    args = [
        "$(rootpath //pt/my_server)",
        "$(rootpath //pt/my_client)",
    ],
    data = [
        "//pt/my_client",
        "//pt/my_server",
        "@openjdk_linux_x64//jdk:bin/java",
    ],
)
