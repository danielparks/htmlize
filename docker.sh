#!/bin/bash

# Usage: ./docker.sh cargo bench --features iai iai
# Interactive usage: ./docker.sh

set -eo pipefail

if [[ $# == 0 ]] ; then
  command=(bash)
else
  command=("$@")
fi

image=dp-rust:0.3

if ! docker inspect "$image" >/dev/null ; then
  docker build -t "$image" .
fi

docker run -ti --rm --privileged -v $(pwd):/work -w /work "$image" "$@"
