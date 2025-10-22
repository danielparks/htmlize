#!/bin/bash

set -eo pipefail
shopt -s extglob

version=$1
branch_name=$(git rev-parse --abbrev-ref HEAD)

awk-in-place () {
  local tmpfile=$(mktemp)
  local original="$1"
  shift
  cp "$original" "$tmpfile"
  awk "$@" <"$tmpfile" >"$original"
  rm "$tmpfile"
}

check-changes () {
  if git ls-files --exclude-standard --other | grep . >/dev/null ; then
    echo 'Found untracked files:' >&2
    git ls-files --exclude-standard --other | sed -e 's/^/  /' >&2
    echo >&2
    echo 'Please commit changes before proceeding.' >&2
    return 1
  fi

  git diff --color --exit-code HEAD || {
    echo >&2
    echo 'Please commit changes before proceeding.' >&2
    return 1
  }
}

auto-pr () {
  pr_url=$((gh pr view --json url,closed 2>/dev/null || true) \
    | jq -r 'select(.closed | not) | .url')

  if [[ "$pr_url" ]] ; then
    echo "Found existing PR: $pr_url"
    echo
  else
    # Create a PR
    gh pr create --fill-verbose --title "$1"
  fi

  gh pr merge --disable-auto --delete-branch
  sleep 3
  gh pr checks --watch --fail-fast

  git checkout main
  git pull
  git merge --ff-only "$branch_name"
  git push origin HEAD

  git branch -d "$branch_name"
  git push origin --delete "$branch_name"
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
  +([0-9]).+([0-9]).+([0-9])*) ;; # Good
  *) echo "Usage $0 VERSION" >&2 ; exit 1 ;;
esac

command -v gh &>/dev/null || {
  echo "gh not installed (https://cli.github.com)" >&2
  exit 1
}

command -v jq &>/dev/null || {
  echo "jq not installed (https://jqlang.org)" >&2
  exit 1
}

command -v parse-changelog &>/dev/null || {
  echo "parse-changelog not installed (https://github.com/taiki-e/parse-changelog)" >&2
  exit 1
}

if [[ "$branch_name" = main ]] ; then
  git switch -c "release-$version"
fi

check-changes

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

# Commit version bump if necessary.
check-changes &>/dev/null || {
  git add -u
  git commit --cleanup=verbatim --file - <<EOF
Release ${version}

$(parse-changelog CHANGELOG.md "$version")
EOF
}

check-changes

git tag --force --sign --file "$changelog" --cleanup=verbatim "v${version}"
git push --force --tags origin HEAD
auto-pr "Release ${version}"

cargo publish

awk-in-place CHANGELOG.md '
  /^## Release/ && !done {
    print "## main branch\n"
    done=1
  }
  { print }'

git switch -c post-release
git add CHANGELOG.md
git diff --staged

confirm 'Commit with message "Prepping CHANGELOG.md for development."?'

git commit -m 'Prepping CHANGELOG.md for development.'
git push --force origin HEAD
auto-pr "Prepping CHANGELOG.md for development"
