<script lang="ts">
    import { uiStack } from "$lib/runes/ui.svelte";
    
    /** Mapeo de iconos para actividades para evitar condicionales en el template */
    const activityIcons: Record<string, string> = {
        kanban: '📋', erd: '🏗️', flow: '🌊',
        sandbox: '🧪', agents: '🤖', explorer: '📁',
        graph: '🕸️', skills: '🧠', git: '🌿'
    };

    let showModeSelector = $state(false);

    /** Cambia el modo y cierra el selector visual */
    function selectMode(mode: any) {
        uiStack.setMode(mode);
        showModeSelector = false;
    }
</script>

<aside class="w-14 flex flex-col items-center py-4 bg-sidebar border-r border-white/5 z-[100] relative">
    
    <button 
        onclick={() => showModeSelector = !showModeSelector}
        class="w-10 h-10 mb-6 rounded-xl bg-accent flex items-center justify-center text-xl shadow-lg shadow-accent/40 hover:scale-110 transition-transform"
    >
        {uiStack.mode === 'blueprint' ? '🧠' : uiStack.mode === 'forge' ? '🔥' : '📡'}
    </button>

    {#if showModeSelector}
        <div class="absolute left-16 top-4 bg-sidebar border border-white/10 p-2 rounded-2xl shadow-2xl flex flex-col gap-2">
            <button onclick={() => selectMode('blueprint')} class="p-3 hover:bg-white/5 rounded-xl">🧠 Blueprint</button>
            <button onclick={() => selectMode('forge')} class="p-3 hover:bg-white/5 rounded-xl">🔥 Forge</button>
            <button onclick={() => selectMode('pulse')} class="p-3 hover:bg-white/5 rounded-xl">📡 Pulse</button>
        </div>
    {/if}

    <nav class="flex-1 flex flex-col gap-4">
        {#each uiStack.currentActivities as activity}
            <button 
                onclick={() => uiStack.setActivity(activity)}
                class="w-10 h-10 flex items-center justify-center text-xl transition-all
                {uiStack.activeActivity === activity ? 'opacity-100 scale-110' : 'opacity-30 hover:opacity-100 hover:scale-110'}"
            >
                {activityIcons[activity] || '❓'}
            </button>
        {/each}
    </nav>
</aside>