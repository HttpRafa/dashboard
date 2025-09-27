import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// CSS
import tailwindcss from '@tailwindcss/vite'

// https://vite.dev/config/
export default defineConfig({
    plugins: [svelte(), tailwindcss()],
})
