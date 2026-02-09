<script lang="ts">
	import { type Snippet } from 'svelte';
	let { children, hoverable = false }: { children: Snippet, hoverable?: boolean } = $props();
</script>

<div class="bg-sidebar/30 border border-border-subtle rounded-xl p-4 transition-all
	{hoverable ? 'hover:border-accent/40 hover:bg-sidebar/50 cursor-pointer' : ''}">
	{@render children()}
</div>