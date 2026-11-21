<script lang="ts">
    import { toggleUIStack } from '#lib/core/runes/toggle-ui.svelte';
    import { workStack } from '#lib/core/runes/work-modes.svelte';
    import ResizeHandle from '#lib/components/common/ResizeHandle.svelte';
    import TerminalDock from '#lib/features/terminal/TerminalDock.svelte';
    import EditorMainSpace from '#lib/systems/pulse/editor/EditorMainSpace.svelte';
    import FilesMainSpace from '#lib/systems/pulse/files/FilesMainSpace.svelte';
    import KanbanMainSpace from '#lib/systems/forge/kanban/KanbanMainSpace.svelte';
    import ReviewMainSpace from '#lib/systems/pulse/review/ReviewMainSpace.svelte';
    import SessionsMainSpace from '#lib/systems/forge/sessions/SessionsMainSpace.svelte';
    import SettingsMainSpace from '#lib/systems/workspace/settings/SettingsMainSpace.svelte';
    import ThinkingGraphMainSpace from '#lib/systems/forge/thinking-graph/ThinkingGraphMainSpace.svelte';

    let startHeight = 0;
</script>

<main class="flex min-w-0 flex-1 flex-col overflow-hidden bg-main">
    <section class="min-h-0 flex-1 overflow-hidden">
        {#if workStack.activity === 'sessions'}
            <SessionsMainSpace />
        {:else if workStack.activity === 'thinking-graph'}
            <ThinkingGraphMainSpace />
        {:else if workStack.activity === 'kanban'}
            <KanbanMainSpace />
        {:else if workStack.activity === 'files'}
            <FilesMainSpace />
        {:else if workStack.activity === 'review'}
            <ReviewMainSpace />
        {:else if workStack.activity === 'editor'}
            <EditorMainSpace />
        {:else if workStack.activity === 'settings'}
            <SettingsMainSpace />
        {/if}
    </section>

    <!-- La terminal vive siempre montada (aunque oculta) para no perder el historial de las sesiones. -->
    {#if toggleUIStack.terminalOpen}
        <ResizeHandle
            orientation="horizontal"
            onstart={() => (startHeight = toggleUIStack.terminalHeight)}
            ondrag={(delta) => toggleUIStack.setTerminalHeight(startHeight - delta)}
        />
    {/if}
    <div
        class="shrink-0 overflow-hidden border-t border-border-subtle {toggleUIStack.terminalOpen
            ? ''
            : 'hidden'}"
        style="height:{toggleUIStack.terminalHeight}px"
    >
        <TerminalDock />
    </div>
</main>
