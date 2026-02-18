<script lang="ts">
	import { scale } from 'svelte/transition';
	import { type Snippet } from 'svelte';
	import Portal from './Portal.svelte';
	import Overlay from './Overlay.svelte';

	let { children, onclose }: { children: Snippet, onclose: () => void } = $props();

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') onclose();
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<Portal>
	<Overlay isDark={true} onclick={onclose} />
	<div class="fixed inset-0 z-999 flex items-center justify-center pointer-events-none">
		<div 
			transition:scale={{ start: 0.95, duration: 200 }}
			class="pointer-events-auto shadow-2xl"
		>
			{@render children()}
		</div>
	</div>
</Portal>