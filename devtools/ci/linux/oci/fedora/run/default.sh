#!/bin/sh -

# This script is meant to be used as the default command run in the PhasicJ
# CI Linux container.

assert_env_var_dir_exists()
{
    VAR_NAME="$1"
    VAR_VALUE="$(printenv ${VAR_NAME})"

    if [ -z "$VAR_VALUE" ]; then
        echo "ERROR: Missing a required environment variable: $VAR_NAME"
        exit 1
    fi

    if [ ! -d "$VAR_VALUE" ]; then
        echo "ERROR: Environment variable's value is expected to be a directory, but it's not: ${VAR_NAME}=${VAR_VALUE}"
        exit 1
    fi
}

assert_file_exists()
{
    REQUIRED_FILE="$1"

    if [ ! -f "$REQUIRED_FILE" ]; then
        echo "ERROR: A required file is missing: $REQUIRED_FILE"
        exit 1
    fi
}

assert_file_is_not_empty()
{
    REQUIRED_FILE="$1"

    if [ ! -s "$REQUIRED_FILE" ]; then
        echo "ERROR: A required file is empty, but it should contain some contents: $REQUIRED_FILE"
        exit 1
    fi
}

check_test_environment()
{
    # Check that container volume mounts appear to be mounted.
    assert_env_var_dir_exists "PHASICJ_WORKSPACE_MOUNT_POINT"
    assert_env_var_dir_exists "BAZEL_CACHE_MOUNT_POINT"
    assert_env_var_dir_exists "BAZELISK_CACHE_MOUNT_POINT"

    # Check that all of the mounts points appear to be mounted correctly.
    if [ ! -f "$PHASICJ_WORKSPACE_MOUNT_POINT/.phasicj_workspace_root" ]; then
        echo "ERROR: The PhasicJ workspace directory doesn't contain an expected file: $PHASICJ_WORKSPACE_DIR/.phasicj_workspace_root is missing. Was the workspace directory mounted wrong?"
        exit 1
    fi
    if [ ! -f "$BAZEL_CACHE_MOUNT_POINT/.bazel_cache_volume_root" ]; then
        echo "ERROR: The Bazel cache directory doesn't contain an expected file."
        exit 1
    fi
    if [ ! -f "$BAZELISK_CACHE_MOUNT_POINT/.bazelisk_cache_volume_root" ]; then
        echo "ERROR: The Bazelisk cache directory doesn't contain an expected file."
        exit 1
    fi
    if [ ! -f "$MSMTP_CONFIG_MOUNT_POINT/config" ]; then
        echo "ERROR: The msmtp config directory doesn't contain an expected file."
        exit 1
    fi

    SSH_AUTHORIZED_KEYS_PATH="/root/.ssh/authorized_keys"
    assert_file_exists "$SSH_AUTHORIZED_KEYS_PATH"
    assert_file_is_not_empty "$SSH_AUTHORIZED_KEYS_PATH"
}

check_test_environment

# Create any ssh host keys which don't already exist.
ssh-keygen -A

# Run the SSH server, but don't daemonize it (i.e. detach from the terminal).
/usr/sbin/sshd -D
