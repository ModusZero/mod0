<script lang="ts">
    import { Plus } from '@lucide/svelte';
    import StatusIcon from '#lib/components/common/StatusIcon.svelte';
    import { StatusStyle } from '#lib/core/constants/work-config';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { openSession } from '#lib/core/registry/actions';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import { toggleUIStack } from '#lib/core/runes/toggle-ui.svelte';
    import { workStack } from '#lib/core/runes/work-modes.svelte';
    import { basename, formatDuration } from '#lib/core/utils/format';
    import type { Session } from '#lib/core/types/session';

    /** Línea de contexto bajo el nombre: lo que importa saber de la sesión de un vistazo. */
    function summary(session: Session): { text: string; tone: string } {
        const since = formatDuration(sessionsStack.clock - session.statusSince);

        switch (session.status) {
            case 'waiting':
                return { text: `${t('status.waitingYou')} · ${since}`, tone: 'text-st-waiting' };
            case 'review':
                return {
                    text: `${session.kpis.filesChanged} ${t('unit.files')} · +${session.kpis.additions} −${session.kpis.deletions}`,
                    tone: 'text-st-review',
                };
            case 'working':
                return {
                    text: session.now?.target
                        ? basename(session.now.target)
                        : (session.now?.label ?? t('status.working')),
                    tone: 'text-muted',
                };
            case 'starting':
                return { text: t('status.starting'), tone: 'text-faint' };
            case 'exited':
                return { text: t('status.exited'), tone: 'text-faint' };
            default:
                return { text: `${t('status.idle')} · ${since}`, tone: 'text-faint' };
        }
    }
</script>

{#if toggleUIStack.railOpen}
    <aside class="flex w-60 shrink-0 flex-col border-l border-border-subtle bg-sidebar">
        <div class="panel-title">
            <span>{t('rail.title')}</span>
            <button
                class="icon-btn size-6"
                title="{t('session.new')} (Ctrl+Shift+N)"
                onclick={() => toggleUIStack.openNewSession()}
            >
                <Plus size={15} />
            </button>
        </div>

        <div class="min-h-0 flex-1 overflow-y-auto px-1.5 pb-2">
            {#each sessionsStack.sessions as session, index (session.id)}
                {@const info = summary(session)}
                {@const focused = sessionsStack.focusedId === session.id}
                <button
                    class="group relative mb-0.5 flex w-full flex-col gap-0.5 rounded-lg py-1.5 pr-2 pl-3 text-left transition-colors {focused
                        ? 'bg-active'
                        : session.status === 'waiting'
                          ? 'bg-st-waiting/8 hover:bg-st-waiting/14'
                          : 'hover:bg-hover'}"
                    onclick={() => sessionsStack.focus(session.id)}
                    ondblclick={() => openSession(session.id)}
                >
                    <span
                        class="absolute top-2 bottom-2 left-0.5 w-0.5 rounded-full"
                        style="background:var(--s{session.colorIdx})"
                    ></span>
                    <span class="flex items-center gap-2">
                        <StatusIcon status={session.status} size={13} />
                        <span
                            class="min-w-0 flex-1 truncate font-medium {session.status === 'exited'
                                ? 'text-muted'
                                : ''}"
                        >
                            {session.name}
                        </span>
                        {#if index < 9}
                            <span
                                class="font-mono text-[0.625rem] text-faint opacity-0 transition-opacity group-hover:opacity-100"
                            >
                                Alt {index + 1}
                            </span>
                        {/if}
                    </span>
                    <span class="truncate pl-5.5 text-meta {info.tone}">{info.text}</span>
                </button>
            {:else}
                <div class="px-3 py-8 text-center text-meta text-faint">{t('rail.empty')}</div>
            {/each}
        </div>

        {#if sessionsStack.counts.waiting + sessionsStack.counts.review > 0}
            <div class="border-t border-border-subtle p-2 text-meta text-muted">
                <span class="kbd">Ctrl Shift A</span>
                <span class="ml-1.5">{t('rail.nextAttention')}</span>
            </div>
        {/if}
    </aside>
{/if}
