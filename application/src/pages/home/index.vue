<template>
    <div class="info">
        <Menu :items="items" @select="passSelect"></Menu>
        <Wallpaper />
    </div>
</template>

<script setup>
import Wallpaper from '@/components/Wallpaper.vue';
import Menu from '@/components/Menu.vue';
</script>

<script>
export default {
    emits: ['select'],
    data() {
        return {
            items: [
                { name: "Now playing", more: true },
                { name: "Music", more: true },
                { name: "Photos", more: true },
                { name: "Videos", more: true },
                { name: "Extras", more: true },
                { name: "Settings", more: true },
                { name: "Shuffle Songs", more: false },
            ]
        }
    },
    methods: {
        passSelect(payload) {
            this.$emit('select', payload);
        }
    }
}
</script>