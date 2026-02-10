<script lang="ts">
    import "../app.css";
    import { type } from '@tauri-apps/plugin-os';
    import { getCurrentWebview } from '@tauri-apps/api/webview';
    import Header from "$lib/components/layout/Header.svelte";
    import ActivityBar from "$lib/components/layout/ActivityBar.svelte";
    import Sidebar from "$lib/components/layout/Sidebar.svelte";
    import Footer from "$lib/components/layout/Footer.svelte";
    import Terminal from "$lib/features/terminal/Terminal.svelte";
    import Settings from "$lib/components/settings(future-migration)/Settings.svelte";

    let { children } = $props();
    let osType = $state(type()); 

    
    let zoomLevel = 1.0;

    async function changeZoom(delta: number) {
        const webview = getCurrentWebview();
        zoomLevel = Math.min(Math.max(zoomLevel + delta, 0.5), 1.0); // Limitar entre 50% y 100%
        
        await webview.setZoom(zoomLevel);
    }

    $effect(() => {
        // Escuchar Ctrl + / Ctrl -
        const handleKeyDown = (e: KeyboardEvent) => {
            if (e.ctrlKey) {
                if (e.key === '+' || e.key === '=') { e.preventDefault(); changeZoom(0.1); }
                if (e.key === '-') { e.preventDefault(); changeZoom(-0.1); }
                if (e.key === '0') { e.preventDefault(); zoomLevel = 1.0; changeZoom(0); }
            }
        };

        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    });
</script>

<div class="flex flex-col h-screen w-screen overflow-hidden bg-main text-text selection:bg-accent/30 os-{osType}">
    <Header />

    <div class="flex flex-1 overflow-hidden relative">
        <ActivityBar />
        <Sidebar />

        <main class="flex-1 flex flex-col min-w-0 min-h-0 relative">
            <div class="flex-1 min-h-0 overflow-hidden">
                {@render children()}
            </div>

            <Terminal />
        </main>
    </div>

    <Footer />
</div>

<Settings />