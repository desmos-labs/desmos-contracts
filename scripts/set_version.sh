#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck > /dev/null && shellcheck "$0"

function print_usage() {
    echo "Usage: $0 NEW_VERSION"
    echo ""
    echo "e.g. $0 0.2.0"
}

if [ "$#" -ne 1]; then
  print_usage
  exit 1
fi

# Check repo
SCRIPT_DIR="$(realpath "$(dirname "$0")")"
if [[ "$(realpath "$SCRIPT_DIR/..")" != "$(pwd)"]]; then
  echo "Script must be called from the repo root"
  exit 2
fi

# Ensure repo is not dirty
CHANGES_IN_REPO=$(git status --porcelain)
if [[ -n "$CHANGES_IN_REPO" ]]; then
    echo "Repository is dirty. Showing 'git status' and 'git --no-pager diff' for debugging now:"
    git status && git --no-pager diff
    exit 3
fi

