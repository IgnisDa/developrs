export default {
  // Global page headers (https://go.nuxtjs.dev/config-head)
  head: {
    title: '{{cookiecutter.nuxt_project_name}}',
    meta: [
      { charset: 'utf-8' },
      { name: 'viewport', content: 'width=device-width, initial-scale=1' },
      { hid: 'description', name: 'description', content: '' },
    ],
    link: [{ rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }],
  },

  // Global CSS (https://go.nuxtjs.dev/config-css)
  css: ['~/assets/css/global.scss'],

  // Plugins to run before rendering page (https://go.nuxtjs.dev/config-plugins)
  plugins: [],

  // Auto import components (https://go.nuxtjs.dev/config-components)
  components: true,

  // Modules for dev and build (recommended) (https://go.nuxtjs.dev/config-modules)
  buildModules: [
    // https://go.nuxtjs.dev/eslint
    '@nuxtjs/eslint-module',
    // https://go.nuxtjs.dev/stylelint
    '@nuxtjs/stylelint-module',
    // https://image.nuxtjs.org
    '@nuxt/image',
    // https://typescript.nuxtjs.org
    '@nuxt/typescript-build',
    // https://typed-vuex.roe.dev
    'nuxt-typed-vuex',
    // https://windicss.org
    'nuxt-windicss',
  ],

  modules: [
    // https://go.nuxtjs.dev/pwa
    '@nuxtjs/pwa',
    // https://google-fonts.nuxtjs.org/
    '@nuxtjs/google-fonts',
    // https://www.npmjs.com/package/@nuxtjs/localtunnel
    process.env.ENABLE_LOCALTUNNEL === '1' && '@nuxtjs/localtunnel',
  ].filter(Boolean),

  // Build Configuration (https://go.nuxtjs.dev/config-build)
  build: {},
  googleFonts: {
    families: {
      Inter: true,
    },
    display: 'swap',
  },

  telemetry: false,
  target: 'static',
}
