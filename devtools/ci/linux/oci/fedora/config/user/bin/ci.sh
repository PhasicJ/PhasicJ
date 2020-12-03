#!/usr/bin/env bash

PHASICJ_CI_TASK_ID="$1"
if [ -z "$PHASICJ_CI_TASK_ID" ]; then
    echo "ERROR: Expected to get a PhasicJ CI Task ID as an argument, but none given."
    exit 1
fi

set -e

# NOTE(dwtj): We don't want to inherit the GIT_DIR value from when this task
# was added to task-spooler.
unset GIT_DIR

# TODO(dwtj): Figure out how to use $PHASICJ_BUILD_REPO_DIR here instead of
# this hard-coded value.
cd "/root/phasicj.build"

git fetch edge 2> /dev/null

git checkout "ci/$PHASICJ_CI_TASK_ID"

echo
echo '# bazel build //... ####################################################'
echo
bazel build //...
echo
echo '# bazel test //... #####################################################'
echo
bazel test //...
