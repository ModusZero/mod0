<script lang="ts">
    import { fileStack } from "$lib/core/runes/filesystem.svelte";
    import { Search, EllipsisVertical } from "lucide-svelte";
    import FileTree from "$lib/features/filesystem/FileTree.svelte";
    import { slide } from "svelte/transition";

    let searchQuery = $state("");

    function filterNodes(nodes: any[], query: string): any[] {
        return nodes.reduce((acc, node) => {
            if (node.name.toLowerCase().includes(query.toLowerCase())) {
                acc.push(node);
            } else if (node.children) {
                const children = filterNodes(node.children, query);
                if (children.length) acc.push({ ...node, children });
            }
            return acc;
        }, []);
    }

    const displayedFiles = $derived(searchQuery ? filterNodes(fileStack.files, searchQuery) : fileStack.files);
</script>

<section class="flex flex-col h-full bg-sidebar select-none">
    <div class="p-4 flex items-center justify-between">
        <span class="text-[10px] font-bold uppercase tracking-widest opacity-30">Explorer</span>
        <div class="flex gap-1">
            <button onclick={() => fileStack.isSearching = !fileStack.isSearching} class="p-1 hover:bg-white/10 rounded"><Search size={14}/></button>
            <button class="p-1 hover:bg-white/10 rounded"><EllipsisVertical size={14}/></button>
        </div>
    </div>

    {#if fileStack.isSearching}
        <div transition:slide class="px-4 pb-2">
            <input 
                bind:value={searchQuery}
                class="w-full bg-main/50 border border-white/10 rounded px-2 py-1 text-xs outline-none focus:border-accent/30"
                placeholder="Find in files..."
            />
        </div>
    {/if}

    <div class="flex-1 overflow-y-auto custom-scroll">
        <FileTree nodes={displayedFiles} />
    </div>
</section>