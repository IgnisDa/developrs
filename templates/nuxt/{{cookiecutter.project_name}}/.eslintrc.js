module.exports = {
  root: true,
  env: {
    browser: true,
    node: true,
  },
  parserOptions: {},
  extends: [
    '@nuxtjs',
    'prettier',
    'prettier/vue',
    '@nuxtjs/eslint-config-typescript',
    'plugin:prettier/recommended',
    'plugin:nuxt/recommended',
  ],
  plugins: ['prettier'],
  // add your custom rules here
  rules: {
    'vue/singleline-html-element-content-newline': 'off',
    'vue/html-self-closing': [
      'error',
      {
        html: {
          void: 'any',
        },
      },
    ],
  },
}
