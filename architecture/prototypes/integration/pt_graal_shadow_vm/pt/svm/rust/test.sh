#!/bin/sh -

set -e

export EXEC_PATH="$1"

"./${EXEC_PATH}" | tee stdout.log

grep 'Hello, from `pt.svm.Analysis`, implemented in Java.' stdout.log
