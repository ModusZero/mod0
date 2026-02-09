<script lang="ts">
	import { onMount, onDestroy, type Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
	let container = $state<HTMLElement | null>(null);

	onMount(() => {
		container = document.createElement('div');
		container.dataset.portal = "modus-zero";
		document.body.appendChild(container);
	});

	onDestroy(() => {
		if (container && container.parentNode === document.body) {
			document.body.removeChild(container);
		}
	});
</script>

{#if container}
	{@render children()}
{/if}