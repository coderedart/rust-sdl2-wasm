#!/bin/sh
# this script just calls build script and runs a local python server for development.
set -eu
./build.sh
echo "launching server using python http.server "
(cd dist && python -m http.server)

