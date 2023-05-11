#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
source sh/pid.sh
set -ex

if [ ! -d "node_modules" ]; then
  yarn install
fi

bunx concurrently --kill-others --raw -- \
  "watchexec -n --project-origin . -w ./test --exts coffee,lua -- direnv exec . ./test/lua.coffee"
