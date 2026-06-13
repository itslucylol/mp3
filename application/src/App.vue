<template>
  <Layout :view="view">
    
    <Home v-if="view === 'Home'"></Home>
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
  
  font-family: 'MyCustomFont';
  src: url('/fonts/NotoSans/NotoSans-Medium.ttf') format('truetype');
  font-weight: normal;
  font-style: normal;
}
</style>

<script>
import Layout from './layouts/default.vue';
import Home from './pages/home/index.vue';
import Song from './pages/song/index.vue';

export default {
  components: { Layout, Song, Home },
  data() {
    return {
      view: "Home"
    }
  }
}
</script>