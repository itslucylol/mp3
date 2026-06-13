<template>
    <div class="fsb">
        <div></div>
        <img :src="img || '/sample/wallpaper.jpg'" />
    </div>
</template>

<style scoped>
.fsb {
    position: absolute;
    top: 0;
    left: 0;
    width: 100vw;
    height: auto;
    z-index: -100;
}
.fsb > div {
    position: absolute;
    inset: 0;
    backdrop-filter: blur(1rem) brightness(0.5);
}
.fsb > img {
    height: 100vh;
    width: 100vw;
}
</style>

<script>
export default {
    props: ['img']
}
</script>