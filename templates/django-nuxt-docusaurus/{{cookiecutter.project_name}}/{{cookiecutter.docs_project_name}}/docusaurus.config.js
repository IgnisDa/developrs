module.exports = {
  title: "{{ cookiecutter.project_name }} Documentation",
  tagline: "The tagline of my site",
  url: "https://{{ cookiecutter.github_username }}.github.io",
  baseUrl: "/{{ cookiecutter.project_name }}/",
  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",
  favicon: "img/favicon.ico",
  organizationName: "{{ cookiecutter.github_username }}",
  projectName: "{{ cookiecutter.project_name }}",
  themeConfig: {
    navbar: {
      title: "{{ cookiecutter.project_name }}",
      logo: {
        alt: "{{ cookiecutter.project_name }} Logo",
        src: "img/logo.svg",
      },
      items: [
        {
          href:
            "https://github.com/{{ cookiecutter.github_username }}/{{ cookiecutter.project_name }}",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      copyright: `Copyright © ${new Date().getFullYear()} {{ cookiecutter.github_name }}, Built with Docusaurus.`,
    },
  },
  presets: [
    [
      "@docusaurus/preset-classic",
      {
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          // Please change this to your repo.
          routeBasePath: "/",
          editUrl:
            "https://github.com/{{ cookiecutter.github_username }}/{{ cookiecutter.project_name }}/edit/main/{{cookiecutter.docs_project_name}}/",
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      },
    ],
  ],
};
