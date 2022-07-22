#!/usr/bin/env bash

# shellcheck disable=SC1091,SC2086

export DIRECTORY=/tmp/developrs
rm -rf $DIRECTORY
mkdir -p $DIRECTORY

curl -L https://api.github.com/repos/IgnisDa/developrs/tarball \
    | tar -xz --strip-components=1 -C $DIRECTORY

. "$DIRECTORY/setup/commons.sh"


if [ -z "$1" ]; then
    error "You must specify one of the arguments to this script: install, update"
    exit 1
elif [ "$1" == "install" ]; then
    info "Running install command"
    $DIRECTORY/packages/cinder/setup/install.sh
elif [ "$1" == "update" ]; then
    info "Running update command"
    $DIRECTORY/packages/cinder/setup/update.sh
fi
