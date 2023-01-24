#!/bin/bash

# Usage: ./docker.sh cargo bench --features iai iai
# Interactive usage: ./docker.sh

set -eo pipefail

# cd to script directory
cd -P -- "$(dirname -- "${BASH_SOURCE[0]}")"

image=dp-$(basename $(pwd)):0.1

if [[ $1 == "--rebuild" ]] ; then
  shift
  docker image rm -f "$image"
fi

if [[ $# == 0 ]] ; then
  command=(bash)
else
  command=("$@")
fi

if ! docker inspect "$image" >/dev/null ; then
  docker build -t "$image" .
fi

docker run -ti --rm --privileged -v $(pwd):/work -w /work "$image" "$@"
