<template>
  <Layout :view="view">
    <p>Test</p>
  </Layout>
</template>

<style>
html, body {
  color: white;
  background-color: #333;
}
</style>

<script>
import Layout from './components/Layout.vue';

export default {
  components: { Layout },
  data() {
    return {
      view: "Music"
    }
  }
}
</script>