<template>
    <main>
        <header>
            <span>{{ new Date().toLocaleTimeString('en-US', { hour: 'numeric', minute: 'numeric', hour12: true }) }}</span>
            <span class="application">{{ view }}</span>
            <span style="margin: auto;"></span>
            <Icon name="bluetooth" size="30x30" />
            <Icon name="battery/2" size="30x30" />
        </header>
        <slot />
    </main>
</template>

<style scoped>
header {
    font-size: 1.2rem;
    display: flex;
    padding: 0 1rem;
}

.application {
    opacity: 50%;
    margin-left: 1rem;
}
</style>

<script>
export default {
    props: ['view']
}
</script>

<script setup>
import Icon from './Icon.vue';
</script>