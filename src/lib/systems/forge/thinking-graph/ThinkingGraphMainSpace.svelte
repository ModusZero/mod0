<script lang="ts">
    import { ArrowDownToLine, Network } from '@lucide/svelte';
    import Segmented from '#lib/components/common/Segmented.svelte';
    import EmptyState from '#lib/components/common/EmptyState.svelte';
    import StatusIcon from '#lib/components/common/StatusIcon.svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { graphStack } from '#lib/core/runes/graph.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import NodeDetail from '#lib/features/system-atoms/forge/thinking-graph/NodeDetail.svelte';
    import PhaseDiagram from '#lib/features/system-atoms/forge/thinking-graph/PhaseDiagram.svelte';
    import ThinkingCanvas from '#lib/features/system-atoms/forge/thinking-graph/ThinkingCanvas.svelte';

    const session = $derived(sessionsStack.focused);
    const selected = $derived(graphStack.selected(session?.id ?? null));
    const empty = $derived(session ? graphStack.nodes(session.id).length === 0 : true);

    $effect(() => {
        if (session) void graphStack.load(session.id);
    });
</script>

{#if !session}
    <EmptyState icon={Network} title={t('graph.noSession')} hint={t('graph.noSessionHint')} />
{:else}
    <div class="flex h-full flex-col">
        <header class="flex h-12 shrink-0 items-center gap-3 border-b border-border-subtle px-4">
            <StatusIcon status={session.status} />
            <h2 class="font-semibold">{session.name}</h2>
            <span class="truncate text-meta text-faint">
                {session.now?.label ?? t(`status.${session.status}`)}
            </span>
            <div class="ml-auto flex items-center gap-3">
                <span class="font-mono text-meta text-muted">
                    {session.kpis.turns}
                    {t('unit.turns')} · {session.kpis.toolCalls}
                    {t('unit.steps')}
                </span>
                <Segmented
                    options={[
                        { value: 'diagram', label: t('graph.viewDiagram') },
                        { value: 'timeline', label: t('graph.viewTimeline') },
                    ]}
                    value={graphStack.view}
                    onchange={(view) => graphStack.setView(view)}
                />
                <button
                    class="btn {graphStack.follow ? 'btn-primary' : ''}"
                    onclick={() => (graphStack.follow = !graphStack.follow)}
                    title={t('graph.follow')}
                >
                    <ArrowDownToLine size={13} />
                    {t('graph.follow')}
                </button>
            </div>
        </header>

        <div class="flex min-h-0 flex-1">
            <div class="min-w-0 flex-1">
                {#if empty}
                    <EmptyState
                        icon={Network}
                        title={t('graph.empty')}
                        hint={session.observed ? t('graph.emptyHint') : t('graph.noHooks')}
                    />
                {:else}
                    {#key session.id}
                        {#if graphStack.view === 'diagram'}<PhaseDiagram
                                sessionId={session.id}
                            />{:else}<ThinkingCanvas sessionId={session.id} />{/if}
                    {/key}
                {/if}
            </div>
            {#if selected}<NodeDetail node={selected} />{/if}
        </div>
    </div>
{/if}
