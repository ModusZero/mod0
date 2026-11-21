<script lang="ts">
    import { ChevronDown, ChevronRight, CircleX, Loader } from '@lucide/svelte';
    import { tick } from 'svelte';
    import { FamilyVisual } from '#lib/core/constants/family-config';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { graphStack } from '#lib/core/runes/graph.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import { LANE_WIDTH, ROW_HEIGHT, layoutGraph } from '#lib/core/utils/graph-layout';
    import { basename } from '#lib/core/utils/format';
    import type { ThoughtNode } from '#lib/core/types/session';

    interface Props {
        sessionId: string;
    }

    let { sessionId }: Props = $props();

    let scroller = $state<HTMLDivElement>();

    const nodes = $derived(graphStack.nodes(sessionId));
    const lastTurn = $derived(nodes.at(-1)?.turn ?? 0);
    const layout = $derived(layoutGraph(nodes, (turn) => graphStack.isExpanded(sessionId, turn, lastTurn)));
    const gutter = $derived(layout.lanes * LANE_WIDTH + 28);
    const session = $derived(sessionsStack.byId(sessionId));

    const x = (lane: number) => 18 + lane * LANE_WIDTH;
    const y = (value: number) => value + ROW_HEIGHT / 2;

    function path(link: (typeof layout.links)[number]): string {
        const [x1, y1, x2, y2] = [x(link.from.lane), y(link.from.y), x(link.to.lane), y(link.to.y)];
        if (x1 === x2) return `M${x1},${y1} L${x2},${y2}`;
        const mid = (y1 + y2) / 2;
        return `M${x1},${y1} C${x1},${mid} ${x2},${mid} ${x2},${y2}`;
    }

    function duration(node: ThoughtNode): string {
        if (node.durationMs === null) return '';
        return node.durationMs < 1000 ? `${node.durationMs}ms` : `${(node.durationMs / 1000).toFixed(1)}s`;
    }

    // Seguir lo último que hace la IA, salvo que el usuario se haya desplazado hacia arriba.
    $effect(() => {
        void layout.height;
        if (!graphStack.follow || !scroller) return;
        void tick().then(() => scroller?.scrollTo({ top: scroller.scrollHeight }));
    });

    $effect(() => {
        const turn = graphStack.jumpTurn;
        if (turn === null || !scroller) return;
        const row = layout.rows.find((item) => item.type === 'header' && item.turn === turn);
        if (row) {
            graphStack.follow = false;
            scroller.scrollTo({ top: row.y, behavior: 'smooth' });
        }
        graphStack.jumpTurn = null;
    });

    function onScroll() {
        if (!scroller) return;
        const atBottom = scroller.scrollHeight - scroller.scrollTop - scroller.clientHeight < 60;
        if (atBottom !== graphStack.follow) graphStack.follow = atBottom;
    }
</script>

