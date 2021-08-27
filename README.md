# developrs

A set of tools to provision development environments, and templates that make it easy to
bootstrap new projects.

## Docker development environment

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

2. django-nuxt-docusaurus

   This has sets up a fullstack application, with a django backend, nuxtjs frontend and
   a docusaurus documentation. The nuxtjs and docusaurus dependencies are managed by yarn
   and the django backend is managed using poetry.

   ```bash
   cookiecutter gh:IgnisDa/developrs --directory="templates/django-nuxt-docusaurus"
   ```

3. nuxt

   This will setup a NuxtJS project with tailwind CSS support.

   ```bash
   cookiecutter gh:IgnisDa/developrs --directory="templates/nuxt"
   ```

4. django

   This will setup a django project with PostgreSQL database support.

   ```bash
   cookiecutter gh:IgnisDa/developrs --directory="templates/django"
   ```

5. rust

   This will setup a rust project, with PostgreSQL and vagrant support.

   ```bash
   cookiecutter gh:IgnisDa/developrs --directory="rust"
   ```
