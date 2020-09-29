#!/bin/sh -
#
# This script builds and runs some very basic tests on the architecture
# prototypes. It should be invoked from the root of the repository.

set -e

REPOSITORY_ROOT="$PWD"

cd "${REPOSITORY_ROOT}/architecture/prototypes/instrumentation/pt_asm_java_agent"
bazel build //...
bazel test //...

# TODO(dwtj): This way of invoking the client and server is a an ugly and flaky
#  hack. Consider fixing it and doing it properly.
cd "${REPOSITORY_ROOT}/architecture/prototypes/integration/pt_grpc"
bazel build //...
timeout 5s bazel-bin/pt/my_server/my_server &
sleep 1s
bazel run //pt/my_client

cd "${REPOSITORY_ROOT}/architecture/prototypes/benchmarking/pt_instrument_renaissance"
bazel build //...
bazel test //...
