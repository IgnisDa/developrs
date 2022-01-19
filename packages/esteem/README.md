# esteem

Make your [NX](https://nx.dev/) workspaces go easier on your disk.

- [esteem](#esteem)
  - [Why?](#why)
  - [How does it work?](#how-does-it-work)
  - [Installation](#installation)
  - [Usage](#usage)
    - [`init`](#init)
    - [`add`](#add)
    - [`remove`](#remove)
    - [`install-isolated`](#install-isolated)
      - [Some caveats](#some-caveats)
  - [Example](#example)
  - [Contributing](#contributing)

## Why?

This tool exists for a very simple reason - I am a :sparkles: cheapskate :sparkles:. I
would rather spend a weekend developing a tool than pay an additional $5 on Digital Ocean
to buy some additional storage and calling it a day.

I present you with **_esteem_**!

NX monorepos totally rock, but where they don't is when you need to deploy your projects
and now a simple Next app takes up 2GBs of storage because you also had to install
dependencies of all other projects in the monorepo. `esteem` solves this problem by keeping
track of each individual project's dependencies while also following NX's single-version
policy.

## How does it work?

It just keeps track of individual project's dependencies. For existing projects this means
a small amount of manual work where you copy the dependencies of a project from
`package.json` to its `project.json`.

When [`install-isolated`](#install-isolated) is called for a project, it simple collects
the dependencies for that package and writes it to the root `package.json`. You can then
call your package manager's install command and viola! you have a smaller `node_modules`!

## Installation

- Guided installation:

  ```bash
  curl https://raw.githubusercontent.com/IgnisDa/developrs/main/packages/esteem/install.sh -o install.sh
  # Warning: always examine scripts downloaded from the internet before running them locally.
  bash install.sh
  ```

- Manual installation:

  You can also download the appropriate executable from the
  [releases](https://github.com/IgnisDa/developrs/releases) page.

- CI environments:
  For example in `Dockerfile`

  ```Dockerfile
  # This script writes to /usr/local/bin/, you can change this via the `--bin-dir` flag
  RUN curl https://raw.githubusercontent.com/IgnisDa/developrs/main/packages/esteem/install.sh | sudo sh -s -- --yes
  ```

## Usage

`esteem` has very few commands of it's own, most of the heavy lifting is done by your
package manager (npm, yarn, pnpm etc).

Right now `esteem` is only compatible with NX monorepos (or all projects where there is a
root level `workspace.json` and project level configurations in
`<project_type>/<project_name>/project.json`).

**_Note_:** You can always run `esteem <subcommand> --help` for more information.

### `init`

Prepares new repositories to be used with `esteem`. This will add an object of this
structure to all `project.json` files.

```json
{
  "dependencies": {
    "development": [],
    "required": []
  }
}
```

Now for each `project.json`, you will have to manually add the dependent packages for that
project. So if project `server` depends on `nestjs` and `prettier`, you will have to make
these changes.

Eg: `apps/server/project.json`

```json
{
  "dependencies": {
    "development": ["prettier"],
    "required": ["nestjs"]
  }
}
```

[Here](https://github.com/IgnisDa/bookius/blob/main/apps/server/project.json) is a
`project.json` from the [example](#example) repository.

### `add`

It adds a dependency to a project. Eg:

```bash
$ esteem add server redis luxon
[INFO ] Dependency "redis" added to "apps/server/project.json".
[WARN ] Dependency "luxon" already exists in "apps/server/project.json". Skipping...
[INFO ] Writing new workspace dependencies to "apps/server/project.json"
[INFO ] Installing package(s) ["redis", "luxon"] for you
# your package manager called automatically here
```

Pass the `-D` flag to add it a development dependency.

### `remove`

Removes a dependency from a project taking into account whether other projects are
dependent on that dependency.

```bash
$ esteem remove server luxon typescript bull
[INFO ] Writing new workspace dependencies to "apps/server/project.json"
[WARN ] Found "luxon" in site's "required", won't be removing it!
[WARN ] Found "typescript" in site's "development", won't be removing it!
# your package manager called automatically here
```

### `install-isolated`

This command is meant to be run only on CI environments because it changes your
`package.json` file. It makes a backup of the `package.json` file and then rewrites the
dependencies of the project into `package.json`. It is your job to call your package
manager to install the dependencies.

```bash
esteem install-isolated server
```

#### Some caveats

- The lockfile will **NOT** be in sync with `package.json` because `esteem` does not
  resolve dependencies. However this should not be a problem since dependencies have only
  been _removed_ and not _added_ (and they were already resolved in the lockfile). However,
  this means that `pnpm install --frozen-lockfile` and similar commands **WILL** fail. Just
  remove that flag and it should work as expected.

- `esteem` can not solve cross project dependencies. For example, project `server`
  depends on project `model`, and both project's dependencies must be installed for `server`
  to build (as in the [example repository](#example)). Instead you can call it for multiple
  projects:

  ```bash
  esteem install-isolated server model
  ```

- If you run the command `esteem install-isolated server`, the following files are expected to be
  present (with the paths intact):

  - `package.json`
  - `pnpm-lock.yaml` (or your package manager's lockfile)
  - `workspace.json`
  - `apps/server/project.json`

  You can refer to
  [these lines](https://github.com/IgnisDa/bookius/blob/49713a5d0beb1528d471563faf565cabbbbe4ff5/apps/server/Dockerfile#L4-L5)
  of the [example](#example) repository to see this in action in a `Dockerfile`.

## Example

[Bookius](https://github.com/IgnisDa/bookius) is a project where `esteem` is used in
conjunction with Dokku, Github Actions and Docker for deployment. All projects therein are
deployed from a single Digital Ocean droplet.

You can consult
[`Dockerfile`](https://github.com/IgnisDa/bookius/blob/49713a5d0beb1528d471563faf565cabbbbe4ff5/apps/server/Dockerfile#L8)
to see it being used in a docker environment.

## Contributing

Your PRs and stars are always welcome.
