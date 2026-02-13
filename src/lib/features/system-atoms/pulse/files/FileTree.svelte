<script lang="ts">
	import { fileStack } from "$lib/features/filesystem/files-runes.svelte";
	import { ChevronRight, ChevronDown, Folder, FileCode } from "lucide-svelte";
    import Tooltip from "$lib/components/common/Tooltip.svelte";
    import Self from "./FileTree.svelte";
	import type { GenericFile } from "$lib/core/types/tab";

	let { nodes }: { nodes: GenericFile[] } = $props();
	let hoveredPath = $state<string | null>(null);
</script>

<ul class="pl-3 border-l border-white/5 ml-1.5 space-y-0.5">
	{#each nodes as node (node.path)}
		<li class="relative">
			<button 
				onclick={() => node.is_dir ? fileStack.toggleFolder(node.path) : fileStack.openFile(node, true)}
				ondblclick={() => !node.is_dir && fileStack.openFile(node, false)}
				onmouseenter={() => hoveredPath = node.path}
				onmouseleave={() => hoveredPath = null}
				class="w-full flex items-center gap-2 py-1 px-2 hover:bg-white/5 rounded text-[12px] group transition-colors select-none"
			>
				<div class="w-3 flex items-center justify-center">
					{#if node.is_dir}
						<span class="text-text/30 group-hover:text-text/60">
							{#if fileStack.expandedPaths.has(node.path)}
								<ChevronDown size={10} />
							{:else}
								<ChevronRight size={10} />
							{/if}
						</span>
					{/if}
				</div>

				{#if node.is_dir}
					<Folder size={14} class="text-accent/60 fill-accent/5" />
				{:else}
					<FileCode size={14} class="text-text/40 group-hover:text-text/60" />
				{/if}

				<span class="truncate text-text/80 group-hover:text-text">
					{node.name}
				</span>

				{#if hoveredPath === node.path}
					<div class="absolute left-full ml-2 z-50">
						<Tooltip text={node.name} />
					</div>
				{/if}
			</button>

			{#if node.is_dir && fileStack.expandedPaths.has(node.path) && node.children}
				<div class="overflow-hidden">
					<Self nodes={node.children} />
				</div>
			{/if}
		</li>
	{/each}
</ul>