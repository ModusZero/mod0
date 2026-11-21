<script lang="ts">
    import { LayoutGrid, Plus } from '@lucide/svelte';
    import EmptyState from '#lib/components/common/EmptyState.svelte';
    import Segmented from '#lib/components/common/Segmented.svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { graphStack } from '#lib/core/runes/graph.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import { toggleUIStack } from '#lib/core/runes/toggle-ui.svelte';
    import SessionCard from '#lib/features/system-atoms/forge/sessions/SessionCard.svelte';

    const counts = $derived(sessionsStack.counts);

    const visible = $derived(
        sessionsStack.sessions.filter((session) => {
            switch (sessionsStack.filter) {
                case 'attention':
                    return session.status === 'waiting' || session.status === 'review';
                case 'working':
                    return session.status === 'working' || session.status === 'starting';
                case 'idle':
                    return session.status === 'idle' || session.status === 'exited';
                default:
                    return true;
            }
        }),
    );

    // Los rastros de las tarjetas necesitan el grafo de cada sesión.
    $effect(() => {
        for (const session of sessionsStack.sessions) void graphStack.load(session.id);
    });
</script>

<div class="flex h-full flex-col">
    <header class="flex h-12 shrink-0 items-center justify-between gap-4 border-b border-border-subtle px-4">
        <Segmented
            options={[
                { value: 'all', label: t('filter.all'), badge: sessionsStack.sessions.length },
                { value: 'attention', label: t('filter.attention'), badge: counts.waiting + counts.review },
                { value: 'working', label: t('filter.working'), badge: counts.working + counts.starting },
                { value: 'idle', label: t('filter.idle'), badge: counts.idle + counts.exited },
            ]}
            value={sessionsStack.filter}
            onchange={(value) => (sessionsStack.filter = value)}
        />
        <button class="btn btn-primary" onclick={() => toggleUIStack.openNewSession()}>
            <Plus size={14} />
            {t('session.new')}
        </button>
    </header>

    <div class="min-h-0 flex-1 overflow-y-auto p-4">
        {#if sessionsStack.sessions.length === 0}
            <EmptyState icon={LayoutGrid} title={t('sessions.empty')} hint={t('sessions.emptyHint')}>
                <button class="btn btn-primary" onclick={() => toggleUIStack.openNewSession()}>
                    <Plus size={14} />
                    {t('session.new')}
                </button>
            </EmptyState>
        {:else if visible.length === 0}
            <p class="py-16 text-center text-muted">{t('sessions.noMatch')}</p>
        {:else}
            <div class="grid gap-3" style="grid-template-columns:repeat(auto-fill,minmax(19.5rem,1fr))">
                {#each visible as session (session.id)}
                    <SessionCard {session} position={sessionsStack.sessions.indexOf(session)} />
                {/each}
            </div>
        {/if}
    </div>
</div>
