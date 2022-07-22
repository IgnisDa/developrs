#!/usr/bin/env bash

# shellcheck disable=SC1091,SC2016

if [ -z "${APP_NAME-}" ]; then
  APP_NAME="cinder"
fi

. "$DIRECTORY/setup/commons.sh"

IMAGE_NAME="ignisda/cinder:latest"

info "Creating dokku app named '$APP_NAME'"
dokku apps:create "$APP_NAME"

info "Checking if postgres plugin is installed"
if ! dokku plugin:installed postgres; then
  warn "Postgres plugin not installed, doing it for you"
  dokku plugin:install https://github.com/dokku/dokku-postgres.git postgres
fi
if ! dokku postgres:exists "$APP_NAME"; then
  info "Creating dokku postgres service named '$APP_NAME'"
  dokku postgres:create "$APP_NAME"
fi

info "Pulling docker image"
docker pull "$IMAGE_NAME"

info "Linking app and database..."
dokku postgres:link "$APP_NAME" "$APP_NAME"

info "Setting docker options"
dokku docker-options:add "$APP_NAME" "run" '-e DATABASE_URL="$(dokku config:get $APP_NAME DATABASE_URL)"'

completed "Initializing docker container and starting service"
dokku git:from-image "$APP_NAME" "$(docker inspect --format='{{index .RepoDigests 0}}' $IMAGE_NAME)"
completed "Cinder was setup successfully!"
