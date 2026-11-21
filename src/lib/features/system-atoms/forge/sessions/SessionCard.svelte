<script lang="ts">
    import { FileDiff, Network, Pencil, Play, Square, Trash } from '@lucide/svelte';
    import StatusIcon from '#lib/components/common/StatusIcon.svelte';
    import { FamilyVisual } from '#lib/core/constants/family-config';
    import { StatusStyle } from '#lib/core/constants/work-config';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { graphStack } from '#lib/core/runes/graph.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import { settingsStack } from '#lib/core/runes/settings.svelte';
    import { toastStack } from '#lib/core/runes/toast-runes.svelte';
    import { workStack } from '#lib/core/runes/work-modes.svelte';
    import { sessionService } from '#lib/core/services/session-service';
    import { windowService } from '#lib/core/services/window-service';
    import type { Session } from '#lib/core/types/session';
    import { basename, cn, formatCompact, formatDuration } from '#lib/core/utils/format';

    interface Props {
        session: Session;
        position: number;
    }

    let { session, position }: Props = $props();

    let editing = $state(false);
    let draft = $state('');

    const focused = $derived(sessionsStack.focusedId === session.id);
    const family = $derived(FamilyVisual[session.now?.kind ?? 'think']);
    const planDone = $derived(session.plan.filter((item) => item.status === 'completed').length);
    const planNow = $derived(session.plan.find((item) => item.status === 'in_progress'));
    const trail = $derived(graphStack.trail(session.id));
    const since = $derived(formatDuration(sessionsStack.clock - session.statusSince));
    const contextRatio = $derived(
        Math.min(1, session.kpis.contextTokens / settingsStack.config.contextLimit),
    );
    const contextTone = $derived(
        contextRatio > 0.85 ? 'bg-danger' : contextRatio > 0.6 ? 'bg-st-waiting' : 'bg-accent',
    );

    /** Línea principal: lo más relevante según el estado. */
    const headline = $derived.by(() => {
        if (session.status === 'waiting') return t('status.waitingYou');
        if (session.status === 'working' && session.now) return session.now.label;
        return session.lastMessage ?? t(`status.${session.status}`);
    });

    async function run(action: () => Promise<unknown>) {
        try {
            await action();
        } catch (error) {
            toastStack.error(error);
        }
    }

    async function remove() {
        const confirmed = await windowService.confirm(
            t('session.removeConfirm', { name: session.name }),
            'MOD0',
        );
        if (!confirmed) return;

        const withWorktree =
            session.worktree && (await windowService.confirm(t('session.removeWorktreeConfirm'), 'MOD0'));
        await run(() => sessionService.remove(session.id, withWorktree));
    }

    function open(activity: 'thinking-graph' | 'review') {
        sessionsStack.focus(session.id);
        workStack.setActivity(activity);
    }

    async function commitRename() {
        editing = false;
        if (draft.trim() && draft.trim() !== session.name)
            await run(() => sessionService.rename(session.id, draft));
    }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions, a11y_no_noninteractive_element_interactions -->
<article
    class={cn(
        'group relative flex flex-col gap-2.5 overflow-hidden rounded-xl border bg-sidebar p-3 pl-4 transition-colors',
        focused ? 'border-accent/50 bg-raised' : 'border-border-subtle hover:bg-raised',
        session.status === 'waiting' && 'border-st-waiting/50',
    )}
    onclick={() => sessionsStack.focus(session.id)}
    ondblclick={() => open('thinking-graph')}
