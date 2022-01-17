#!/usr/bin/env bash

# shellcheck disable=SC1091,SC2086

. "$DIRECTORY/setup/commons.sh"

if ! dokku apps:exists "$APP_NAME"; then
    error "Dokku app $APP_NAME does not exist"
    info "To setup Cinder, run this script without any arguments"
    exit 1
fi

info "Updating your cinder installation"
warn "Removing docker image $IMAGE_NAME"
docker rmi "$IMAGE_NAME" || true

info "Downloading new image"
docker pull "$IMAGE_NAME"

warn "Updating app"
dokku git:from-image "$APP_NAME" "$(docker inspect --format='{{index .RepoDigests 0}}' $IMAGE_NAME)"
completed "Cinder was updated successfully!"
