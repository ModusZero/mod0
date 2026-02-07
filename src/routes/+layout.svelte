<script lang="ts">
    import "../app.css";
    import { configStack } from "$lib/runes/config.svelte";
    import { uiStack } from "$lib/runes/ui.svelte";
    import ActivityBar from "$lib/components/layout/ActivityBar.svelte";
    import ModeSelector from "$lib/components/layout/ModeSelector.svelte";
    
    let { children } = $props();

    $effect(() => {
        document.documentElement.classList.toggle('dark', configStack.current.theme === 'dark');
    });
</script>

<div class="flex flex-col h-screen w-screen overflow-hidden bg-main text-black dark:text-white transition-colors duration-300">
    
    <header class="h-10 border-b border-black/10 dark:border-white/5 flex items-center px-4 bg-sidebar/50 backdrop-blur-md justify-between select-none relative z-50">
        <div class="flex gap-4 text-[11px] font-medium text-black/50 dark:text-white/40">
            <span class="hover:text-accent cursor-pointer transition-colors">Archivo</span>
            <span class="hover:text-accent cursor-pointer transition-colors">Editar</span>
        </div>

        <div class="absolute left-1/2 -translate-x-1/2">
            <ModeSelector />
        </div>

        <div class="flex items-center gap-3">
            <div class="text-[9px] opacity-30 font-black tracking-[0.3em] dark:text-white">MOD0</div>
            <div class="w-2 h-2 rounded-full bg-accent shadow-[0_0_8px_var(--accent)] animate-pulse"></div>
        </div>
    </header>

    <div class="flex flex-1 overflow-hidden relative">
        <ActivityBar />
        
        {#if uiStack.sidebarOpen}
            <aside class="w-64 bg-sidebar border-r border-black/10 dark:border-white/5 flex flex-col overflow-hidden transition-all">
                <div class="p-4 text-[10px] font-bold text-black/40 dark:text-white/30 uppercase tracking-widest border-b border-black/5 dark:border-white/5">
                    {uiStack.activeActivity.replace('-', ' ')}
                </div>
                <div class="flex-1 overflow-y-auto">
                    </div>
            </aside>
        {/if}

        <main class="flex-1 flex flex-col relative overflow-hidden bg-main shadow-inner">
            {@render children()}
        </main>
    </div>

    <footer class="h-7 border-t border-black/10 dark:border-white/5 bg-sidebar flex items-center px-3 justify-between text-[10px]">
        <div class="flex gap-4 items-center text-black/60 dark:text-white/50">
            <div class="flex items-center gap-1.5">
                <div class="w-2 h-2 rounded-full bg-green-500 shadow-[0_0_5px_rgba(34,197,94,0.5)]"></div>
                <span class="font-bold uppercase tracking-tighter">System Online</span>
            </div>
        </div>
        
        <div class="flex gap-4 items-center">
            <span class="bg-accent/10 text-accent px-2 py-0.5 rounded font-bold uppercase tracking-tighter">
                {uiStack.mode}
            </span>
            <span class="text-black/40 dark:text-white/40 uppercase font-bold">{configStack.current.theme} mode</span>
        </div>
    </footer>
</div>