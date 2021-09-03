#!/bin/bash

# setup the git identities
git config --global user.name "${GIT_AUTHOR_NAME}"
git config --global user.email "${GIT_AUTHOR_EMAIL}"

# remove the default fish shell prompt
set fish_greeting

# own the HOME cache directory
sudo chown -R $USERNAME:$USERNAME $HOME/.cache

# Install the dependencies of each micro-service
PROJECT_FOLDER="/workspace/{{ cookiecutter.project_name }}"
cd "$PROJECT_FOLDER"

for filename in $PROJECT_FOLDER/{{ cookiecutter.devcontainer_folder_name }}/scripts/private-*; do
    [ -e "$filename" ] || continue
    . "$filename"
done
