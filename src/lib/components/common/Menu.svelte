<script lang="ts">
    import type { MenuItem } from '$lib/core/types/menu';
    import Self from './Menu.svelte';
	import { fly } from 'svelte/transition';

	let { items, onclose }: { items: MenuItem[], onclose: () => void } = $props();
	let activeSubmenu = $state<number | null>(null);
</script>

<div 
	transition:fly={{ y: 5, duration: 150 }}
	class="min-w-50 bg-sidebar border border-border-subtle rounded-lg py-1 shadow-xl backdrop-blur-xl"
>
	{#each items as item, i}
		<div class="relative px-1">
			<button 
				onmouseenter={() => activeSubmenu = item.children ? i : null}
				onclick={() => {
					if (!item.children) {
						item.action?.();
						onclose();
					}
				}}
				class="w-full flex items-center gap-3 px-3 py-1.5 text-[11px] rounded-md transition-colors
				{item.danger ? 'text-red-400 hover:bg-red-500/10' : 'text-text/80 hover:bg-accent/10 hover:text-text'}"
			>
				{#if item.icon}<item.icon size={14} />{/if}
				<span class="flex-1 text-left">{item.label}</span>
				{#if item.shortcut}<span class="text-[9px] opacity-30 font-mono">{item.shortcut}</span>{/if}
				{#if item.children}<span class="text-[10px] opacity-30">→</span>{/if}
			</button>

			{#if item.children && activeSubmenu === i}
				<div class="absolute left-full top-0 ml-1">
					<Self items={item.children} {onclose} />
				</div>
			{/if}
		</div>
	{/each}
</div>