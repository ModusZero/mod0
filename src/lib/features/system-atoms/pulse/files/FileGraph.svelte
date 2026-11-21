<script lang="ts">
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import type { ChangedFile, DepGraph } from '#lib/core/types/files';
    import { forceLayout, type LayoutNode } from '#lib/core/utils/force-layout';
    import { relativeTo } from '#lib/core/utils/format';

    interface Props {
        graph: DepGraph;
        root: string;
        selected: string | null;
        /** Cambios de la sesión enfocada (se resaltan con su letra M/A/D/U). */
        changes: ChangedFile[];
        /** Archivos afectados por esos cambios (análisis de impacto). */
        impacted: string[];
        onselect: (id: string | null) => void;
        onexpand: (id: string) => void;
    }

    let { graph, root, selected, changes, impacted, onselect, onexpand }: Props = $props();

    const WIDTH = 1000;
    const HEIGHT = 680;
    const TONE: Record<string, string> = {
        M: 'var(--st-waiting)',
        A: 'var(--ok)',
        U: 'var(--ok)',
        D: 'var(--danger)',
    };

    let zoom = $state(1);
    let pan = $state({ x: 0, y: 0 });
    let hovered = $state<string | null>(null);
    let dragging = false;
    let moved = false;
    let last = { x: 0, y: 0 };

    const radius = (files: number) => Math.min(34, 8 + Math.sqrt(files) * 2.4);

    const laid = $derived.by(() => {
        const nodes: LayoutNode[] = graph.nodes.map((node) => ({
            id: node.id,
            radius: radius(node.files),
            x: 0,
            y: 0,
        }));
        return new Map(forceLayout(nodes, graph.edges, WIDTH, HEIGHT).map((node) => [node.id, node]));
    });

    const nodeIds = $derived(new Set(graph.nodes.map((node) => node.id)));

    /** Nodo visible que contiene un archivo (el prefijo más largo que exista en el grafo). */
    function nodeOfRelative(relative: string): string | undefined {
        const parts = relative.split('/');
        for (let end = parts.length; end > 0; end--) {
            const id = parts.slice(0, end).join('/');
            if (nodeIds.has(id)) return id;
        }
        return undefined;
    }

    /** Letras de cambio por nodo: `{ M: 3, U: 1 }`. */
    const changed = $derived.by(() => {
        const map = new Map<string, Record<string, number>>();
        for (const change of changes) {
            const id = nodeOfRelative(change.path);
            if (!id) continue;
            const counts = map.get(id) ?? {};
            counts[change.status] = (counts[change.status] ?? 0) + 1;
            map.set(id, counts);
        }
        return map;
    });

    const impactedNodes = $derived(new Set(impacted.map(nodeOfRelative).filter((id): id is string => !!id)));

    /** Qué sesiones están trabajando en cada módulo ahora mismo. */
    const activity = $derived.by(() => {
        const map = new Map<string, Array<{ color: number; name: string; working: boolean }>>();
        for (const session of sessionsStack.sessions) {
            if (session.status === 'exited') continue;

            const files = new Set(session.touched.map((file) => relativeTo(session.cwd, file)));
            if (session.now?.target) files.add(relativeTo(session.cwd, session.now.target));

            for (const id of new Set([...files].map(nodeOfRelative).filter((id): id is string => !!id))) {
                map.set(id, [
                    ...(map.get(id) ?? []),
                    { color: session.colorIdx, name: session.name, working: session.status === 'working' },
                ]);
            }
        }
        return map;
    });

    const related = $derived.by(() => {
        const focus = hovered ?? selected;
        if (!focus) return null;
        const ids = new Set([focus]);
        for (const edge of graph.edges) {
            if (edge.from === focus) ids.add(edge.to);
            if (edge.to === focus) ids.add(edge.from);
        }
        return ids;
    });

    function wheel(event: WheelEvent) {
        event.preventDefault();
        zoom = Math.min(3, Math.max(0.35, zoom * (event.deltaY < 0 ? 1.1 : 0.9)));
    }

    function down(event: PointerEvent) {
        dragging = true;
        moved = false;
        last = { x: event.clientX, y: event.clientY };
        (event.currentTarget as Element).setPointerCapture(event.pointerId);
    }

    function move(event: PointerEvent) {
        if (!dragging) return;
        const scale = WIDTH / (event.currentTarget as SVGElement).getBoundingClientRect().width;
        pan = { x: pan.x + (event.clientX - last.x) * scale, y: pan.y + (event.clientY - last.y) * scale };
        if (Math.abs(event.clientX - last.x) + Math.abs(event.clientY - last.y) > 2) moved = true;
        last = { x: event.clientX, y: event.clientY };
    }
</script>

<svg
    class="h-full w-full touch-none select-none"
    viewBox="0 0 {WIDTH} {HEIGHT}"
    preserveAspectRatio="xMidYMid meet"
    onwheel={wheel}
    onpointerdown={down}
    onpointermove={move}
    onpointerup={() => {
        dragging = false;
        if (!moved) onselect(null);
    }}
    role="img"
    aria-label="Grafo de dependencias"
