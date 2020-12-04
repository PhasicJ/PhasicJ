#!/usr/bin/env bash

PHASICJ_CI_TASK_ID="$1"
if [ -z "$PHASICJ_CI_TASK_ID" ]; then
    echo "ERROR: Expected to get a PhasicJ CI Task ID as an argument, but none given."
    exit 1
fi

# NOTE(dwtj): We don't want to inherit the GIT_DIR value from when this task
# was added to task-spooler.
unset GIT_DIR

# TODO(dwtj): Fix this hard-coded value.
cd "/Users/phasicj-ci/phasicj.build"

git fetch edge 2> /dev/null

git checkout "ci/$PHASICJ_CI_TASK_ID"

echo
echo '# bazel build //... ####################################################'
echo
bazel build //...
exit_code=$?
if [ $exit_code != "0" ]; then
    echo "ERROR: Bazel build failed!"
    exit $exit_code
fi
echo
echo '# bazel test //... #####################################################'
[ $? ] || exit $?
echo
bazel test //...
exit_code=$?
if [ $exit_code != "0" ]; then
    echo "ERROR: Bazel test failed!"
    exit $exit_code
fi
