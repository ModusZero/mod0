<script lang="ts">
    import { GripVertical, Pencil, Play, Plus, Trash } from '@lucide/svelte';
    import StatusIcon from '#lib/components/common/StatusIcon.svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { openSession } from '#lib/core/registry/actions';
    import { kanbanStack } from '#lib/core/runes/kanban.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import { toggleUIStack } from '#lib/core/runes/toggle-ui.svelte';
    import type { KanbanColumnID, KanbanTask } from '#lib/core/types/kanban';

    interface Props {
        column: KanbanColumnID;
        onedit: (task: KanbanTask) => void;
    }

    let { column, onedit }: Props = $props();

    let draft = $state('');
    let over = $state(false);

    const tasks = $derived(kanbanStack.inColumn(column));

    function add() {
        if (!draft.trim()) return;
        kanbanStack.add(draft, '', column);
        draft = '';
    }

    function drop(event: DragEvent) {
        event.preventDefault();
        over = false;
        const id = event.dataTransfer?.getData('text/plain');
        if (id) kanbanStack.move(id, column);
    }

    function start(task: KanbanTask) {
        toggleUIStack.openNewSession({
            name: task.title,
            prompt: [task.title, task.notes].filter(Boolean).join('. '),
            taskId: task.id,
        });
    }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<section
    class="flex min-h-0 flex-col rounded-xl border bg-sidebar transition-colors {over
        ? 'border-accent/60 bg-accent-soft'
        : 'border-border-subtle'}"
    ondragover={(event) => {
        event.preventDefault();
        over = true;
    }}
    ondragleave={() => (over = false)}
    ondrop={drop}
>
    <header class="flex h-10 shrink-0 items-center justify-between px-3">
        <h3 class="text-meta font-semibold tracking-wider text-muted uppercase">
            {t(`kanban.${column}`)} <span class="ml-1 font-mono text-faint">{tasks.length}</span>
        </h3>
    </header>

    <div class="min-h-0 flex-1 space-y-2 overflow-y-auto px-2 pb-2">
        {#each tasks as task (task.id)}
            {@const session = sessionsStack.byId(task.sessionId)}
            <article
                class="group rounded-lg border border-border-subtle bg-raised p-2.5"
                draggable="true"
                ondragstart={(event) => event.dataTransfer?.setData('text/plain', task.id)}
            >
                <div class="flex items-start gap-1.5">
                    <GripVertical
                        size={13}
                        class="mt-0.5 shrink-0 cursor-grab text-faint opacity-0 group-hover:opacity-100"
                    />
                    <div class="min-w-0 flex-1">
                        <p class="font-medium break-words">{task.title}</p>
                        {#if task.notes}<p class="mt-0.5 line-clamp-3 text-meta text-muted">
                                {task.notes}
                            </p>{/if}
                    </div>
                </div>

                <div class="mt-2 flex items-center gap-1">
                    {#if session}
                        <button
                            class="flex min-w-0 flex-1 items-center gap-1.5 rounded bg-hover px-1.5 py-0.5 text-meta"
                            onclick={() => openSession(session.id)}
                        >
                            <StatusIcon status={session.status} size={11} />
                            <span class="truncate">{session.name}</span>
                        </button>
                    {:else}
                        <button class="btn btn-ghost h-6 px-1.5 text-meta" onclick={() => start(task)}>
                            <Play size={11} />
                            {t('kanban.start')}
                        </button>
                        <span class="flex-1"></span>
                    {/if}
                    <button
                        class="icon-btn size-6 opacity-0 group-hover:opacity-100"
                        onclick={() => onedit(task)}
                        aria-label={t('common.edit')}><Pencil size={12} /></button
                    >
                    <button
                        class="icon-btn size-6 opacity-0 group-hover:opacity-100 hover:text-danger"
                        onclick={() => kanbanStack.remove(task.id)}
                        aria-label={t('common.delete')}><Trash size={12} /></button
                    >
                </div>
            </article>
        {/each}
    </div>

    <form
        class="flex items-center gap-1 border-t border-border-subtle p-1.5"
        onsubmit={(event) => {
            event.preventDefault();
            add();
        }}
    >
        <input
            class="input h-7 border-transparent bg-transparent"
            placeholder={t('kanban.add')}
            bind:value={draft}
        />
        <button class="icon-btn" type="submit" disabled={!draft.trim()} aria-label={t('common.add')}
            ><Plus size={14} /></button
        >
    </form>
</section>