>
    <defs>
        <marker
            id="arrow"
            viewBox="0 0 10 10"
            refX="9"
            refY="5"
            markerWidth="4.5"
            markerHeight="4.5"
            orient="auto-start-reverse"
        >
            <path d="M0,0 L10,5 L0,10 z" fill="var(--text-faint)" />
        </marker>
    </defs>

    <g
        transform="translate({pan.x + (WIDTH * (1 - zoom)) / 2} {pan.y +
            (HEIGHT * (1 - zoom)) / 2}) scale({zoom})"
    >
        {#each graph.edges as edge (`${edge.from}>${edge.to}`)}
            {@const a = laid.get(edge.from)}
            {@const b = laid.get(edge.to)}
            {#if a && b}
                {@const dx = b.x - a.x}
                {@const dy = b.y - a.y}
                {@const length = Math.hypot(dx, dy) || 1}
                {@const focus = hovered ?? selected}
                {@const active = focus !== null && (edge.from === focus || edge.to === focus)}
                <line
                    x1={a.x + (dx / length) * a.radius}
                    y1={a.y + (dy / length) * a.radius}
                    x2={b.x - (dx / length) * (b.radius + 4)}
                    y2={b.y - (dy / length) * (b.radius + 4)}
                    stroke={active ? 'var(--accent)' : 'var(--text-faint)'}
                    stroke-width={0.8 + Math.log2(1 + edge.weight) * 0.55}
                    stroke-opacity={related ? (active ? 0.9 : 0.08) : 0.32}
                    marker-end="url(#arrow)"
                />
            {/if}
        {/each}

        {#each graph.nodes as node (node.id)}
            {@const point = laid.get(node.id)}
            {#if point}
                {@const sessions = activity.get(node.id) ?? []}
                {@const counts = changed.get(node.id)}
                {@const dim = related && !related.has(node.id)}
                {@const r = radius(node.files)}
                <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
                <g
                    transform="translate({point.x} {point.y})"
                    opacity={dim ? 0.25 : 1}
                    class="cursor-pointer"
                    onpointerenter={() => (hovered = node.id)}
                    onpointerleave={() => (hovered = null)}
                    onpointerdown={(event) => event.stopPropagation()}
                    onclick={(event) => {
                        event.stopPropagation();
                        onselect(selected === node.id ? null : node.id);
                    }}
                    ondblclick={(event) => {
                        event.stopPropagation();
                        if (node.kind === 'dir') onexpand(node.id);
                    }}
                >
                    {#each sessions as session, index (index)}
                        <circle
                            r={r + 5 + index * 4}
                            fill="none"
                            stroke="var(--s{session.color})"
                            stroke-width="2.2"
                            class={session.working ? 'anim-pulse' : ''}
                        />
                    {/each}
                    {#if impactedNodes.has(node.id)}
                        <circle
                            r={r + 6}
                            fill="none"
                            stroke="var(--st-waiting)"
                            stroke-width="1.6"
                            stroke-dasharray="4 3"
                        />
                    {/if}

                    {#if node.kind === 'file'}
                        <rect
                            x={-r}
                            y={-r * 0.8}
                            width={r * 2}
                            height={r * 1.6}
                            rx="5"
                            fill={selected === node.id ? 'var(--accent-soft)' : 'var(--bg-raised)'}
                            stroke={counts
                                ? 'var(--st-waiting)'
                                : selected === node.id
                                  ? 'var(--accent)'
                                  : 'var(--text-faint)'}
                            stroke-width={counts ? 2.2 : 1.5}
                        />
                    {:else}
                        <circle
                            {r}
                            fill={selected === node.id ? 'var(--accent-soft)' : 'var(--bg-raised)'}
                            stroke={counts
                                ? 'var(--st-waiting)'
                                : selected === node.id
                                  ? 'var(--accent)'
                                  : 'var(--text-faint)'}
                            stroke-width={counts ? 2.2 : 1.5}
                        />
                        <text
                            text-anchor="middle"
                            dy="0.35em"
                            font-size="10"
                            font-family="var(--font-mono)"
                            fill="var(--text-faint)"
                        >
                            {node.files}
                        </text>
                    {/if}

                    <text
                        y={r + 14 + sessions.length * 4}
                        text-anchor="middle"
                        font-size="11.5"
                        fill="var(--text-muted)"
                    >
                        {node.label}{node.kind === 'dir' ? '/' : ''}
                    </text>

                    {#if counts}
                        <g transform="translate({r * 0.7} {-r - 4})">
                            {#each Object.entries(counts) as [letter, count], index (letter)}
                                <text
                                    x={index * 17}
                                    font-size="10.5"
                                    font-family="var(--font-mono)"
                                    font-weight="700"
                                    fill={TONE[letter]}
                                >
                                    {letter}{count > 1 ? count : ''}
                                </text>
                            {/each}
                        </g>
                    {/if}
                </g>
            {/if}
        {/each}
    </g>
</svg>
