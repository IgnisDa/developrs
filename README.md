# developrs

This is my [cookiecutter](https://cookiecutter.readthedocs.io/) template that I use to
initialize new projects. The following templates exist for different kind of projects:

1. devcontainer

   This adds the required files for devcontainer configuration.

   ```bash
   cookiecutter gh:IgnisDa/developrs --directory="templates/devcontainer"
   ```

1. django-nuxt-docusaurus

   This has sets up a fullstack application, with a django backend, nuxtjs frontend and
   a docusaurus documentation. The nuxtjs and docusaurus dependencies are managed by yarn
   and the django backend is managed using poetry.

   ```bash
   cookiecutter gh:IgnisDa/developrs --directory="templates/django-nuxt-docusaurus"
   ```

1. nuxt

   This will setup a NuxtJS project with tailwind CSS support.

   ```bash
   cookiecutter gh:IgnisDa/developrs --directory="templates/nuxt"
   ```

1. django

   This will setup a django project with PostgreSQL database support.

   ```bash
   cookiecutter gh:IgnisDa/developrs --directory="templates/django"
   ```

1. rust

   This will setup a rust project, with PostgreSQL and vagrant support.

   ```bash
   cookiecutter gh:IgnisDa/developrs --directory="rust"
   ```
