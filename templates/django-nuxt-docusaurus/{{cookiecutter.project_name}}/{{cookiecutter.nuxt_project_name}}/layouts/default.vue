<template>
  <div :class="mode !== 'production' ? 'debug-screens' : ''">
    <Nuxt />
  </div>
</template>

<script>
export default {
  data: () => ({
    mode: process.env.NODE_ENV,
  }),
}
</script>
