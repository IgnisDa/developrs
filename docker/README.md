# docker configuration

There are two images in this directory. The [`Dockerfile`](./Dockerfile) contains the base
`developr-workspace`. The [`rust.Dockerfile`](./rust.Dockerfile) contains the image for the
`developr-workspace:rust`.

## Building

To build the image, execute the following command in this directory:

**Note**: Replace `<VERSION>` with the correct version number, for example `0.1.1`.

### Base

- To build the image:

  ```bash
  docker build --build-arg BUILD_DATE=(date -u +'%Y-%m-%dT') --build-arg BUILD_VERSION="<VERSION>" -t ignisda/developr-workspace:latest .
  ```

- To push the image:

  ```bash
  docker push ignisda/developr-workspace:latest
  ```

### Rust

- To build the image:

  ```bash
  docker build -t ignisda/developr-workspace:rust --file rust.Dockerfile .
  ```

- To push the image:

  ```bash
  docker push ignisda/developr-workspace:rust
  ```

## Data persistence

If you want to persist the data, make sure you mount the
`/workspace/.postgresql/pgsql/data` directory.

[Here](https://github.com/IgnisDa/learning/blob/main/.devcontainer/docker-compose.yml) is
an example.

## Adding redis

Just add the following to `.devcontainer/Dockerfile`

```Dockerfile
RUN sudo apt-get update && sudo apt-get upgrade -y ;\
    sudo apt-get install -y redis-server ;\
    sudo rm -rf /var/lib/apt/lists/* ;\
    echo "[[ \$(redis-cli ping) ]] || redis-server > /dev/null" | sudo tee -a "/etc/bash.bashrc" ;
```
