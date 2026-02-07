<script lang="ts">
    import ModeSelector from "$lib/components/layout/ModeSelector.svelte";
    
    const headerOptions = ["Archivo", "Editar", "Vista", "Opciones"];
</script>

<header class="h-10 border-b border-border-subtle flex items-center px-4 bg-sidebar/50 backdrop-blur-md justify-between select-none relative z-50">
    <div class="flex gap-4 text-[11px] font-medium text-text/50">
        {#each headerOptions as option}
            <button class="hover:text-text/80 transition-colors">{option}</button>
        {/each}
    </div>

    <div class="absolute left-1/2 -translate-x-1/2">
        <ModeSelector />
    </div>

    <div class="flex items-center gap-3">
        <span class="text-[9px] opacity-30 font-black tracking-[0.3em]">MOD0</span>
        <div class="w-2 h-2 rounded-full bg-accent shadow-[0_0_8px_var(--accent)] animate-pulse"></div>
    </div>
</header>