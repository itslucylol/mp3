<template>
    <main>
        <header>
            <span>{{ new Date().toLocaleTimeString('en-US', { hour: 'numeric', minute: 'numeric', hour12: true }) }}</span>
            <span class="application">{{ view }}</span>
            <span style="margin: auto;"></span>
            <Icon name="bluetooth" :class="bluetooth === 'disconnected' ? 'pulse' : ''" size="22x22" />
            <Icon :name="`battery/${battery}`" size="22x22" :class="battery === 1 ? 'empty-battery-fix' : ''" />
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

.empty-battery-fix {
    transform: translateY(-1px);
}

.application {
    opacity: 50%;
    margin-left: 1rem;
}

.pulse {
  animation: pulse-size 1.5s infinite ease-in-out;
}
@keyframes pulse-size {
  0%, 100% {
    filter: opacity(1) invert(1);
  }
  50% {
    filter: opacity(0.25) invert(1);
  }
}
</style>

<script>
export default {
    props: ['view'],
    data() {
        return {
            battery: 5,
            bluetooth: 'disconnected' // off, disconnected, connected
        }
    },
    async mounted() {
        while (true) {
            if (this.battery + 1 > 5) this.battery = 0;
            this.battery++;
            await new Promise((resolve) => setTimeout(resolve, 250));
        }
    }
}
</script>

<script setup>
import Icon from '@/components/Icon.vue';
</script>