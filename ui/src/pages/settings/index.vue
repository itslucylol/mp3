<template>
    <div class="info">
        <Menu :items="items" @select="changeView" v-if="view === 'Menu'"></Menu>
        <Wallpaper />

        <About v-if="view == 'About'" />
    </div>
</template>

<script setup>
import Wallpaper from '@/components/Wallpaper.vue';
import Menu from '@/components/Menu.vue';

import About from './about.vue';
</script>

<script>
export default {
    emits: ['goto'],
    data() {
        return {
            view: "Menu",
            items: [
                { name: "About",   more: true  },
            ]
        }
    },
    methods: {
        changeView(payload) {
            this.view = payload.item.name
        }
    }
}
</script>