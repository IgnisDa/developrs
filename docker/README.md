# docker configuration

There are two images in this directory. The [`Dockerfile`](./Dockerfile) contains the base
`developr-workspace`. The [`rust.Dockerfile`](./rust.Dockerfile) contains the image for the
`developr-workspace:rust`.

To build the image, execute the following command in this directory:

**Note**: Replace `<VERSION>` with the correct version number, for example `0.1.1`.

## Base

- To build the image:

    ```bash
    docker build --build-arg BUILD_DATE=(date -u +'%Y-%m-%dT') --build-arg VCS_REF=(git rev-parse HEAD) --build-arg BUILD_VERSION="<VERSION>" -t ignisda/developr-workspace:latest .
    ```

- To push the image:

    ```bash
    docker push ignisda/developr-workspace:latest
    ```

## Rust

- To build the image:

    ```bash
    docker build --build-arg BUILD_DATE=(date -u +'%Y-%m-%dT') --build-arg VCS_REF=(git rev-parse HEAD) --build-arg BUILD_VERSION="<VERSION>" -t ignisda/developr-workspace:rust --file rust.Dockerfile .
    ```

- To push the image:

    ```bash
    docker push ignisda/developr-workspace:rust
    ```
