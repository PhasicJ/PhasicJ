# Rust gRPC Inter-Procedure Communication Prototype

This architectural integration prototype is meant to demonstrate various
integrations of the following technologies:

- [Bazel][1] or [Cargo][2] a the primary project build system.
- [Rust][3] as a client and server implementation language.
- [Tokio][4] as a Rust async runtime and async I/O utility library.
- [gRPC][5] as a standard for defining service interfaces using [protobufs][6].
- [tonic][7] as a Rust gRPC implementation and code generator.
- *UNIX domain sockets* as a means of transporting inter-process communication.

This exploration was undertaken to help find an approach for inter-process
communication between the PhasicJ native agent and a planned server component
which consumes and analyzes PhasicJ events.

## Notes

We chose UNIX domain sockets in order to rely on UNIX permissions as a simple
authentication strategy. They can only be used for host-local communication,
but this limitation is no problem at this point.

An example which includes a gRPC/Tonic client and server communicating over
UNIX domain sockets is available [here][8].

When using Bazel, we also use the `cargo-raze` Cargo plugin/extension to
generate Cargo dependency information to make Cargo dependencies available from
within Bazel. Configuration is put in `//third_pary/cargo:Cargo.toml` and
`cargo raze` is executed to generate appropriate `.bzl` and `BUILD` files.

Bazel's `rules_rust` appear to provide us with two ways to generate source code
from gRPC/protobuf files. We can either use the built-in rules (e.g.,
`rust_grpc_library`) or we can use the Cargo-style `build.rs` approach. We
demonstrate these approaches with `//pt/grpc/bazel_style` and
`//pt/grpc/cargo_style`, respectively.

---

[1]: https://bazel.build
[2]: https://github.com/rust-lang/cargo
[3]: https://www.rust-lang.org/
[4]: https://tokio.rs/
[5]: https://grpc.io/
[6]: https://developers.google.com/protocol-buffers
[7]: https://docs.rs/tonic/
[8]: https://github.com/hyperium/tonic/blob/e5e311853bff347355722bc829d40f54e8954aee/examples/src/uds/server.rs
