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
        echo "ERROR: Environment variable's value is not a directory: ${VAR_NAME}=${VAR_VALUE}"
        exit 1
    fi
}

check_test_environment()
{
    # Check that container volume mounts appear to be mounted.
    assert_env_var_dir_exists "PHASICJ_WORKSPACE_DIR"
    assert_env_var_dir_exists "BAZEL_CACHE_DIR"
    assert_env_var_dir_exists "BAZELISK_CACHE_DIR"

    # Check that the PhasicJ volume mount appears to be the root of the project.
    if [ ! -f "$PHASICJ_WORKSPACE_DIR/.phasicj_workspace_root" ]; then
        echo "ERROR: The PhasicJ workspace directory doesn't contain an expected file: $PHASICJ_WORKSPACE_DIR/.phasicj_workspace_root is missing. Was the workspace directory mounted wrong?"
    fi

    # TODO(dwtj): Check that the Bazel cache directory appears to be mounted correctly.

    # TODO(dwtj): Check that the Bazelisk cache directory appears to be mounted correctly.
}

check_test_environment
