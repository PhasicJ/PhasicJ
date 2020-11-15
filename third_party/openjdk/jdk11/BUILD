package(
    default_visibility = ["//visibility:public"],
)

_LINUX = "@bazel_tools//src/conditions:linux_x86_64"
_MACOS = "@bazel_tools//src/conditions:darwin"

alias(
    name = "java",
    actual = select({
        _LINUX: "@openjdk_11_linux_x64//jdk:bin/java",
        _MACOS: "@openjdk_11_macos_x64//jdk:bin/java",
    }),
)
