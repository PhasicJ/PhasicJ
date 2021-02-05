load("@rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "csv_parser",
    srcs = ["src/lib.rs"],
    edition = "2018",
    compile_data = ["src/csv.pest"],
    crate_root = "src/lib.rs",
    deps = ["//third_party/cargo:pest"],
    proc_macro_deps = ["//third_party/cargo:pest_derive"],
)
