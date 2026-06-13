<template>
  <Layout :view="view">
    
    <Song v-if="view === 'Music'" img="/sample/Foreigner-head-games-79.jpg" title="Head Games" artist="Foreigner" album="Head Games"></Song>

  </Layout>
</template>

<style>
* {
  -webkit-user-select: none;
  user-select: none;
}

html, body {
  color: white;
  /* background-color: #333; */
  overflow: hidden;
}
</style>

<script>
import Layout from './components/Layout.vue';
import Song from './components/song/index.vue';

export default {
  components: { Layout, Song },
  data() {
    return {
      view: "Music"
    }
  }
}
</script>