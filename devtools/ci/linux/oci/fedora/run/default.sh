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
    assert_env_var_dir_exists "PHASICJ_EDGE_REPO_MOUNT_POINT"
    assert_env_var_dir_exists "PHASICJ_BUILD_REPO_MOUNT_POINT"
    assert_env_var_dir_exists "BAZEL_CACHE_MOUNT_POINT"
    assert_env_var_dir_exists "BAZELISK_CACHE_MOUNT_POINT"
    assert_env_var_dir_exists "MSMTP_CONFIG_MOUNT_POINT"

    # For each required mount point, check that the mounts point appears to be
    # mounted correctly by superficially checking its contents.
    file_path="$PHASICJ_EDGE_REPO_MOUNT_POINT/HEAD"
    if [ ! -f "$file_path" ]; then
        echo "ERROR: The PhasicJ edge Git repository doesn't contain an expected file: $file_path is missing. Was the PhasicJ edge repository volume mounted wrong?"
        exit 1
    fi

    file_path="$PHASICJ_BUILD_REPO_MOUNT_POINT/.phasicj_workspace_root"
    if [ ! -f "$file_path" ]; then
        echo "ERROR: The PhasicJ build Git repository doesn't contain an expected file: $file_path is missing. Was the PhasicJ build repository volume mounted wrong?"
        exit 1
    fi

    file_path="$BAZEL_CACHE_MOUNT_POINT/.bazel_cache_volume_root"
    if [ ! -f "$file_path" ]; then
        echo "ERROR: The Bazel cache directory doesn't contain an expected file: $file_path is missing. Was the Bazel cache volume mounted wrong?"
        exit 1
    fi

    file_path="$BAZELISK_CACHE_MOUNT_POINT/.bazelisk_cache_volume_root"
    if [ ! -f "$file_path" ]; then
        echo "ERROR: The Bazelisk cache directory doesn't contain an expected file: $file_path is missing. Was the Bazel cache volume mounted wrong?"
        exit 1
    fi

    file_path="$MSMTP_CONFIG_MOUNT_POINT/config"
    if [ ! -f "$file_path" ]; then
        echo "ERROR: The msmtp config directory doesn't contain an expected file: $file_path is missing. Was the msmtp config bind mount mounted wrong?"
        exit 1
    fi

    # Check that some users might be able login via SSH key.
    assert_file_exists "$SSH_AUTHORIZED_KEYS_FILE"
    assert_file_is_not_empty "$SSH_AUTHORIZED_KEYS_FILE"
}

check_test_environment

# Create any ssh host keys which don't already exist.
ssh-keygen -A

# Run the SSH server, but don't daemonize it (i.e. detach from the terminal).
/usr/sbin/sshd -D
