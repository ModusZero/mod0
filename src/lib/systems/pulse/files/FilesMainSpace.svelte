<script lang="ts">
    import { FileCode, Minimize2, Network, Plus, RefreshCw, Zap } from '@lucide/svelte';
    import EmptyState from '#lib/components/common/EmptyState.svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { editorStack } from '#lib/core/runes/editor.svelte';
    import { filesStack } from '#lib/core/runes/files.svelte';
    import { reviewStack } from '#lib/core/runes/review.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import { formatCompact, joinPath } from '#lib/core/utils/format';
    import FileGraph from '#lib/features/system-atoms/pulse/files/FileGraph.svelte';

    let selected = $state<string | null>(null);
    let timer: ReturnType<typeof setTimeout> | undefined;

    const graph = $derived(filesStack.graph);
    const node = $derived(graph?.nodes.find((item) => item.id === selected) ?? null);
    const links = $derived.by(() => ({
        uses: graph?.edges.filter((edge) => edge.from === selected) ?? [],
        usedBy: graph?.edges.filter((edge) => edge.to === selected) ?? [],
    }));
    const live = $derived(sessionsStack.sessions.filter((session) => session.status !== 'exited'));
    const session = $derived(sessionsStack.focused);
    const changes = $derived(reviewStack.changes);

    /** Nivel visible = la carpeta expandida más profunda + 1. */
    const level = $derived(Math.max(1, ...filesStack.graphExpanded.map((dir) => dir.split('/').length + 1)));

    // Carga el grafo al entrar y al cambiar la raíz.
    $effect(() => {
        void filesStack.rootPath;
        clearTimeout(timer);
        timer = setTimeout(() => void filesStack.loadGraph(), 120);
    });

    // Mantiene los cambios de la sesión enfocada (para resaltarlos) sin leer diffs.
    $effect(() => {
        void sessionsStack.focusedId;
        void session?.kpis.filesChanged;
        void reviewStack.reload(false);
    });

    // Análisis de impacto: qué archivos dependen de lo que cambió la sesión.
    $effect(() => {
        const paths = changes.filter((change) => change.status !== 'D').map((change) => change.path);
        if (filesStack.impactOn) void filesStack.loadImpact(paths);
        else filesStack.impact = [];
    });

    const impactedModules = $derived.by(() => {
        if (!graph) return 0;
        const ids = new Set(graph.nodes.map((item) => item.id));
        const found = new Set<string>();
        for (const path of filesStack.impact) {
            const parts = path.split('/');
            for (let end = parts.length; end > 0; end--) {
                const id = parts.slice(0, end).join('/');
                if (ids.has(id)) {
                    found.add(id);
                    break;
                }
            }
        }
        return found.size;
    });
</script>

