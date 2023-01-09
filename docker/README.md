# docker configuration

The [`Dockerfile`](./Dockerfile) is used to build the image for the devcontainers
development environment.

## Building

To build the image, execute the following command in this directory:

```bash
docker build --build-arg BUILD_DATE=(date -u +'%Y-%m-%dT') --file docker/Dockerfile --tag ignisda/archlinux:latest .
```