<div bind:this={scroller} class="h-full overflow-x-hidden overflow-y-auto" onscroll={onScroll}>
    <div class="relative" style="height:{layout.height + 16}px">
        <svg
            class="pointer-events-none absolute top-0 left-0"
            width={gutter}
            height={layout.height}
            aria-hidden="true"
        >
            {#each layout.links as link, index (index)}
                <path
                    d={path(link)}
                    fill="none"
                    stroke="var(--text-faint)"
                    stroke-opacity="0.45"
                    stroke-width="1.5"
                />
            {/each}
            {#each layout.rows as row, index (index)}
                {#if row.type === 'node' && row.node}
                    {@const selected = graphStack.selectedId === row.node.id}
                    <circle
                        cx={x(row.lane)}
                        cy={y(row.y)}
                        r={row.node.kind === 'tool' ? 4.5 : 5.5}
                        fill={row.node.status === 'error'
                            ? 'var(--danger)'
                            : row.node.kind === 'tool'
                              ? `var(--f-${row.node.family})`
                              : 'var(--bg-main)'}
                        stroke={row.node.kind === 'tool'
                            ? 'none'
                            : row.node.kind === 'notice'
                              ? 'var(--st-waiting)'
                              : 'var(--text-muted)'}
                        stroke-width="2"
                        class={row.node.status === 'running' ? 'anim-pulse' : ''}
                    />
                    {#if selected}<circle
                            cx={x(row.lane)}
                            cy={y(row.y)}
                            r="8.5"
                            fill="none"
                            stroke="var(--accent)"
                            stroke-width="1.5"
                        />{/if}
                {/if}
            {/each}
        </svg>

        {#each layout.rows as row, index (index)}
            {#if row.type === 'header'}
                <button
                    class="absolute right-0 flex items-center gap-2 pr-4 text-left transition-colors hover:bg-hover"
                    style="top:{row.y}px;left:0;height:{ROW_HEIGHT}px;padding-left:10px"
                    onclick={() => graphStack.toggleTurn(sessionId, row.turn, lastTurn)}
                >
                    {#if row.expanded}<ChevronDown size={13} class="text-faint" />{:else}<ChevronRight
                            size={13}
                            class="text-faint"
                        />{/if}
                    <span class="font-mono text-meta font-semibold text-accent">T{row.turn}</span>
                    <span class="min-w-0 flex-1 truncate font-medium">{row.prompt}</span>
                    <span class="flex shrink-0 items-center gap-3 font-mono text-meta text-faint">
                        {#if row.summary?.running}<Loader size={12} class="anim-spin text-st-working" />{/if}
                        {#if row.summary?.errors}<span class="text-danger">⚠ {row.summary.errors}</span>{/if}
                        <span>{row.summary?.steps} {t('unit.steps')}</span>
                        {#if row.summary?.files}<span>{row.summary.files} {t('unit.files')}</span>{/if}
                    </span>
                </button>
            {:else if row.node}
                {@const node = row.node}
                {@const visual = FamilyVisual[node.family]}
                <button
                    class="absolute right-0 flex items-center gap-2 pr-4 text-left transition-colors {graphStack.selectedId ===
                    node.id
                        ? 'bg-accent-soft'
                        : 'hover:bg-hover'}"
                    style="top:{row.y}px;left:{gutter}px;height:{ROW_HEIGHT}px"
                    onclick={() =>
                        (graphStack.selectedId = graphStack.selectedId === node.id ? null : node.id)}
                >
                    {#if node.kind === 'tool'}
                        <visual.icon size={13} class="shrink-0 {visual.text}" />
                    {:else if node.kind === 'notice'}
                        <CircleX size={13} class="shrink-0 text-st-waiting" />
                    {/if}
                    <span
                        class="max-w-[46%] shrink-0 truncate {node.kind === 'prompt'
                            ? 'font-medium'
                            : ''} {node.status === 'error' ? 'text-danger' : ''} {node.kind === 'stop'
                            ? 'text-ok'
                            : ''}"
                    >
                        {node.kind === 'prompt' ? t('node.prompt') : node.label}
                    </span>
                    {#if node.target && node.kind === 'tool'}
                        <span class="shrink-0 truncate font-mono text-meta text-faint"
                            >{basename(node.target)}</span
                        >
                    {/if}
                    {#if node.intent}
                        <span class="min-w-0 flex-1 truncate text-meta text-muted italic">{node.intent}</span>
                    {:else}
                        <span class="flex-1"></span>
                    {/if}
                    <span class="shrink-0 font-mono text-meta text-faint">{duration(node)}</span>
                </button>
            {/if}
        {/each}

        {#if session?.status === 'working' && layout.rows.length > 0}
            <div
                class="absolute flex items-center gap-2 text-meta text-muted"
                style="top:{layout.height}px;left:{gutter}px"
            >
                <Loader size={12} class="anim-spin" />
                {session.now?.label ?? t('status.working')}
            </div>
        {/if}
    </div>
</div>
