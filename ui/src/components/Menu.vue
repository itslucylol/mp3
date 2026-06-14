<template>
    <div class="menu">
        <div>
            <div v-for="(item, index) in items" :key="index" class="menu-item" :class="index === selected ? 'menu-item-active' : ''">
                <span>{{ item.name }}</span>
                <span v-if="item.more" class="menu-more">></span>
            </div>
        </div>
    </div>
</template>

<style scoped>
.menu-item {
    opacity: 0.5;
    display: flex;
    padding: 0.25rem 0.75rem;
    font-size: 1rem;
}
.menu-item-active {
    opacity: 1;
    background-color: rgba(0, 50, 100, 0.5);
}

.menu-more {
    margin-left: auto;
}
</style>

<script>
export default {
    emits: ['select'],
    props: ['items'],
    data() {
        return {
            selected: 0,
        }
    },
    mounted() {
        window.addEventListener('keydown', this.handleKeyDown);
    },
    beforeUnmount() {
        window.removeEventListener('keydown', this.handleKeyDown);
    },
    methods: {
        handleKeyDown(event) {
            if (event.key === 'ArrowDown') {
                event.preventDefault(); // Prevents page scrolling
                this.selected = (this.selected + 1) % this.items.length;
            } else if (event.key === 'ArrowUp') {
                event.preventDefault(); // Prevents page scrolling
                this.selected = (this.selected - 1 + this.items.length) % this.items.length;
            } else if (event.key === 'Enter') {
                event.preventDefault();
                // Emits 'select' event with the payload object
                this.$emit('select', {
                    item: this.items[this.selected],
                    index: this.selected
                });
            }
        }
    }
}
</script>