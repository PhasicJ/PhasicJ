#!/bin/sh -

set -e

BAZELISK_URL="$1"
BAZELISK_SHA256_FILE="$2"

BAZELISK_FILE="bazelisk-linux-amd64"

# Download a copy of the Bazelisk executable and check its SHA256 hash.
curl --location -o "$BAZELISK_FILE" "$BAZELISK_URL"
sha256sum --check "$BAZELISK_SHA256_FILE"

# Put Bazelisk in a directory on the system PATH, renaming it to `bazel`.
mkdir -p /root/bin
cp "$BAZELISK_FILE" /root/bin/bazel
chmod u+x /root/bin/bazel
