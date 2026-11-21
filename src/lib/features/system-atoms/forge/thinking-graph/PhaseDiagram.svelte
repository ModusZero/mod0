<script lang="ts">
    import {
        ArrowRight,
        ChevronDown,
        ChevronRight,
        CircleAlert,
        CircleCheck,
        Loader,
        RotateCcw,
    } from '@lucide/svelte';
    import { tick } from 'svelte';
    import { FamilyVisual } from '#lib/core/constants/family-config';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { graphStack } from '#lib/core/runes/graph.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import { basename } from '#lib/core/utils/format';
    import { buildDiagram, type Phase } from '#lib/core/utils/graph-layout';

    interface Props {
        sessionId: string;
    }

    let { sessionId }: Props = $props();

    let scroller = $state<HTMLDivElement>();
    let opened = $state<Record<string, boolean>>({});

    const nodes = $derived(graphStack.nodes(sessionId));
    const turns = $derived(buildDiagram(nodes));
    const lastTurn = $derived(turns.at(-1)?.turn ?? 0);
    const session = $derived(sessionsStack.byId(sessionId));

    /** Archivos distintos que toca una fase (para el resumen de la caja). */
    function targets(phase: Phase): string[] {
        return [...new Set(phase.nodes.map((node) => (node.target ? basename(node.target) : node.label)))];
    }

    function summary(phase: Phase): string {
        const unit =
            phase.family === 'exec' ? t('unit.commands') : phase.family === 'agent' ? '' : t('unit.files');
        return phase.nodes.length > 1 ? `×${phase.nodes.length} ${unit}`.trim() : '';
    }

    $effect(() => {
        void nodes.length;
        if (!graphStack.follow || !scroller) return;
        void tick().then(() => scroller?.scrollTo({ top: scroller.scrollHeight }));
    });

    $effect(() => {
        const turn = graphStack.jumpTurn;
        if (turn === null || !scroller) return;
        document.getElementById(`turn-${turn}`)?.scrollIntoView({ behavior: 'smooth', block: 'start' });
        graphStack.follow = false;
        graphStack.jumpTurn = null;
    });

    function onScroll() {
        if (!scroller) return;
        const atBottom = scroller.scrollHeight - scroller.scrollTop - scroller.clientHeight < 60;
        if (atBottom !== graphStack.follow) graphStack.follow = atBottom;
    }
</script>

