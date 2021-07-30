#!/usr/bin/env bash

# setup the git identities
git config --global user.name "${GIT_AUTHOR_NAME}"
git config --global user.email "${GIT_AUTHOR_EMAIL}"

# remove the default fish shell prompt
set fish_greeting

# Install the dependencies of each micro-service
PROJECT_FOLDER="/workspace/{{cookiecutter.nuxt_project_name}}"
cd "$PROJECT_FOLDER"
yarn install --frozen-lockfile

for filename in $PROJECT_FOLDER/.devcontainer/scripts/private-*; do
    [ -e "$filename" ] || continue
    source "$filename"
done
