<script lang="ts">
    import { SquareKanban } from '@lucide/svelte';
    import Dialog from '#lib/components/common/Dialog.svelte';
    import EmptyState from '#lib/components/common/EmptyState.svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { filesStack } from '#lib/core/runes/files.svelte';
    import { kanbanColumns, kanbanStack } from '#lib/core/runes/kanban.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import type { KanbanTask } from '#lib/core/types/kanban';
    import KanbanColumn from '#lib/features/system-atoms/forge/kanban/KanbanColumn.svelte';

    let editing = $state<KanbanTask | null>(null);
    let title = $state('');
    let notes = $state('');

    $effect(() => {
        void kanbanStack.load(filesStack.projectPath);
    });

    // Las tareas ligadas a una sesión se mueven solas según el estado de esta.
    $effect(() => {
        kanbanStack.sync(sessionsStack.sessions);
    });

    function edit(task: KanbanTask) {
        editing = task;
        title = task.title;
        notes = task.notes;
    }

    function save() {
        if (editing && title.trim())
            kanbanStack.update(editing.id, { title: title.trim(), notes: notes.trim() });
        editing = null;
    }
</script>

{#if !filesStack.projectPath}
    <EmptyState icon={SquareKanban} title={t('kanban.noProject')} hint={t('kanban.noProjectHint')} />
{:else}
    <div class="grid h-full grid-cols-4 gap-3 p-4">
        {#each kanbanColumns as column (column)}
            <KanbanColumn {column} onedit={edit} />
        {/each}
    </div>
{/if}

<Dialog open={editing !== null} title={t('kanban.editTask')} onclose={() => (editing = null)}>
    <div class="space-y-3">
        <div>
            <label class="label" for="kb-title">{t('kanban.taskTitle')}</label>
            <input id="kb-title" class="input" bind:value={title} />
        </div>
        <div>
            <label class="label" for="kb-notes">{t('kanban.taskNotes')}</label>
            <textarea id="kb-notes" class="input" rows="5" bind:value={notes}></textarea>
        </div>
    </div>
    {#snippet footer()}
        <button class="btn" onclick={() => (editing = null)}>{t('common.cancel')}</button>
        <button class="btn btn-primary" onclick={save}>{t('common.save')}</button>
    {/snippet}
</Dialog>
