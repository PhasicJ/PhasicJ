load("@io_bazel_rules_docker//container:container.bzl", "container_image")

container_image(
    name = "hello_world",
    base = "@fedora//image",
    files = ["hello_world.sh"],
    cmd = ["./hello_world.sh"],
)
