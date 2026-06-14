<template>
  <Layout :view="view">
    
    <Home v-if="view === 'Home'" @goto="homepage_selection" />
    <Song v-if="view === 'Now playing'" img="/sample/Foreigner-head-games-79.jpg" title="Head Games" artist="Foreigner" album="Head Games"></Song>

    <Music v-if="view === 'Music'" />
    <Photos v-if="view === 'Photos'" />
    <Videos v-if="view === 'Videos'" />
    <Extras v-if="view === 'Extras'" />
    <Settings v-if="view === 'Settings'" />

  </Layout>
</template>

<style>
* {
  -webkit-user-select: none;
  user-select: none;
}

html, body {
  color: white;
  overflow: hidden;
  
  font-family: 'Noto Sans';
  src: url('/fonts/NotoSans/NotoSans-Medium.ttf') format('truetype');
  font-weight: normal;
  font-style: normal;
}
</style>

<script setup>
import Layout from './layouts/default.vue';
import Home from './pages/home/index.vue';
import Song from './pages/song/index.vue';

import Music from './pages/music/index.vue';
import Photos from './pages/photos/index.vue';
import Videos from './pages/videos/index.vue';
import Extras from './pages/extras/index.vue';
import Settings from './pages/settings/index.vue';
</script>

<script>
export default {
  data() {
    return {
      view: "Home"
    }
  },
  methods: {
    homepage_selection(selection) {
      console.log("Navigating to", selection);
      this.view = selection;
    },
    handleKeyDown(event) {
      if (event.key === 'Escape') { // Escape Goes Home
        event.preventDefault();
        this.view = "Home";
      }
    }
  },
  mounted() {
    window.addEventListener('keydown', this.handleKeyDown);
  },
  beforeUnmount() {
    window.removeEventListener('keydown', this.handleKeyDown);
  },
}
</script>