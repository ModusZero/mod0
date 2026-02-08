<script lang="ts">
    import { scale, fade } from 'svelte/transition';
    import { cubicOut, cubicInOut } from 'svelte/easing';
    import { workStack } from "$lib/core/runes/work-runes.svelte";
    import { WorkSectionIDs, WorkSectionVisual } from "$lib/core/constants/work-config";
    import type { WorkSectionID } from "$lib/core/types/work";

    let isOpen = $state(false);
    const current = $derived(WorkSectionVisual[workStack.mode]);

    const colorMap: Record<string, string> = {
        'text-blue-400': '#60a5fa',
        'text-orange-400': '#fb923c',
        'text-purple-400': '#c084fc',
        'text-gray-400': '#9ca3af'
    };

    function selectMode(id: WorkSectionID) {
        workStack.setMode(id);
        isOpen = false;
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Escape') isOpen = false;
    }
</script>

<div class="relative flex flex-col items-center">
    <button 
        onclick={() => isOpen = !isOpen}
        onkeydown={handleKeydown}
        aria-expanded={isOpen}
        aria-haspopup="true"
        aria-label="Seleccionar modo de trabajo"
        class="relative z-30 flex items-center gap-3 px-4 py-1.5 bg-text/5 border border-border-subtle rounded-full hover:border-accent/50 transition-all duration-300 group overflow-hidden"
    >
        <current.icon size={16} class="text-accent relative z-10" />
        <span class="text-[10px] font-bold uppercase tracking-[0.2em] text-text/80 relative z-10">
            {current.label}
        </span>
    </button>

    {#if isOpen}
        <button 
            type="button"
            onclick={() => isOpen = false}
            in:fade={{ duration: 200 }}
            out:fade={{ duration: 150 }}
            class="fixed top-0 left-0 right-0 bottom-0 w-[200vw] h-[200vw] -translate-x-1/2 -translate-y-1/2 z-10 bg-black/5 backdrop-blur-[2px] cursor-default border-none outline-none"
            aria-label="Cerrar menú"
        ></button>

        <div 
            role="menu"
            in:scale={{ duration: 400, start: 0.9, easing: cubicOut }}
            out:scale={{ duration: 250, start: 0.95, easing: cubicInOut }}
            class="absolute top-full mt-4 flex gap-5 p-5 bg-sidebar/40 backdrop-blur-3xl rounded-[2.5rem] border border-white/10 shadow-[0_25px_70px_rgba(0,0,0,0.5)] z-20"
        >
            {#each Object.values(WorkSectionIDs) as id, i}
                {@const visual = WorkSectionVisual[id]}
                {@const isActive = workStack.mode === id}
                {@const marbleColor = visual.color? colorMap[visual.color] : '#ffffff'}
                
                <button
                    role="menuitem"
                    onclick={() => selectMode(id)}
                    in:fade={{ delay: i * 40, duration: 300 }}
                    class="relative group w-12 h-12 rounded-full flex items-center justify-center transition-all duration-700
                    {isActive ? 'scale-110' : 'hover:scale-105'}"
                    style="--this-color: {marbleColor}"
                >
                    <div class="absolute inset-0 rounded-full overflow-hidden border border-white/20 shadow-2xl bg-black/40">
                        
                        <div 
                            class="absolute inset-0 opacity-40 group-hover:opacity-100 transition-all duration-1000 blur-xl scale-150 animate-swirl"
                            style="background: radial-gradient(circle at {20 + (i * 15)}% {80 - (i * 10)}%, var(--this-color) 0%, transparent 70%);"
                        ></div>
                        
                        <div 
                            class="absolute inset-0 opacity-20 group-hover:opacity-60 transition-all duration-1000 blur-2xl scale-125 animate-swirl-reverse"
                            style="background: conic-gradient(from {i * 90}deg at 50% 50%, transparent, var(--this-color), transparent);"
                        ></div>

                        <div class="absolute top-1 left-3 right-3 h-1/2 bg-linear-to-b from-white/30 to-transparent rounded-t-full"></div>
                        <div class="absolute bottom-2 right-4 w-1.5 h-1.5 bg-white/20 rounded-full blur-[0.5px]"></div>
                        
                        <div class="absolute inset-0 transition-colors duration-500 {isActive ? 'bg-(--this-color)/20' : 'group-hover:bg-(--this-color)/10'}"></div>
                    </div>

                    <visual.icon 
                        size={24} 
                        class="relative z-10 transition-all duration-500 group-hover:rotate-12 group-hover:scale-110
                        {isActive ? 'text-white drop-shadow-[0_0_8px_var(--this-color)]' : 'text-white/40 group-hover:text-white'}" 
                    />
                </button>
            {/each}
        </div>
    {/if}
</div>

<style>
    @keyframes swirl {
        0% { transform: scale(1.5) rotate(0deg); }
        50% { transform: scale(1.8) rotate(180deg); }
        100% { transform: scale(1.5) rotate(360deg); }
    }

    @keyframes swirl-reverse {
        0% { transform: scale(1.2) rotate(360deg); }
        100% { transform: scale(1.2) rotate(0deg); }
    }

    .animate-swirl { animation: swirl 10s infinite linear; }
    .animate-swirl-reverse { animation: swirl-reverse 14s infinite linear; }

    button div {
        -webkit-mask-image: -webkit-radial-gradient(white, black);
        mask-image: radial-gradient(white, black);
    }
</style>