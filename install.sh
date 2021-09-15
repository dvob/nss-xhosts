#!/usr/bin/env bash

set -o errexit

cargo build --release
cp target/release/libnss_xhosts.so libnss_xhosts.so.2
strip libnss_xhosts.so.2
sudo install -m 0644 libnss_xhosts.so.2 /lib
sudo /sbin/ldconfig -n /lib /usr/lib
