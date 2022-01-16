#!/usr/bin/env bash

# shellcheck disable=SC2039,SC2016

printf '\n'

if [ -z "${APP_NAME-}" ]; then
  APP_NAME="cinder"
fi

IMAGE_NAME="ignisda/cinder:latest"

BOLD="$(tput bold 2>/dev/null || printf '')"
GREY="$(tput setaf 0 2>/dev/null || printf '')"
RED="$(tput setaf 1 2>/dev/null || printf '')"
GREEN="$(tput setaf 2 2>/dev/null || printf '')"
YELLOW="$(tput setaf 3 2>/dev/null || printf '')"
NO_COLOR="$(tput sgr0 2>/dev/null || printf '')"

info() {
  printf '%s\n' "${BOLD}${GREY}>${NO_COLOR} $*"
}

warn() {
  printf '%s\n' "${YELLOW}! $*${NO_COLOR}"
}

error() {
  printf '%s\n' "${RED}x $*${NO_COLOR}" >&2
}

completed() {
  printf '%s\n' "${GREEN}✓${NO_COLOR} $*"
}


if [ "$1" == "update"  ]; then

  if ! dokku apps:exists $APP_NAME; then
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

else

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

fi
