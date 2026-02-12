<script lang="ts">
    import { fileStack } from "$lib/features/filesystem/files-runes.svelte";
    import { Search, EllipsisVertical } from "lucide-svelte";
    import FileTree from "$lib/features/filesystem/FileTree.svelte";
    import { slide } from "svelte/transition";
    import type { GenericFile } from "$lib/core/types/tab";

    let searchQuery = $state("");

    /**
     * Filtra nodos recursivamente. 
     * Si searchQuery está vacío, no debería ni ejecutarse.
     */
    function filterNodes(nodes: GenericFile[], query: string): GenericFile[] {
        const lowerQuery = query.toLowerCase();
        
        return nodes.reduce((acc: GenericFile[], node) => {
            // Comprobamos si el nombre coincide
            const nameMatches = node.name.toLowerCase().includes(lowerQuery);
            
            if (node.is_dir) {
                // Si es directorio, filtramos sus hijos primero
                const filteredChildren = node.children ? filterNodes(node.children, query) : [];
                
                // Si la carpeta coincide OR tiene hijos que coinciden, la incluimos
                if (nameMatches || filteredChildren.length > 0) {
                    acc.push({ 
                        ...node, 
                        children: filteredChildren 
                    });
                }
            } else if (nameMatches) {
                // Si es archivo y coincide, lo incluimos
                acc.push(node);
            }
            
            return acc;
        }, []);
    }

    /** * IMPORTANTE: Usamos un derivado robusto.
     * Si no hay query, devolvemos la referencia directa de fileStack.files.
     */
    const displayedFiles = $derived.by(() => {
        const query = searchQuery.trim();
        if (!query) return fileStack.files;
        return filterNodes(fileStack.files, query);
    });
</script>

<section class="flex flex-col h-full bg-sidebar select-none border-r border-white/5">
    <div class="p-4 flex items-center justify-between">
        <span class="text-[10px] font-bold uppercase tracking-widest opacity-30">Explorer</span>
        <div class="flex gap-1">
            <button 
                onclick={() => fileStack.isSearching = !fileStack.isSearching} 
                class="p-1 hover:bg-white/10 rounded transition-colors {fileStack.isSearching ? 'text-accent' : 'text-text/50'}"
            >
                <Search size={14}/>
            </button>
            <button class="p-1 hover:bg-white/10 rounded text-text/50"><EllipsisVertical size={14}/></button>
        </div>
    </div>

    {#if fileStack.isSearching}
        <div transition:slide={{ duration: 150 }} class="px-4 pb-3">
            <input 
                bind:value={searchQuery}
                placeholder="Filter by name..."
                class="w-full bg-main/30 border border-white/10 rounded px-2 py-1.5 text-xs outline-none focus:border-accent/40"
            />
        </div>
    {/if}

    <div class="flex-1 overflow-y-auto custom-scroll pb-4">
        {#if displayedFiles && displayedFiles.length > 0}
            <FileTree nodes={displayedFiles} />
        {:else}
            <div class="px-6 py-10 flex flex-col items-center justify-center opacity-20">
                <p class="text-[10px] uppercase tracking-tighter font-bold">
                    {searchQuery ? "No matches found" : "Empty Workspace"}
                </p>
            </div>
        {/if}
    </div>
</section>