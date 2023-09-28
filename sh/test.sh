#!/usr/bin/env sh

if [ -z $KV_HOST_PORT ]; then
  exit
fi

DIR=$(realpath $0) && DIR=${DIR%/*/*}
cd $DIR
set -ex

bun x ava --verbose
