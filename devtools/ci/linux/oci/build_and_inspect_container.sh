#!/bin/sh -

# This convenience script is meant to help a PhasicJ developer build and
# inspect this CI container on their local development system. By "inspect", we
# mean that this script runs `bash` in the container so that the developer can
# explore the container's contents.
#
# This script is meant to be executed from the root of the PhasicJ workspace.
#
# This CI container is meant to have three volumes mounted into it. Within the
# container, the three mount points are
#
# 1. `/mnt/phasicj`
# 2. `~/.cache/bazel`
# 3. `~/.cache/bazelisk`
#
# The first of these is mapped to this PhasicJ workspace (read-only). The
# other two are mapped to the developer's Bazel and Bazelisk cache directories
# (read-write). Thus, if `bazel` is run within the container, it will modify
# these cache directories outside of the container.

# TODO(dwtj): Don't use the host's cache directories. Use empty podman volumes.

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
        -v .:/mnt/phasicj:ro \
        -v ~/.cache/bazel:/root/.cache/bazel \
        -v ~/.cache/bazelisk:/root/.cache/bazelisk \
        phasicj_ci_dev \
        bash
}

check_environment
build_container
inspect_container
