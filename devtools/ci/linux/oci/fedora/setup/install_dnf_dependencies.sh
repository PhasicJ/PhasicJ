#!/bin/sh -

set -e

dnf install --assumeyes \
    findutils \
    nodejs \
    colordiff \
    gcc-c++ \
    clang \
    java-latest-openjdk-devel \
    cargo \
    zlib-static \
    perl-Digest-SHA \
    mailx \
    msmtp \
    openssh-server \
    git \
    task-spooler

dnf clean all
