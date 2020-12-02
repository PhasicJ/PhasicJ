#!/bin/sh -

# This convenience script is meant to help a PhasicJ developer build and
# run this CI container on their local development system. It is not meant
# for deploying this CI container on a CI server.
#
# This script is meant to be executed from the root of the PhasicJ workspace.
#
# Port 22 within the container is published as port 2222 on the host.
#
# This CI container is intended to have five mount points mounted in the
# following ways.
#
# 1. `/mnt/phasicj` - A read-only bind mount to the PhasicJ workspace.
# 2. `~/.config/msmtp/` - A read-only bind mount to email config & secret creds.
# 3. `~/.cache/bazel` - A volume for Bazel fetch/build/test cache.
# 4. `~/.cache/bazelisk` - A volume for bazelisk download cache.
# 5. `/etc/ssh` - A volume for SSH host keys and SSH server configuration.
#
# This script helps a developer mount these five mount points with some
# defaults:
#
# 1. This PhasicJ Bazel workspace is bind-mounted read-only.
# 2. The host user's ~/.config/msmtp directory is bind-mounted read-only.
# 3. The volume named `phasicj_ci_dev_bazel_cache` is used.
# 4. The volume named `phasicj_ci_dev_bazelisk_cache` is used.
# 5. The volume named `phasicj_ci_dev_etc_ssh` is used.

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

run_container()
{
    podman run \
        --rm \
        --privileged \
        -it \
        --mount type=bind,source=.,destination=/mnt/phasicj,ro=true \
        --mount type=bind,source="$CONTAINERFILE_DIR/secrets/msmtp",destination=/root/.config/msmtp,ro=true \
        --mount type=volume,source=phasicj_ci_dev_bazel_cache,destination=/root/.cache/bazel \
        --mount type=volume,source=phasicj_ci_dev_bazelisk_cache,destination=/root/.cache/bazelisk \
        --mount type=volume,source=phasicj_ci_dev_etc_ssh,destination=/etc/ssh \
        --publish 2222:22 \
        phasicj_ci_dev
}

check_environment
build_container
run_container
