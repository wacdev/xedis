#!/usr/bin/env bash

if [ -z $REDIS_HOST_PORT ]; then
  exit
fi

DIR=$(realpath $0) && DIR=${DIR%/*/*}
cd $DIR
set -ex

bunx ava