<div bind:this={scroller} class="h-full space-y-5 overflow-y-auto p-5" onscroll={onScroll}>
    {#each turns as turn (turn.turn)}
        {@const expanded = graphStack.isExpanded(sessionId, turn.turn, lastTurn)}
        <section id="turn-{turn.turn}" class="rounded-2xl border border-border-subtle bg-sidebar">
            <button
                class="flex w-full items-center gap-3 px-4 py-3 text-left"
                onclick={() => graphStack.toggleTurn(sessionId, turn.turn, lastTurn)}
            >
                {#if expanded}<ChevronDown size={14} class="text-faint" />{:else}<ChevronRight
                        size={14}
                        class="text-faint"
                    />{/if}
                <span class="font-mono text-meta font-bold text-accent">T{turn.turn}</span>
                <span class="min-w-0 flex-1 truncate text-[0.9375rem] font-medium">{turn.prompt}</span>
                <span class="shrink-0 font-mono text-meta text-faint">
                    {turn.phases.length}
                    {t('unit.phases')}
                </span>
            </button>

            {#if expanded}
                <div class="flex flex-wrap items-start gap-y-4 px-4 pb-4">
                    {#each turn.phases as phase, index (phase.id)}
                        {@const visual = FamilyVisual[phase.family]}
                        {#if index > 0}
                            <span
                                class="mx-1.5 mt-5 flex shrink-0 items-center {phase.retry
                                    ? 'text-st-waiting'
                                    : 'text-faint'}"
                            >
                                {#if phase.retry}<RotateCcw size={14} />{:else}<ArrowRight size={16} />{/if}
                            </span>
                        {/if}

                        <div
                            class="w-56 rounded-xl border bg-raised {phase.failed
                                ? 'border-danger/60'
                                : phase.running
                                  ? 'border-st-working/60'
                                  : 'border-border-subtle'}"
                            style="border-left:3px solid var(--f-{phase.family})"
                        >
                            <button
                                class="flex w-full items-start gap-2.5 px-3 py-2.5 text-left"
                                onclick={() => (opened[phase.id] = !opened[phase.id])}
                            >
                                <visual.icon size={16} class="mt-0.5 shrink-0 {visual.text}" />
                                <span class="min-w-0 flex-1">
                                    <span class="flex items-center gap-1.5 text-[0.9375rem] font-semibold">
                                        {t(visual.label)}
                                        <span class="font-mono text-meta font-normal text-muted"
                                            >{summary(phase)}</span
                                        >
                                    </span>
                                    <span class="mt-0.5 block truncate font-mono text-meta text-faint">
                                        {targets(phase).slice(0, 2).join(', ')}{targets(phase).length > 2
                                            ? ` +${targets(phase).length - 2}`
                                            : ''}
                                    </span>
                                    {#if phase.retry}<span class="text-meta text-st-waiting"
                                            >{t('graph.retry')}</span
                                        >{/if}
                                </span>
                                {#if phase.running}<Loader
                                        size={14}
                                        class="anim-spin mt-0.5 text-st-working"
                                    />
                                {:else if phase.failed}<CircleAlert size={14} class="mt-0.5 text-danger" />
                                {:else}<CircleCheck size={14} class="mt-0.5 text-ok/70" />{/if}
                            </button>

                            {#if opened[phase.id]}
                                <ul class="space-y-px border-t border-border-subtle px-1.5 py-1.5">
                                    {#each phase.nodes as node (node.id)}
                                        <li>
                                            <button
                                                class="row-hover flex w-full items-center gap-2 rounded px-1.5 py-1 text-left text-meta {graphStack.selectedId ===
                                                node.id
                                                    ? 'bg-accent-soft'
                                                    : ''}"
                                                onclick={() =>
                                                    (graphStack.selectedId =
                                                        graphStack.selectedId === node.id ? null : node.id)}
                                            >
                                                <span
                                                    class="min-w-0 flex-1 truncate {node.status === 'error'
                                                        ? 'text-danger'
                                                        : ''}">{node.label}</span
                                                >
                                                {#if node.durationMs !== null}
                                                    <span class="font-mono text-faint"
                                                        >{(node.durationMs / 1000).toFixed(1)}s</span
                                                    >
                                                {/if}
                                            </button>
                                        </li>
                                    {/each}
                                </ul>
                            {/if}
                        </div>
                    {/each}

                    {#if session?.status === 'working' && turn.turn === lastTurn && !turn.stop}
                        <span class="mx-1.5 mt-5 text-faint"><ArrowRight size={16} /></span>
                        <div
                            class="mt-1 flex items-center gap-2 rounded-xl border border-dashed border-border-subtle px-3 py-2.5 text-muted"
                        >
                            <Loader size={14} class="anim-spin" />
                            {session.now?.label ?? t('status.working')}
                        </div>
                    {/if}

                    {#each turn.notices as notice (notice.id)}
                        <span class="mx-1.5 mt-5 text-faint"><ArrowRight size={16} /></span>
                        <button
                            class="mt-1 max-w-56 rounded-xl border border-st-waiting/50 bg-st-waiting/8 px-3 py-2.5 text-left text-meta text-st-waiting"
                            onclick={() => (graphStack.selectedId = notice.id)}
                        >
                            {notice.label}
                        </button>
                    {/each}

                    {#if turn.stop}
                        <span class="mx-1.5 mt-5 text-faint"><ArrowRight size={16} /></span>
                        <button
                            class="mt-1 max-w-64 rounded-xl border border-ok/40 bg-ok/8 px-3 py-2.5 text-left"
                            onclick={() => (graphStack.selectedId = turn.stop!.id)}
                        >
                            <span class="flex items-center gap-1.5 font-semibold text-ok"
                                ><CircleCheck size={14} />{t('graph.done')}</span
                            >
                            {#if turn.stop.detail}<span class="mt-0.5 line-clamp-2 block text-meta text-muted"
                                    >{turn.stop.detail}</span
                                >{/if}
                        </button>
                    {/if}
                </div>
            {/if}
        </section>
    {/each}
</div>
