#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck > /dev/null && shellcheck "$0"

# Packages (not used atm)
# ALL_PACKAGES="<packages-name>"

ALL_CONTRACTS="filter-posts"

SLEEP_TIME=30

#
#for pack in $ALL_PACKAGES; do
#  (
#    cd "packages/$pack"
#    echo "Publishing $pack"
#    cargo publish
#  )
#done

# wait for these to be processed on crates.io
# echo "Waiting for publishing al packages"
# sleep $SLEEP_TIME

for cont in $ALL_CONTRACTS; do
  (
    cd "contracts/$cont"
    echo "Publishing $cont"
    cargo publish
  )
done

echo "Everything is published!"
