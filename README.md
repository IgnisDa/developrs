# developrs

A set of tools to provision development environments, and templates that make it easy to
bootstrap new projects.

## Packages

- [Esteem](./apps/esteem): Make your NX workspaces go easier on your disk
- [Cinder](./apps/cinder): A project to make deployment for dokku projects easier

## Docker development environments

The directory contains the configuration that is used to create the
[developr workspace](https://hub.docker.com/r/ignisda/developr-workspace). This image can
be used for creating
[remote development environments](https://code.visualstudio.com/docs/remote/containers).

It has a PostgreSQL database server running by default.

It has two images:

- Python and node projects
- Rust projects

Learn more [here](./docker).

## Templates

[Cookiecutter](https://cookiecutter.readthedocs.io/) templates that I use to
initialize new projects. The following templates exist for different kind of projects:

1. devcontainer

   This adds the required files for devcontainer configuration.

   ```bash
   cookiecutter gh:IgnisDa/developrs --directory="templates/devcontainer"
   ```
