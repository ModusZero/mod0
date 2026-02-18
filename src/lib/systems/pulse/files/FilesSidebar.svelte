<script lang="ts">
	import { EllipsisVertical, Search as SearchIcon, FolderPlus, FilePlus, RefreshCw, Trash2 } from "lucide-svelte";
	import { fileStack } from "$lib/features/system-atoms/pulse/files/files-runes.svelte";
    import Menu from "$lib/components/common/Menu.svelte";
    import FloatingPanel from "$lib/components/common/low-level/FloatingPanel.svelte";
	import FileHistory from "$lib/features/system-atoms/pulse/files/FileHistory.svelte";
	import FileSearch from "$lib/features/system-atoms/pulse/files/FileSearch.svelte";
	import FileTree from "$lib/features/system-atoms/pulse/files/FileTree.svelte";
	import type { GenericFile } from "$lib/core/types/tab";

	let searchQuery = $state("");
	let viewMode = $state<'tree' | 'list'>('tree');
	
	// Control del menú de opciones
	let menuAnchor = $state<HTMLElement | null>(null);
	let showMenu = $state(false);

	const projectActions = [
		{ label: 'New File', icon: FilePlus, action: () => fileStack.createNewFile() },
		{ label: 'New Folder', icon: FolderPlus, action: () => console.log('New Folder') },
		{ label: 'Refresh Explorer', icon: RefreshCw, action: () => fileStack.openFolder(fileStack.rootPath!) },
		{ label: 'Delete Project Cache', icon: Trash2, danger: true, action: () => console.log('Delete') },
	];

	function filterRecursive(nodes: GenericFile[], query: string): GenericFile[] {
		const q = query.toLowerCase();
		return nodes.reduce((acc: GenericFile[], node) => {
			const match = node.name.toLowerCase().includes(q);
			if (node.is_dir) {
				const children = node.children ? filterRecursive(node.children, query) : [];
				if (match || children.length > 0) acc.push({ ...node, children });
			} else if (match) {
				acc.push(node);
			}
			return acc;
		}, []);
	}

	const displayedFiles = $derived.by(() => {
		if (!searchQuery.trim() || fileStack.searchMode === 'content') return fileStack.files;
		return filterRecursive(fileStack.files, searchQuery);
	});
</script>

<section class="flex flex-col h-full bg-sidebar border-r border-white/5 select-none relative">
	<div class="p-4 flex items-center justify-between">
		<span class="text-[10px] font-black uppercase tracking-[0.3em] opacity-20">Project</span>
		<div class="flex gap-0.5">
			<button 
                title="Buscar"
				onclick={() => fileStack.isSearching = !fileStack.isSearching}
				class="p-1.5 hover:bg-white/10 rounded transition-colors {fileStack.isSearching ? 'text-accent bg-accent/5' : 'text-text/40'}"
			>
				<SearchIcon size={14}/>
			</button>
			
			<button 
				bind:this={menuAnchor}
                title="Más opciones"
				onclick={() => showMenu = !showMenu}
				class="p-1.5 hover:bg-white/10 rounded text-text/40 hover:text-text transition-colors"
			>
				<EllipsisVertical size={14}/>
			</button>

			{#if showMenu}
				<FloatingPanel anchorEl={menuAnchor} offset={8}>
					<Menu items={projectActions} onclose={() => showMenu = false} />
				</FloatingPanel>
			{/if}
		</div>
	</div>

	{#if fileStack.isSearching}
		<FileSearch bind:searchQuery bind:viewMode />
	{/if}

	<div class="flex-1 overflow-y-auto custom-scroll">
		<FileHistory />
		
		<div class="py-2">
			{#if displayedFiles.length > 0}
				<FileTree nodes={displayedFiles} />
			{:else}
				<div class="p-10 text-center opacity-20 text-[10px] font-bold uppercase">
					{searchQuery ? 'No results' : 'Empty'}
				</div>
			{/if}
		</div>
	</div>
</section>