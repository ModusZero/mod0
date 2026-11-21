<script lang="ts">
    import { Play, Square, SquareTerminal, X } from '@lucide/svelte';
    import EmptyState from '#lib/components/common/EmptyState.svelte';
    import StatusIcon from '#lib/components/common/StatusIcon.svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import { toastStack } from '#lib/core/runes/toast-runes.svelte';
    import { toggleUIStack } from '#lib/core/runes/toggle-ui.svelte';
    import { sessionService } from '#lib/core/services/session-service';
    import Terminal from './Terminal.svelte';

    const focused = $derived(sessionsStack.focused);

    /** Las terminales se crean la primera vez que se ven y luego se conservan. */
    let started = $state<Record<string, boolean>>({});

    $effect(() => {
        if (focused && toggleUIStack.terminalOpen) started[focused.id] = true;
    });

    async function run(action: () => Promise<unknown>) {
        try {
            await action();
        } catch (error) {
            toastStack.error(error);
        }
    }
</script>

<div class="flex h-full flex-col bg-main">
    <header
        class="flex h-8 shrink-0 items-center justify-between border-b border-border-subtle bg-sidebar pr-1 pl-3"
    >
        {#if focused}
            <div class="flex min-w-0 items-center gap-2 text-meta">
                <StatusIcon status={focused.status} size={12} />
                <span class="truncate font-medium text-text">{focused.name}</span>
                <span class="truncate text-faint">{focused.cwd}</span>
            </div>
            <div class="flex items-center">
                {#if focused.status === 'exited'}
                    <button
                        class="btn btn-ghost h-6 text-meta"
                        onclick={() => run(() => sessionService.restart(focused.id))}
                    >
                        <Play size={12} />
                        {focused.claudeSessionId ? t('session.resume') : t('session.restart')}
                    </button>
                {:else}
                    <button
                        class="icon-btn size-6"
                        title={t('session.stop')}
                        onclick={() => run(() => sessionService.stop(focused.id))}
                    >
                        <Square size={12} />
                    </button>
                {/if}
                <button class="icon-btn size-6" onclick={toggleUIStack.toggleTerminal} aria-label="Cerrar"
                    ><X size={13} /></button
                >
            </div>
        {:else}
            <span class="text-meta text-faint">{t('terminal.title')}</span>
            <button class="icon-btn size-6" onclick={toggleUIStack.toggleTerminal} aria-label="Cerrar"
                ><X size={13} /></button
            >
        {/if}
    </header>

    <div class="relative min-h-0 flex-1">
        {#each sessionsStack.sessions as session (session.id)}
            {#if started[session.id]}
                <Terminal
                    {session}
                    visible={session.id === sessionsStack.focusedId && toggleUIStack.terminalOpen}
                />
            {/if}
        {/each}

        {#if !focused}
            <EmptyState icon={SquareTerminal} title={t('terminal.empty')} hint={t('terminal.emptyHint')} />
        {/if}
    </div>
</div>
