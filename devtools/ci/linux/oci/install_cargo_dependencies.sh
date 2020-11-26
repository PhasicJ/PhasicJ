#!/bin/sh -

cargo install bindgen

# Add `bindgen` to a directory on the system PATH.
mkdir -p /root/bin
ln -s /root/.cargo/bin/bindgen /root/bin/bindgen
