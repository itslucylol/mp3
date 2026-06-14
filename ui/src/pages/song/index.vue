<template>
    <div class="song">
        <div class="hero">
            <img class="hero-image" :src="img" alt="Cover Art" />
            <div class="info">
                <p class="title">{{ title }}</p>
                <p class="artist">{{ artist }}</p>
                <p class="album">{{ album }}</p>
            </div>
        </div>
        <div class="progress-content progress-content-top">
            <span>{{ currentTime }}</span>
            <span class="progress-content-spacer"></span>
            <span>{{ totalTime }}</span>
        </div>
        <progress class="progress" />
        <div class="progress-content progress-content-bottom">
            <Icon class="audio-device-icon" name="headphones" size="20x20" />
            <span class="audio-device">Lucy's Headphones</span>
        </div>
        <Wallpaper :img="img" />
    </div>
</template>

<style scoped>
.song {
    margin: 0.25rem;
}
.hero {
    display: flex;
}
.hero-image {
    width: 6rem;
    height: 6rem;
    border-radius: 1rem;
}

.info {
    margin-left: 1rem;
}
.info > p {
    margin: 0;
    margin-bottom: 0.5rem;
    padding: 0;
}
.info > .artist {
    opacity: 75%;
    font-size: 0.8rem;
}
.info > .album {
    opacity: 50%;
    font-size: 0.8rem;
}

.progress {
    width: 100%;
}
.progress-content {
    display: flex;
}
.progress-content-top {
    transform: translateY(0.75rem);
}
.progress-content-bottom {
    margin-top: 1rem;
}
.progress-content-spacer {
    margin: auto;
}

.audio-device {
    margin-left: 0.5rem;
    opacity: 75%;
    font-size: 0.8rem;
}
.audio-device-icon {
    opacity: 75%;
}
</style>

<script setup>
import Wallpaper from '@/components/Wallpaper.vue';
import Icon from '@/components/Icon.vue';
</script>

<script>
export default {
    props: ['img', 'title', 'artist', 'album'],
    data() {
        return {
            currentTime: "0:00",
            totalTime: "0:00",
        }
    }
}
</script>