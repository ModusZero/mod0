<script lang="ts">
    import { Check, ChevronDown, ChevronRight, FileDiff, Folder, List, ListTree } from '@lucide/svelte';
    import EmptyState from '#lib/components/common/EmptyState.svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { reviewStack } from '#lib/core/runes/review.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import type { ChangedFile, ChangeStatus } from '#lib/core/types/files';
    import { basename } from '#lib/core/utils/format';
    import { buildChangeTree, type ChangeTreeNode } from '#lib/core/utils/tree';

    const session = $derived(sessionsStack.focused);
    let closed = $state<Record<string, boolean>>({});

    const statusTone: Record<ChangeStatus, string> = {
        M: 'text-st-waiting',
        A: 'text-ok',
        U: 'text-ok',
        D: 'text-danger',
    };

    // Recarga cuando cambia de sesión o cuando la sesión reporta más/menos cambios.
    $effect(() => {
        void sessionsStack.focusedId;
        void session?.kpis.filesChanged;
        void session?.status;
        void reviewStack.reload();
    });

    const done = $derived(reviewStack.changes.filter((file) => reviewStack.isReviewed(file.path)).length);
    const tree = $derived(buildChangeTree(reviewStack.changes));
</script>

{#snippet fileRow(file: ChangedFile, depth: number, showDir: boolean)}
    {@const reviewed = reviewStack.isReviewed(file.path)}
    {@const slash = file.path.lastIndexOf('/')}
    <div
        class="group flex items-center gap-1 rounded-sm pr-1 {reviewStack.selectedPath === file.path
            ? 'bg-active'
            : 'hover:bg-hover'}"
    >
        <button
            class="flex min-w-0 flex-1 items-center gap-2 py-1 text-left"
            style="padding-left:{depth * 12 + 8}px"
            onclick={() => reviewStack.select(file.path)}
            title={file.path}
        >
            <span class="w-3 shrink-0 font-mono text-meta font-bold {statusTone[file.status]}"
                >{file.status}</span
            >
            <span class="min-w-0 flex-1 truncate {reviewed ? 'text-faint line-through' : ''}">
                {basename(file.path)}
                {#if showDir && slash > 0}<span class="ml-1 text-meta text-faint no-underline"
                        >{file.path.slice(0, slash)}</span
                    >{/if}
            </span>
            <span class="shrink-0 font-mono text-meta text-faint">
                {#if file.additions}<span class="text-ok">+{file.additions}</span>{/if}
                {#if file.deletions}<span class="ml-1 text-danger">−{file.deletions}</span>{/if}
            </span>
        </button>
        <button
            class="icon-btn size-5 {reviewed ? 'text-ok' : 'text-faint opacity-0 group-hover:opacity-100'}"
            title={t('review.markFile')}
            onclick={() => reviewStack.toggleReviewed(file.path)}
        >
            <Check size={13} />
        </button>
    </div>
{/snippet}

{#snippet branch(nodes: ChangeTreeNode[], depth: number)}
    {#each nodes as node (node.path)}
        {#if node.file}
            {@render fileRow(node.file, depth, false)}
        {:else}
            <button
                class="row-hover flex w-full items-center gap-1.5 rounded-sm py-1 pr-2 text-left text-muted"
                style="padding-left:{depth * 12 + 4}px"
                onclick={() => (closed[node.path] = !closed[node.path])}
            >
                {#if closed[node.path]}<ChevronRight size={12} class="shrink-0" />{:else}<ChevronDown
                        size={12}
                        class="shrink-0"
                    />{/if}
                <Folder size={13} class="shrink-0 text-accent/80" />
                <span class="min-w-0 flex-1 truncate">{node.name}</span>
                <span class="font-mono text-meta text-faint">{node.count}</span>
            </button>
            {#if !closed[node.path]}{@render branch(node.children, depth + 1)}{/if}
        {/if}
    {/each}
{/snippet}

{#if !session}
    <EmptyState icon={FileDiff} title={t('review.noSession')} />
{:else}
    <div class="flex h-full flex-col">
        <div class="panel-title">
            <span class="truncate">{session.name}</span>
            <span class="font-mono normal-case">
                <span class="text-ok">+{reviewStack.totals.additions}</span>
                <span class="text-danger">−{reviewStack.totals.deletions}</span>
            </span>
        </div>

        {#if reviewStack.changes.length > 0}
            <div class="flex items-center justify-between px-3 pb-2">
                <span class="font-mono text-meta text-faint"
                    >{done}/{reviewStack.changes.length} {t('review.reviewed')}</span
                >
                <div class="flex rounded-md border border-border-subtle bg-main p-0.5">
                    <button
                        class="flex size-5 items-center justify-center rounded {reviewStack.view === 'list'
                            ? 'bg-raised text-text'
                            : 'text-muted'}"
                        title={t('review.viewList')}
                        onclick={() => reviewStack.setView('list')}><List size={12} /></button
                    >
                    <button
                        class="flex size-5 items-center justify-center rounded {reviewStack.view === 'tree'
                            ? 'bg-raised text-text'
                            : 'text-muted'}"
                        title={t('review.viewTree')}
                        onclick={() => reviewStack.setView('tree')}><ListTree size={12} /></button
                    >
                </div>
            </div>
        {/if}

        <div class="min-h-0 flex-1 overflow-y-auto px-1 pb-2">
            {#if reviewStack.view === 'tree'}
                {@render branch(tree, 0)}
            {:else}
                {#each reviewStack.changes as file (file.path)}
                    {@render fileRow(file, 0, true)}
                {/each}
            {/if}
            {#if reviewStack.changes.length === 0}
                <p class="px-3 py-8 text-center text-meta text-faint">{t('review.noChanges')}</p>
            {/if}
        </div>
    </div>
{/if}
