load(
    "@rules_rust//rust:rust.bzl",
    "rust_binary",
    "rust_library",
    "rust_test",
)

# NOTE(dwtj): This Rust target uses a `src/` subdirectory. Other Rust targets
#  in this workspace do not. Here we are breaking with our workspace convention
#  to help work around a problem that I am currently seeing with Pest and/or
#  `@rules_rust`. Here's the problem that I'm currently seeing:
#
#  - When `#[grammar = "src/PjAgentConfLang.pest"]` is used and the files are in
#   `src/`, Pest finds the grammar file just fine.
#  - But, when `#[grammar = "PjAgentConfLang.pest"]` is used and the files are
#    in this directory, then Pest fails to find the grammar file.
rust_library(
    name = "phasicj_agent_conf",
    srcs = [
        "src/lib.rs",
    ],
    edition = "2018",
    compile_data = [
        "src/PjAgentConfLang.pest",
    ],
    deps = [
        "//third_party/cargo:pest",
    ],
    proc_macro_deps = [
        "//third_party/cargo:pest_derive",
    ],
)

rust_binary(
    name = "conf_test_repl",
    srcs = [
        "src/conf_test_repl.rs",
    ],
    edition = "2018",
    deps = [
        "//third_party/cargo:pest",
        ":phasicj_agent_conf",
    ],
)

rust_test(
    name = "test",
    srcs = [
        "src/test.rs",
    ],
    edition = "2018",
    deps = [
        "phasicj_agent_conf",
        "//third_party/cargo:pest",
    ],
)

sh_test(
    name = "smoke_test_conf_test_repl",
    srcs = ["smoke_test_conf_test_repl.sh"],
    data = [":conf_test_repl"],
    args = ["$(rootpath :conf_test_repl)"],
)
