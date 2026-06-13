<template>
    <main>
        <header>
            <span>{{ new Date().toLocaleTimeString('en-US', { hour: 'numeric', minute: 'numeric', hour12: true }) }}</span>
            <span class="application">{{ view }}</span>
            <span style="margin: auto;"></span>
            <Icon name="bluetooth" size="22x22" />
            <Icon name="battery/3" size="22x22" />
        </header>
        <slot />
    </main>
</template>

<style scoped>
header {
    font-size: 0.9rem;
    display: flex;
    padding: 0 1rem;
    margin-bottom: 0.5rem;
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
import Icon from '@/components/Icon.vue';
</script>