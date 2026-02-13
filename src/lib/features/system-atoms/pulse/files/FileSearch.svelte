<script lang="ts">
    import { fileStack } from "../filesystem/files-runes.svelte";
    import { Search, FileText, List, Trees, X } from "lucide-svelte";
    import { slide } from "svelte/transition";

    let { searchQuery = $bindable(), viewMode = $bindable() } = $props();
</script>

<div transition:slide={{ duration: 150 }} class="flex flex-col gap-2 p-4 border-b border-white/5 bg-sidebar/50">
    <div class="flex items-center justify-between">
        <span class="text-[9px] font-black uppercase tracking-[0.2em] opacity-30">
            {fileStack.searchMode === 'content' ? 'Global Grep' : 'File Filter'}
        </span>
        <div class="flex gap-1">
            <button 
                onclick={() => viewMode = viewMode === 'tree' ? 'list' : 'tree'}
                title={`Cambiar a modo ${viewMode === 'tree' ? 'lista' : 'árbol'}`}
                class="p-1.5 hover:text-accent transition-colors"
            >
                {#if viewMode === 'tree'}
                    <List size={12}/>
                {:else}
                    <Trees size={12}/>
                {/if}
                
            </button>
            <button 
                onclick={() => fileStack.searchMode = fileStack.searchMode === 'files' ? 'content' : 'files'}
                class="p-1.5 hover:bg-white/10 rounded transition-colors {fileStack.searchMode === 'content' ? 'text-accent' : 'text-text/40'}"
            >
                <FileText size={12}/>
            </button>
        </div>
    </div>

    <div class="relative group">
        <input 
            bind:value={searchQuery}
            placeholder={fileStack.searchMode === 'content' ? "Search in files..." : "Filter by name..."}
            class="w-full bg-main/50 border border-white/5 rounded-md px-2 py-1.5 pl-8 text-xs outline-none focus:border-accent/30 group-hover:border-white/10 transition-all"
        />
        <Search size={12} class="absolute left-2.5 top-2.5 opacity-20 group-focus-within:text-accent group-focus-within:opacity-100" />
        {#if searchQuery}
            <button onclick={() => searchQuery = ""} class="absolute right-2 top-2.5 opacity-30 hover:opacity-100">
                <X size={12}/>
            </button>
        {/if}
    </div>
</div>