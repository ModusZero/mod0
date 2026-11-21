import { sveltekit } from '@sveltejs/kit/vite';
import adapter from '@sveltejs/adapter-static';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
    plugins: [
        tailwindcss(),
        // SvelteKit 3 lee su configuración desde el plugin (ya no existe `svelte.config.js`).
        // Tauri no tiene servidor Node.js para SSR: se usa adapter-static en modo SPA.
        // Ver: https://v2.tauri.app/start/frontend/sveltekit/
        sveltekit({
            adapter: adapter({ fallback: 'index.html' }),
        }),
    ],

    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host ? { protocol: 'ws', host, port: 1421 } : undefined,
        watch: {
            ignored: ['**/src-tauri/**'],
        },
    },
});