{#if !filesStack.rootPath}
    <EmptyState icon={Network} title={t('project.none')} hint={t('files.noProjectHint')} />
{:else}
    <div class="flex h-full flex-col">
        <header class="flex h-12 shrink-0 items-center gap-4 border-b border-border-subtle px-4">
            <h2 class="shrink-0 font-semibold whitespace-nowrap">{t('files.graph')}</h2>

            <label class="flex shrink-0 items-center gap-2 text-meta whitespace-nowrap text-muted">
                {t('files.depth')}
                <input
                    type="range"
                    min="1"
                    max={Math.max(2, (graph?.maxDepth ?? 3) + 1)}
                    value={level}
                    oninput={(event) => filesStack.setGraphLevel(Number(event.currentTarget.value))}
                    class="w-28 accent-[var(--accent)]"
                />
                <span class="w-3 font-mono text-text">{level}</span>
            </label>

            <button class="btn" onclick={() => filesStack.setGraphLevel(1)} title={t('files.collapseAll')}>
                <Minimize2 size={13} />
            </button>

            <button
                class="btn {filesStack.impactOn ? 'btn-primary' : ''}"
                disabled={changes.length === 0}
                onclick={() => (filesStack.impactOn = !filesStack.impactOn)}
                title={t('files.impactHint')}
            >
                <Zap size={13} />
                {t('files.impact')}
                {#if filesStack.impactOn}<span class="font-mono">{filesStack.impact.length}</span>{/if}
            </button>

            {#if graph}
                <span class="min-w-0 truncate font-mono text-meta text-faint">
                    {graph.nodes.length}
                    {t('unit.nodes')} · {graph.edges.length}
                    {t('unit.links')} · {formatCompact(graph.fileCount)}
                    {t('unit.files')}
                    {#if filesStack.impactOn && filesStack.impact.length > 0}
                        · <span class="text-st-waiting">{impactedModules} {t('files.affectedNodes')}</span>
                    {/if}
                </span>
            {/if}

            <div class="ml-auto flex min-w-0 items-center gap-3 overflow-hidden">
                {#each live as item (item.id)}
                    <span class="flex shrink-0 items-center gap-1.5 text-meta whitespace-nowrap text-muted">
                        <span class="size-2 rounded-full" style="background:var(--s{item.colorIdx})"></span>
                        {item.name}
                    </span>
                {/each}
                <button
                    class="icon-btn"
                    title={t('common.refresh')}
                    onclick={() => filesStack.loadGraph(true)}
                >
                    <RefreshCw size={14} class={filesStack.graphLoading ? 'anim-spin' : ''} />
                </button>
            </div>
        </header>

        <div class="relative min-h-0 flex-1">
            {#if filesStack.graphError}
                <EmptyState icon={Network} title={t('files.graphError')} hint={filesStack.graphError} />
            {:else if graph && graph.nodes.length > 0}
                <FileGraph
                    {graph}
                    root={filesStack.rootPath}
                    {selected}
                    {changes}
                    impacted={filesStack.impact}
                    onselect={(id) => (selected = id)}
                    onexpand={(id) => filesStack.toggleExpand(id)}
                />
            {:else if !filesStack.graphLoading}
                <EmptyState icon={Network} title={t('files.graphEmpty')} hint={t('files.graphEmptyHint')} />
            {/if}

            <p class="pointer-events-none absolute bottom-3 left-4 text-meta text-faint">
                {t('files.graphTip')}
            </p>

            {#if node}
                <aside
                    class="absolute top-3 right-3 w-64 rounded-xl border border-border-subtle bg-raised p-3 shadow-lg"
                >
                    <p class="font-semibold break-all">{node.id}{node.kind === 'dir' ? '/' : ''}</p>
                    <p class="mb-2 font-mono text-meta text-faint">
                        {node.files}
                        {t('unit.files')} · {formatCompact(node.bytes)} B
                    </p>

                    <div class="mb-2 flex gap-1.5">
                        {#if node.kind === 'dir'}
                            <button class="btn" onclick={() => filesStack.toggleExpand(node.id)}>
                                <Plus size={13} />
                                {filesStack.graphExpanded.includes(node.id)
                                    ? t('files.collapse')
                                    : t('files.expand')}
                            </button>
                        {:else if filesStack.rootPath}
                            <button
                                class="btn"
                                onclick={() => editorStack.open(joinPath(filesStack.rootPath!, node.id))}
                            >
                                <FileCode size={13} />
                                {t('node.open')}
                            </button>
                        {/if}
                    </div>

                    {#each [{ title: t('files.uses'), list: links.uses, key: 'to' as const }, { title: t('files.usedBy'), list: links.usedBy, key: 'from' as const }] as group (group.title)}
                        {#if group.list.length > 0}
                            <h4 class="label mt-2">{group.title}</h4>
                            <ul class="max-h-32 overflow-y-auto font-mono text-meta text-muted">
                                {#each group.list as edge (edge[group.key])}
                                    <li class="flex justify-between gap-2">
                                        <span class="truncate">{edge[group.key]}</span><span
                                            class="text-faint">{edge.weight}</span
                                        >
                                    </li>
                                {/each}
                            </ul>
                        {/if}
                    {/each}
                </aside>
            {/if}
        </div>
    </div>
{/if}
