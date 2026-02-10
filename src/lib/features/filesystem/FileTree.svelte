<script lang="ts">
    import { fileStack, type FileNode } from "$lib/core/runes/filesystem.svelte";
    import { ChevronRight, ChevronDown, Folder, FileCode } from "lucide-svelte";
    import Self from "./FileTree.svelte";

    let { nodes }: { nodes: FileNode[] } = $props();
    let expanded = $state<Record<string, boolean>>({});

    function toggle(path: string) {
        expanded[path] = !expanded[path];
    }
</script>

<ul class="pl-2 border-l border-white/5 ml-1">
    {#each nodes as node}
        <li>
            <button 
                onclick={() => node.is_dir ? toggle(node.path) : fileStack.openFile(node)}
                class="w-full flex items-center gap-1.5 py-0.5 px-2 hover:bg-white/5 rounded text-[12px] group transition-colors"
            >
                {#if node.is_dir}
                    <span class="text-text/30 group-hover:text-text/60">
                        {expanded[node.path] ? "▼" : "▶"}
                    </span>
                    <Folder size={14} class="text-accent/60" />
                {:else}
                    <FileCode size={14} class="text-text/40" />
                {/if}
                <span class="truncate text-text/80 group-hover:text-text">
                    {node.name}
                </span>
            </button>

            {#if node.is_dir && expanded[node.path] && node.children}
                <Self nodes={node.children} />
            {/if}
        </li>
    {/each}
</ul>