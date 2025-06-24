#!/usr/bin/env bash
# This scripts runs various CI-like checks in a convenient way.
set -eux

trunk build
cd dist
python3 -m http.server
