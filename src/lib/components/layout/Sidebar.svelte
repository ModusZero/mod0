<script lang="ts">
    import { ActivitiesWithSidebar } from '#lib/core/constants/work-config';
    import { toggleUIStack } from '#lib/core/runes/toggle-ui.svelte';
    import { workStack } from '#lib/core/runes/work-modes.svelte';
    import ResizeHandle from '#lib/components/common/ResizeHandle.svelte';
    import FilesSidebar from '#lib/systems/pulse/files/FilesSidebar.svelte';
    import ReviewSidebar from '#lib/systems/pulse/review/ReviewSidebar.svelte';
    import SessionsSidebar from '#lib/systems/forge/sessions/SessionsSidebar.svelte';
    import SettingsSidebar from '#lib/systems/workspace/settings/SettingsSidebar.svelte';
    import ThinkingGraphSidebar from '#lib/systems/forge/thinking-graph/ThinkingGraphSidebar.svelte';

    const visible = $derived(toggleUIStack.sidebarOpen && ActivitiesWithSidebar.includes(workStack.activity));

    let startWidth = 0;
</script>

{#if visible}
    <aside
        class="flex shrink-0 flex-col overflow-hidden border-r border-border-subtle bg-sidebar"
        style="width:{toggleUIStack.sidebarWidth}px"
    >
        {#if workStack.activity === 'sessions'}
            <SessionsSidebar />
        {:else if workStack.activity === 'thinking-graph'}
            <ThinkingGraphSidebar />
        {:else if workStack.activity === 'files'}
            <FilesSidebar />
        {:else if workStack.activity === 'review'}
            <ReviewSidebar />
        {:else if workStack.activity === 'settings'}
            <SettingsSidebar />
        {/if}
    </aside>
    <ResizeHandle
        orientation="vertical"
        onstart={() => (startWidth = toggleUIStack.sidebarWidth)}
        ondrag={(delta) => toggleUIStack.setSidebarWidth(startWidth + delta)}
    />
{/if}
