#!/bin/sh
# this script just calls build script and runs a local python server for development.
./build.sh
echo "launching server using python http.server on http://127.0.0.1:8000/"
(cd dist && python -m http.server --bind 127.0.0.1)

