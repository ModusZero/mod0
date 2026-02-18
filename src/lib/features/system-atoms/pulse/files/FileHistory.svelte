<script lang="ts">
	import { fileStack } from "./files-runes.svelte";
    import Badge from "$lib/components/common/Badge.svelte";
    import { Clock } from "lucide-svelte";
</script>

{#if fileStack.recentFiles.size > 0}
	<div class="px-4 py-2 flex items-center justify-between">
		<div class="flex items-center gap-2 opacity-40">
			<Clock size={12} />
			<span class="text-[10px] font-bold uppercase tracking-wider">Recents</span>
		</div>
		<Badge count={fileStack.recentFiles.size} variant="ghost" />
	</div>
	
	<div class="flex flex-col px-2 mb-4">
		{#each fileStack.recentFiles.values() as file}
			<button 
				onclick={() => fileStack.openFile(file, false)}
				class="flex items-center gap-2 px-2 py-1 text-[11px] text-text/60 hover:text-text hover:bg-white/5 rounded transition-all truncate"
			>
				<span class="opacity-30">#</span>
				{file.name}
			</button>
		{/each}
	</div>
{/if}