#!/bin/sh -
#
# This script builds and runs some very basic tests on the architecture
# prototypes. It should be invoked from the root of the repository.

set -e

REPOSITORY_ROOT="$PWD"

cd "${REPOSITORY_ROOT}/architecture/prototypes/integration/pt_asm_java_agent"
pwd
bazel build //...
bazel test //...

cd "${REPOSITORY_ROOT}/architecture/prototypes/integration/pt_jvmti_agent_with_embedded_jar"
pwd
bazel build //...
bazel test //...

cd "${REPOSITORY_ROOT}/architecture/prototypes/integration/pt_grpc"
pwd
bazel build //...
bazel test //...

cd "${REPOSITORY_ROOT}/architecture/prototypes/integration/pt_asm_renaissance"
pwd
bazel build //...
bazel test //...

cd "${REPOSITORY_ROOT}/architecture/prototypes/integration/pt_graal_shadow_vm"
pwd
bazel build //...
bazel test //...

cd "${REPOSITORY_ROOT}/architecture/prototypes/integration/pt_zeromq"
pwd
bazel build //...
# TODO(dwtj): Add some tests to this workspace.
#bazel test //...

cd "${REPOSITORY_ROOT}/architecture/prototypes/integration/pt_rust_jvmti_agent"
pwd
bazel build //...
bazel test //...

#cd "${REPOSITORY_ROOT}/architecture/prototypes/benchmarking/pt_jvmti_vs_bytecode_events"
#pwd
#bazel build //...
#bazel test //...

cd "${REPOSITORY_ROOT}/architecture/prototypes/integration/pt_pest_parser"
pwd
bazel build //...
# TODO(dwtj): Add a test.
#bazel test //...

cd "${REPOSITORY_ROOT}/architecture/prototypes/integration/pt_rust_grpc_ipc/Bazel"
pwd
bazel build //...
bazel test //...

cd "${REPOSITORY_ROOT}/architecture/prototypes/integration/pt_rust_grpc_ipc/Cargo"
pwd
cargo build
cargo test

echo
echo 'SUCCESS: All checks completed successfully!'
echo
