<template>
    <div class="song">
        <div class="hero">
            <img :src="img" />
            <div class="info">
                <p class="title">{{ title }}</p>
                <p class="artist">{{ artist }}</p>
                <p class="album">{{ album }}</p>
            </div>
        </div>
        <fullscreen_blur :img="img" />
    </div>
</template>

<style scoped>
.song {
    margin: 0.25rem;
}
.hero {
    display: flex;
    gap: 1rem;
}
.hero > img {
    width: 6rem;
    height: 6rem;
    border-radius: 1rem;
}
.info > p {
    margin: 0;
    margin-bottom: 0.5rem;
    padding: 0;
}

.info > .artist {
    opacity: 75%;
}
.info > .album {
    opacity: 50%;
}
</style>

<script setup>
import fullscreen_blur from './fullscreen_blur.vue';
</script>

<script>
export default {
    props: ['img', 'title', 'artist', 'album']
}
</script>