# docker configuration

The [`Dockerfile`](./Dockerfile) is used to build the image for the devcontainers
development environment.

## Building

To build the image, execute the following command in this directory:

```bash
docker build --build-arg BUILD_DATE=(date -u +'%Y-%m-%dT') --file docker/Dockerfile --tag ignisda/archlinux:latest .
```

## Notes

- The image has the default timezone set to "Asia/Kolkata". You might want to change it to
  your local timezone. This can be done by changing the value of the `TZ` environment
  variable.
