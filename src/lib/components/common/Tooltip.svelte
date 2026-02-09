<script lang="ts">
	import { fade } from 'svelte/transition';
	let { text }: { text: string } = $props();
</script>

<div 
	transition:fade={{ duration: 100 }}
	class="px-2 py-1 bg-text text-main text-[10px] font-bold rounded shadow-lg whitespace-nowrap"
>
	{text}
</div>