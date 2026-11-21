<script lang="ts">
    import { CheckCheck, FileCode, FileDiff, GitCommitHorizontal } from '@lucide/svelte';
    import EmptyState from '#lib/components/common/EmptyState.svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { editorStack } from '#lib/core/runes/editor.svelte';
    import { reviewStack } from '#lib/core/runes/review.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import { toastStack } from '#lib/core/runes/toast-runes.svelte';
    import { sessionService } from '#lib/core/services/session-service';
    import { joinPath } from '#lib/core/utils/format';
    import DiffView from '#lib/features/editor/DiffView.svelte';

    let message = $state('');

    const session = $derived(sessionsStack.focused);
    const pair = $derived(reviewStack.pair);

    // Sugiere el nombre de la sesión como mensaje de commit.
    $effect(() => {
        message = session ? session.name : '';
    });

    async function commit() {
        if (await reviewStack.commit(message)) message = '';
    }

    async function approve() {
        if (!session) return;
        try {
            await sessionService.setReviewed(session.id, true);
        } catch (error) {
            toastStack.error(error);
        }
    }
</script>

{#if !session}
    <EmptyState icon={FileDiff} title={t('review.noSession')} hint={t('review.noSessionHint')} />
{:else}
    <div class="flex h-full flex-col">
        <header class="flex h-12 shrink-0 items-center gap-3 border-b border-border-subtle px-4">
            {#if pair}
                <span class="font-mono text-meta font-bold">{pair.status}</span>
                <h2 class="min-w-0 flex-1 truncate font-medium" title={pair.path}>{pair.path}</h2>
                {#if pair.status !== 'D'}
                    <button class="btn" onclick={() => editorStack.open(joinPath(session.cwd, pair.path))}>
                        <FileCode size={13} />
                        {t('node.open')}
                    </button>
                {/if}
            {:else}
                <h2 class="flex-1 font-medium text-muted">{t('review.pick')}</h2>
            {/if}
        </header>

        <div class="min-h-0 flex-1">
            {#if pair}
                {#key `${pair.path}:${pair.original.length}:${pair.modified.length}`}
                    <DiffView path={pair.path} original={pair.original} modified={pair.modified} />
                {/key}
            {:else}
                <EmptyState
                    icon={FileDiff}
                    title={reviewStack.changes.length === 0 ? t('review.noChanges') : t('review.pick')}
                    hint={t('review.base')}
                />
            {/if}
        </div>

        {#if reviewStack.changes.length > 0}
            <footer class="flex shrink-0 items-center gap-2 border-t border-border-subtle bg-sidebar p-2.5">
                <input
                    class="input flex-1"
                    bind:value={message}
                    placeholder={t('review.commitMessage')}
                    onkeydown={(event) => event.key === 'Enter' && commit()}
                />
                <button class="btn" onclick={approve} disabled={session.reviewed}>
                    <CheckCheck size={13} />
                    {t('review.approve')}
                </button>
                <button
                    class="btn btn-primary"
                    onclick={commit}
                    disabled={reviewStack.committing || !message.trim()}
                >
                    <GitCommitHorizontal size={13} />
                    {t('review.commit')}
                </button>
            </footer>
        {/if}
    </div>
{/if}
