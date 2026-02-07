<script lang="ts">
    import "../app.css";
    import { configStack } from "$lib/runes/config.svelte";
    import ActivityBar from "$lib/components/layout/ActivityBar.svelte";
    
    let { children } = $props();
</script>

<div class="flex flex-col h-screen w-screen overflow-hidden bg-main text-main {configStack.current.theme}">
    
    <header class="h-9 border-b border-white/5 flex items-center px-4 bg-sidebar/50 backdrop-blur-md justify-between select-none">
        <div class="flex gap-4 text-xs opacity-70">
            <span class="hover:opacity-100 cursor-pointer">Archivo</span>
            <span class="hover:opacity-100 cursor-pointer">Editar</span>
            <span class="hover:opacity-100 cursor-pointer">Vista</span>
            <span class="hover:opacity-100 cursor-pointer">Terminal</span>
        </div>
        <div class="text-[10px] opacity-30 font-bold tracking-widest">MODUS ZERO v0.1.0</div>
    </header>

    <div class="flex flex-1 overflow-hidden">
        <ActivityBar />
        
        <aside class="w-64 bg-sidebar/30 border-r border-white/5">
            </aside>

        <main class="flex-1 flex flex-col relative overflow-hidden bg-main">
            {@render children()}
        </main>
    </div>

    <footer class="h-6 border-t border-white/5 bg-accent text-[10px] text-white flex items-center px-3 justify-between">
        <div class="flex gap-3">
            <span>Main Agent: <b class="uppercase">Online</b></span>
            <span>UTF-8</span>
        </div>
        <div class="flex gap-3 items-center">
            <span>Mod: <b class="uppercase">{configStack.current.theme}</b></span>
            <span class="bg-white/20 px-2 rounded">v0.1.0</span>
        </div>
    </footer>
</div>