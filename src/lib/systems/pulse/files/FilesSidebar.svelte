<script lang="ts">
    import { FolderOpen, RefreshCw, Search } from '@lucide/svelte';
    import EmptyState from '#lib/components/common/EmptyState.svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { openFolder } from '#lib/core/registry/actions';
    import { filesStack } from '#lib/core/runes/files.svelte';
    import { reviewStack } from '#lib/core/runes/review.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import { toggleUIStack } from '#lib/core/runes/toggle-ui.svelte';
    import type { ChangeStatus } from '#lib/core/types/files';
    import { basename } from '#lib/core/utils/format';
    import FileTreeNode from '#lib/features/system-atoms/pulse/files/FileTreeNode.svelte';

    const session = $derived(sessionsStack.focused);

    const roots = $derived.by(() => {
        const options = new Map<string, string>();
        if (filesStack.projectPath) options.set(filesStack.projectPath, basename(filesStack.projectPath));
        for (const item of sessionsStack.sessions) {
            if (item.worktree && item.status !== 'exited') options.set(item.cwd, `${item.name} (worktree)`);
        }
        return [...options.entries()];
    });

    /** Archivos escritos por cada sesión (ruta normalizada → colores de sesión). */
    const touched = $derived.by(() => {
        const map = new Map<string, number[]>();
        for (const item of sessionsStack.sessions) {
            for (const file of item.touched) {
                const key = file.replace(/\\/g, '/');
                map.set(key, [...(map.get(key) ?? []), item.colorIdx]);
            }
        }
        return map;
    });

    /** Archivos que la sesión enfocada cambió (M/A/D/U), igual que el control de versiones de un editor. */
    const changes = $derived(
        new Map<string, ChangeStatus>(reviewStack.changes.map((change) => [change.path, change.status])),
    );

    // Mantiene la lista de cambios al día para marcar el árbol (sin cargar el diff).
    $effect(() => {
        void sessionsStack.focusedId;
        void session?.kpis.filesChanged;
        void reviewStack.reload(false);
    });

    const entries = $derived(filesStack.rootPath ? (filesStack.children[filesStack.rootPath] ?? []) : []);
</script>

{#if !filesStack.rootPath}
    <EmptyState icon={FolderOpen} title={t('project.none')} hint={t('files.noProjectHint')}>
        <button class="btn btn-primary" onclick={openFolder}>{t('project.open')}</button>
    </EmptyState>
{:else}
    <div class="flex h-full flex-col">
        <div class="panel-title">
            <span>{t('files.title')}</span>
            <div class="flex">
                <button
                    class="icon-btn size-6"
                    title="Ctrl+P"
                    onclick={() => toggleUIStack.openPalette('files')}><Search size={13} /></button
                >
                <button
                    class="icon-btn size-6"
                    title={t('common.refresh')}
                    onclick={() => filesStack.refresh()}><RefreshCw size={13} /></button
                >
            </div>
        </div>

        {#if roots.length > 1}
            <div class="px-2 pb-2">
                <select
                    class="input h-7"
                    value={filesStack.rootPath}
                    onchange={(event) => filesStack.setRoot(event.currentTarget.value)}
                >
                    {#each roots as [path, label] (path)}<option value={path}>{label}</option>{/each}
                </select>
            </div>
        {/if}

        {#if session && changes.size > 0}
            <p class="px-3 pb-1.5 text-meta text-muted">
                <span class="font-mono text-st-waiting">{changes.size}</span>
                {t('files.changedBy')}
                <span class="font-medium text-text">{session.name}</span>
            </p>
        {/if}

        <div class="min-h-0 flex-1 overflow-y-auto px-1 pb-2">
            {#each entries as entry (entry.path)}
                <FileTreeNode {entry} depth={0} root={filesStack.rootPath} {touched} {changes} />
            {/each}
        </div>
    </div>
{/if}
