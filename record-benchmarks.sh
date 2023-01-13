#!/bin/bash

set -eo pipefail

amend=
if [[ $1 == "--amend" ]] ; then
  amend=1
  shift
fi

# Helpful to know what we’re benchmarking, especially when run with:
#     git rebase -x "./record-benchmarks.sh --amend" main
git show --color --stat HEAD | cat

cargo criterion --features "$*"

# --target-dir doesn’t seem to work, so we just track parts of target/.
mkdir -p target/iai
./docker.sh cargo bench --quiet --features "iai $*" iai \
  | tee target/iai/iai-output.txt

git add -f target/criterion target/iai

if [[ -z "$amend" ]] ; then
  git commit -m "Update recorded benchmarks." target/criterion target/iai
else
  git commit --amend --no-edit target/criterion target/iai
fi
