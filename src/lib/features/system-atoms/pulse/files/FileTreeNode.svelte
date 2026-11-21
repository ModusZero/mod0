<script lang="ts">
    import { ChevronDown, ChevronRight, FileText, Folder, FolderOpen } from '@lucide/svelte';
    import { editorStack } from '#lib/core/runes/editor.svelte';
    import { filesStack } from '#lib/core/runes/files.svelte';
    import type { ChangeStatus, DirEntryInfo } from '#lib/core/types/files';
    import { relativeTo } from '#lib/core/utils/format';
    import Self from './FileTreeNode.svelte';

    interface Props {
        entry: DirEntryInfo;
        depth: number;
        root: string;
        /** Mapa `ruta normalizada → índices de color` de las sesiones que tocaron ese archivo. */
        touched: Map<string, number[]>;
        /** Cambios de la sesión enfocada: `ruta relativa → estado` (M, A, D, U). */
        changes: Map<string, ChangeStatus>;
    }

    let { entry, depth, root, touched, changes }: Props = $props();

    const norm = (path: string) => path.replace(/\\/g, '/');
    const tone: Record<ChangeStatus, string> = {
        M: 'text-st-waiting',
        A: 'text-ok',
        U: 'text-ok',
        D: 'text-danger',
    };

    const open = $derived(!!filesStack.expanded[entry.path]);
    const children = $derived(filesStack.children[entry.path] ?? []);
    const relative = $derived(relativeTo(root, entry.path));
    const status = $derived(entry.isDir ? undefined : changes.get(relative));
    /** Cuántos archivos cambiados hay dentro de esta carpeta. */
    const changedInside = $derived(
        entry.isDir ? [...changes.keys()].filter((path) => path.startsWith(`${relative}/`)).length : 0,
    );

    /** Colores de las sesiones que han tocado este archivo (o algo dentro de la carpeta). */
    const marks = $derived.by(() => {
        const path = norm(entry.path);
        const colors = new Set<number>();

        for (const [file, indexes] of touched) {
            if (file === path || (entry.isDir && file.startsWith(`${path}/`)))
                indexes.forEach((i) => colors.add(i));
        }
        return [...colors];
    });

    function activate() {
        if (entry.isDir) void filesStack.toggle(entry.path);
        else void editorStack.open(entry.path);
    }
</script>

<button
    class="row-hover flex h-6 w-full items-center gap-1.5 rounded-sm pr-2 text-left {editorStack.activePath ===
    entry.path
        ? 'bg-active'
        : ''}"
    style="padding-left:{depth * 12 + 6}px"
    onclick={activate}
    title={entry.path}
>
    {#if entry.isDir}
        {#if open}<ChevronDown size={12} class="shrink-0 text-faint" />{:else}<ChevronRight
                size={12}
                class="shrink-0 text-faint"
            />{/if}
        {#if open}<FolderOpen size={14} class="shrink-0 text-accent" />{:else}<Folder
                size={14}
                class="shrink-0 text-accent/80"
            />{/if}
    {:else}
        <span class="w-3 shrink-0"></span>
        <FileText size={14} class="shrink-0 text-faint" />
    {/if}
    <span class="min-w-0 flex-1 truncate {status ? tone[status] : ''}">{entry.name}</span>
    {#each marks as color (color)}
        <span class="size-1.5 shrink-0 rounded-full" style="background:var(--s{color})"></span>
    {/each}
    {#if status}
        <span class="w-3 shrink-0 text-center font-mono text-meta font-bold {tone[status]}">{status}</span>
    {:else if changedInside > 0}
        <span class="size-1.5 shrink-0 rounded-full bg-st-waiting" title={String(changedInside)}></span>
    {/if}
</button>

{#if entry.isDir && open}
    {#each children as child (child.path)}
        <Self entry={child} depth={depth + 1} {root} {touched} {changes} />
    {/each}
{/if}
