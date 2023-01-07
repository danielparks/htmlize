#!/bin/bash

set -eo pipefail
shopt -s extglob

version=$1

awk-in-place () {
  local tmpfile=$(mktemp)
  local original="$1"
  shift
  cp "$original" "$tmpfile"
  awk "$@" <"$tmpfile" >"$original"
  rm "$tmpfile"
}

confirm () {
  local prompt="$1"
  local answer
  read -n 1 -p "${prompt} [yN] " answer
  case $answer in
    [yY]*) echo ;; # Continue
    *) echo ; echo Canceling. >&2 ; exit 1 ;;
  esac
}

case $version in
  +([0-9]).+([0-9]).+([0-9])) ;; # Good
  *) echo "Usage $0 VERSION" >&2 ; exit 1 ;;
esac

echo 'Making sure version is correct.'

awk-in-place Cargo.toml '
  /^version *=/ && !done {
    sub(/"[0-9.]+"/, "\"'$version'\"")
    done=1
  }
  { print }'

cargo check --quiet

awk-in-place CHANGELOG.md '
  /^## / && !done {
    $0 = "## Release '$version' ('$(date +%Y-%m-%d)')"
    done=1
  }
  { print }'

# Check for changes
if git ls-files --exclude-standard --other | grep . >/dev/null ; then
  echo 'Found untracked files:' >&2
  git ls-files --exclude-standard --other | sed -e 's/^/  /' >&2
  echo >&2
  echo 'Please commit changes before proceeding.' >&2
  exit 1
fi

git diff --color --exit-code HEAD || {
  echo >&2
  echo 'Please commit changes before proceeding.' >&2
  exit 1
}

# Confirm changelog
changelog=$(mktemp)
{
  echo "## Release ${version}"
  echo
  parse-changelog CHANGELOG.md "$version"
} >"$changelog"

cat "$changelog"
echo
confirm 'Release notes displayed above. Continue?'

git tag --sign --file "$changelog" --cleanup=verbatim "v${version}"
git push --tags origin main

awk-in-place CHANGELOG.md '
  /^## Release/ && !done {
    print "## main branch\n"
    done=1
  }
  { print }'

git add CHANGELOG.md
git diff --staged

confirm 'Commit with message "Prepping CHANGELOG.md for development."?'

git commit -m 'Prepping CHANGELOG.md for development.'
