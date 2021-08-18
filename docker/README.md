# docker configuration

To build the image, execute the following command in this directory:

```bash
docker build --build-arg BUILD_DATE=(date -u +'%Y-%m-%dT%H:%M:%SZ') --build-arg VCS_REF=(git rev-parse HEAD) --build-arg BUILD_VERSION="<VERSION>" -t ignisda/developr-workspace:latest .
```

**Note**: Replace `<VERSION>` with the correct version number, for example `0.1.1`.
