<script lang="ts">
	import FloatingPanel from './low-level/FloatingPanel.svelte';
	import { slide } from 'svelte/transition';

	let { options, value, onselect }: { 
		options: {label: string, value: any}[], 
		value: any,
		onselect: (v: any) => void 
	} = $props();

	let anchor = $state<HTMLElement | null>(null);
	let isOpen = $state(false);
	const currentLabel = $derived(options.find(o => o.value === value)?.label);
</script>

<div class="relative inline-block w-full">
	<button
		bind:this={anchor}
		onclick={() => isOpen = !isOpen}
		class="w-full flex items-center justify-between px-3 py-1.5 bg-sidebar/50 border border-border-subtle rounded-lg text-xs"
	>
		<span class="truncate">{currentLabel}</span>
		<span class="opacity-30">▼</span>
	</button>

	{#if isOpen}
		<FloatingPanel anchorEl={anchor} offset={4}>
			<div 
				transition:slide={{ duration: 150 }}
				class="bg-sidebar border border-border-subtle rounded-lg py-1 shadow-xl min-w-(--anchor-width)"
			>
				{#each options as opt}
					<button 
						onclick={() => { onselect(opt.value); isOpen = false; }}
						class="w-full text-left px-3 py-1.5 text-xs hover:bg-accent/10 transition-colors
						{value === opt.value ? 'text-accent' : 'text-text/70'}"
					>
						{opt.label}
					</button>
				{/each}
			</div>
		</FloatingPanel>
	{/if}
</div>