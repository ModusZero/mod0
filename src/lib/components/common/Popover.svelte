<script lang="ts">
	import { type Snippet } from 'svelte';
	import FloatingPanel from './low-level/FloatingPanel.svelte';
	import { fade, scale } from 'svelte/transition';

	let { anchorEl, children, onclose }: { 
		anchorEl: HTMLElement | null, 
		children: Snippet,
		onclose: () => void 
	} = $props();
</script>

<FloatingPanel {anchorEl} offset={12}>
	<div 
		in:scale={{ duration: 150, start: 0.95 }}
		out:fade={{ duration: 100 }}
		class="bg-main border border-border-subtle rounded-2xl shadow-2xl p-4 min-w-70"
	>
		{@render children()}
	</div>
</FloatingPanel>