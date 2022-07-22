# cinder

Deploy your [Dokku][dokku] projects easier and predictably.

- [cinder](#cinder)
  - [Usage](#usage)
    - [Endpoints](#endpoints)
  - [Contributing](#contributing)

## Usage

- Get the last commit SHA that deployed `my-app`.

  ```bash
  $ curl -X GET https://actions.domain.xyz/my-app
  {
      "id": 2,
      "name": "my-app",
      "sha": "ec372650e6b67fad37b4e5abcccafbc32125c2ba",
      "executed_at": "2022-01-15T08:29:46.338880"
  }
  ```

  It returns a 404 error if that app does not exist.

- Inform `cinder` that `my-app` has been deployed with this SHA. Make sure you pass the
  entire SHA string, so that referencing it is easier in the future.

  ```bash
  $ curl -X POST https://actions.domain.xyz/my-app/update \
      -H "Content-type: application/json" \
      --data '{ "sha": "8f0b1fc04bbe45b148f4ef4acb7e49da2d1a5dd7" }'
  {
      "status": true,
      "id": 2,
      "created_at": "2022-01-13T08:29:46.338880"
  }
  ```

  If `my-app` does not exist in the database, it will be created.

### Endpoints

## Contributing

Your PRs and stars are always welcome.

[dokku]: https://dokku.com/
[httpie]: https://httpie.io/
