<script lang="ts">
	import { Search, Command } from "lucide-svelte";
	import { workStack } from "$lib/core/runes/work-runes.svelte";
	import { WorkSectionVisual } from "$lib/core/constants/work-config";
	import ModeSelector from "./ModeSelector.svelte";
	import CommandPalette from "./CommandPalette.svelte";

	let isModeOpen = $state(false);
	let isCommandOpen = $state(false);
	let omniAnchor = $state<HTMLElement | null>(null);

	const current = $derived(WorkSectionVisual[workStack.mode]);
</script>

<div 
    bind:this={omniAnchor}
    class="w-full max-w-125 flex items-center gap-2 px-2 bg-black/20 border border-white/5 rounded-xl transition-all focus-within:border-accent/50 h-8"
>
    <button 
        bind:this={omniAnchor}
        onclick={() => isModeOpen = true}
        class="w-6 h-6 flex items-center justify-center rounded-md hover:bg-white/5 text-accent active:scale-90 transition-all"
    >
        <current.icon size={15} />
    </button>

    <button 
        class="flex-1 flex items-center gap-2 h-full text-left group"
        onclick={() => isCommandOpen = true}
    >
        <Search size={13} class="text-text/20 group-hover:text-accent transition-all group-active:scale-75" />
        <span class="text-[11px] text-text/40 truncate">Buscar...</span>
    </button>

    <div class="flex items-center gap-1 px-1.5 py-0.5 bg-white/5 rounded border border-white/5 opacity-40">
        <Command size={10} />
        <span class="text-[9px] font-mono">P</span>
    </div>
</div>

{#if isModeOpen}
	<ModeSelector anchorEl={omniAnchor} onclose={() => isModeOpen = false} />
{/if}

{#if isCommandOpen}
	<CommandPalette anchorEl={omniAnchor} onclose={() => isCommandOpen = false} />
{/if}