<script lang="ts">
    import { fly, scale, fade } from 'svelte/transition';
    import { elasticOut } from 'svelte/easing';
    import { uiStack } from "$lib/runes/ui.svelte";
    import { WorkSectionIDs, WorkSectionVisual } from "$lib/constants/ui";
    import type { WorkSectionID } from "$lib/types/ui";

    let isOpen = $state(false);
    const current = $derived(WorkSectionVisual[uiStack.mode]);

    function selectMode(id: WorkSectionID) {
        uiStack.setMode(id);
        isOpen = false;
    }
</script>

<div class="relative flex flex-col items-center">
    <button 
        onclick={() => isOpen = !isOpen}
        class="flex items-center gap-3 px-4 py-1.5 bg-black/3 dark:bg-white/5 border border-black/10 dark:border-white/10 rounded-full hover:border-accent/50 transition-all duration-300 group"
    >
        <current.icon size={16} class="text-accent" />
        <span class="text-[10px] font-bold uppercase tracking-[0.2em] text-black/80 dark:text-white/80">{current.label}</span>
    </button>

    {#if isOpen}
        <div class="fixed inset-0 z-10" onclick={() => isOpen = false}></div>

        <div 
            transition:fly={{ y: 10, duration: 400, easing: elasticOut }}
            class="absolute top-full mt-4 flex gap-4 p-3 bg-white dark:bg-[#18181b] rounded-3xl border border-black/10 dark:border-white/10 shadow-2xl z-20"
        >
            {#each Object.values(WorkSectionIDs) as id, i}
                {@const visual = WorkSectionVisual[id]}
                <button
                    onclick={() => selectMode(id)}
                    in:scale={{ delay: i * 40, duration: 400, easing: elasticOut, start: 0.5 }}
                    class="relative group w-12 h-12 rounded-full flex items-center justify-center transition-all 
                    {uiStack.mode === id ? 'bg-accent text-white' : 'bg-black/5 dark:bg-white/5 text-black/40 dark:text-white/40 hover:text-accent'}"
                >
                    <visual.icon size={20} />
                </button>
            {/each}
        </div>
    {/if}
</div>