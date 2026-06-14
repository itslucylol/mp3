<template>
    <div>
        <span>{{ path }}</span>
        <Menu :items="menu_options" @select="menu_select"></Menu>
        <Wallpaper />
    </div>
</template>

<script setup>
import Wallpaper from '@/components/Wallpaper.vue';
import Menu from '@/components/Menu.vue';
</script>

<script>
export default {
    emits: ['goto'],
    data() {
        return {
            path: undefined,
            menu_options: [],
        }
    },
    methods: {
        async menu_select(option) {
            const is_dir = option.item.more;
            if (is_dir) return this.load_directory(`${this.path}/${option.item.name}`);
            //---   Songs   ---//
            const file_path = this.path +'/'+ option.item.name;
            await fetch('/api/play', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ file: file_path }),
            });
            this.$emit('goto', 'Now playing');
        },

        build_menu(directories, files) {
            this.menu_options = [];
            for (const dir of directories) {
                this.menu_options.push({
                    name: dir,
                    more: true,
                });
            }
            for (const file of files) {
                this.menu_options.push({
                    name: file,
                    more: false,
                });
            }
        },

        async load_directory(new_dir) {
            this.path = new_dir;
            const response = await fetch(`/api/library?path=${new_dir}`);
            const data = await response.json();
            this.build_menu(data.directories, data.files);
        }
    },
    mounted() {
        this.load_directory("/");
    }
}
</script>