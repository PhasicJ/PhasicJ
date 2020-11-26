#!/bin/sh -

# This convenience script is meant to help a PhasicJ developer build and
# inspect this CI container on their local development system. By "inspect", we
# mean that this script runs `bash` in the container so that the developer can
# explore the container's contents.
#
# This script is meant to be executed from the root of the PhasicJ workspace.
#
# This CI container is meant to have three mount points.
#
# 1. `/mnt/phasicj`
# 2. `~/.cache/bazel`
# 3. `~/.cache/bazelisk`
#
# Within the inspected container, the first of these is mapped to this PhasicJ
# workspace (read-only) via a bind mount. The other two are mapped to the
# Podman volumes `phasicj_ci_bazel_cache` and `phasicj_ci_bazelisk_cache`,
# respecitvely.

CONTAINERFILE_DIR="devtools/ci/linux/oci"

check_environment()
{
    if [ ! -f .phasicj_workspace_root ]; then
        echo "ERROR: This script is meant to be invoked from the root of the PhasicJ workspace."
        exit 1
    fi
}

build_container()
{
    podman build \
        --tag phasicj_ci_dev \
        "$CONTAINERFILE_DIR"
    if [ ! $? ]; then
        echo "ERROR: Failed to build container."
        exit 1
    fi
}

inspect_container()
{
    podman run \
        --rm \
        --privileged \
        -it \
        --mount type=bind,source=.,destination=/mnt/phasicj,ro=true \
        --mount type=volume,source=phasicj_ci_bazel_cache,destination=/root/.cache/bazel \
        --mount type=volume,source=phasicj_ci_bazelisk_cache,destination=/root/.cache/bazelisk \
        phasicj_ci_dev \
        bash
}

check_environment
build_container
inspect_container