>
    <span class="absolute inset-y-0 left-0 w-1" style="background:var(--s{session.colorIdx})"></span>

    <header class="flex items-center gap-2">
        <StatusIcon status={session.status} />
        {#if editing}
            <!-- svelte-ignore a11y_autofocus -->
            <input
                class="input h-6 flex-1"
                bind:value={draft}
                autofocus
                onclick={(event) => event.stopPropagation()}
                onblur={commitRename}
                onkeydown={(event) =>
                    event.key === 'Enter' ? commitRename() : event.key === 'Escape' && (editing = false)}
            />
        {:else}
            <h3 class="min-w-0 flex-1 truncate text-[0.9375rem] font-semibold">{session.name}</h3>
        {/if}
        {#if position < 9}<span class="font-mono text-[0.625rem] text-faint">Alt {position + 1}</span>{/if}
    </header>

    <p class="-mt-1.5 flex items-center gap-1.5 truncate text-meta text-faint">
        {session.profileName}
        {#if session.branch}<span>·</span><span class="truncate font-mono">{session.branch}</span>{/if}
        {#if session.worktree}<span class="rounded bg-hover px-1">worktree</span>{/if}
    </p>

    <div class="flex min-h-9 items-start gap-2 text-ui">
        {#if session.status === 'working' && session.now}
            <family.icon size={14} class="mt-0.5 shrink-0 {family.text}" />
            <div class="min-w-0">
                <p class="truncate">{session.now.label}</p>
                {#if session.now.target}<p class="truncate font-mono text-meta text-faint">
                        {basename(session.now.target)}
                    </p>{/if}
            </div>
        {:else}
            <p
                class="line-clamp-2 {session.status === 'waiting'
                    ? 'font-medium text-st-waiting'
                    : 'text-muted'}"
            >
                {headline}
            </p>
        {/if}
    </div>

    {#if session.plan.length > 0}
        <div class="flex items-center gap-2 text-meta text-muted" title={t('plan.title')}>
            <span class="h-1 w-14 shrink-0 overflow-hidden rounded-full bg-hover">
                <span class="block h-full bg-ok" style="width:{(planDone / session.plan.length) * 100}%"
                ></span>
            </span>
            <span class="font-mono">{planDone}/{session.plan.length}</span>
            <span class="min-w-0 truncate">{planNow?.content ?? ''}</span>
        </div>
    {/if}

    <div class="flex h-3 items-end gap-0.75" aria-hidden="true">
        {#each trail as node (node.id)}
            <span
                class="h-full w-1.5 rounded-sm {node.status === 'running' ? 'anim-pulse' : ''}"
                style="background:{node.status === 'error'
                    ? 'var(--danger)'
                    : `var(--f-${node.family})`};opacity:{node.status === 'running' ? 1 : 0.7}"
                title={node.label}
            ></span>
        {:else}
            <span class="h-px w-full bg-border-subtle"></span>
        {/each}
    </div>

    <footer class="flex items-center gap-3 font-mono text-meta text-muted">
        <span class={StatusStyle[session.status].text}>{since}</span>
        <span title={t('kpi.turns')}>↻ {session.kpis.turns}</span>
        <span title={t('kpi.changes')}>
            <span class="text-ok">+{session.kpis.additions}</span>
            <span class="text-danger">−{session.kpis.deletions}</span>
        </span>
        {#if session.kpis.errors > 0}<span class="text-danger" title={t('kpi.errors')}
                >⚠ {session.kpis.errors}</span
            >{/if}
        {#if session.kpis.contextTokens > 0}
            <span class="ml-auto flex items-center gap-1.5" title={t('kpi.context')}>
                <span class="h-1 w-10 overflow-hidden rounded-full bg-hover">
                    <span class="block h-full {contextTone}" style="width:{contextRatio * 100}%"></span>
                </span>
                {formatCompact(session.kpis.contextTokens)}
            </span>
        {/if}
    </footer>

    <div
        class="absolute top-2 right-2 flex items-center gap-0.5 rounded-lg bg-raised/95 p-0.5 opacity-0 shadow transition-opacity group-focus-within:opacity-100 group-hover:opacity-100"
    >
        <button
            class="icon-btn size-6"
            title={t('activity.thinking')}
            onclick={(event) => {
                event.stopPropagation();
                open('thinking-graph');
            }}><Network size={13} /></button
        >
        <button
            class="icon-btn size-6"
            title={t('activity.review')}
            onclick={(event) => {
                event.stopPropagation();
                open('review');
            }}><FileDiff size={13} /></button
        >
        <button
            class="icon-btn size-6"
            title={t('session.rename')}
            onclick={(event) => {
                event.stopPropagation();
                draft = session.name;
                editing = true;
            }}><Pencil size={13} /></button
        >
        {#if session.status === 'exited'}
            <button
                class="icon-btn size-6"
                title={t('session.resume')}
                onclick={(event) => {
                    event.stopPropagation();
                    void run(() => sessionService.restart(session.id));
                }}><Play size={13} /></button
            >
        {:else}
            <button
                class="icon-btn size-6"
                title={t('session.stop')}
                onclick={(event) => {
                    event.stopPropagation();
                    void run(() => sessionService.stop(session.id));
                }}><Square size={13} /></button
            >
        {/if}
        <button
            class="icon-btn size-6 hover:text-danger"
            title={t('session.remove')}
            onclick={(event) => {
                event.stopPropagation();
                void remove();
            }}><Trash size={13} /></button
        >
    </div>
</article>
