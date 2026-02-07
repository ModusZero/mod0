<script lang="ts">
    import { uiStack } from "$lib/runes/ui.svelte";
    import { ActivityIDs, ActivityVisual } from "$lib/constants/ui";
    import type { ActivityID } from "$lib/types/ui";

    const SidebarByActivity: Record<ActivityID, any> = {
        [ActivityIDs.CHAT]: import("$lib/components/layout/activity-sidebars/ChatSidebar.svelte"),
        [ActivityIDs.ARTIFACTS]: import("$lib/components/layout/activity-sidebars/ArtifactsSidebar.svelte"),
        
        [ActivityIDs.THINKING_GRAPH]: import("$lib/components/layout/activity-sidebars/ThinkingGraphSidebar.svelte"),
        [ActivityIDs.KANBAN]: import("$lib/components/layout/activity-sidebars/KanbanSidebar.svelte"),

        [ActivityIDs.FILES]: import("$lib/components/layout/activity-sidebars/FilesSidebar.svelte"),
        [ActivityIDs.SOURCE_CONTROL]: import("$lib/components/layout/activity-sidebars/SourceControlSidebar.svelte"),

        [ActivityIDs.WORKFLOW]: import("$lib/components/layout/activity-sidebars/WorkflowSidebar.svelte"),
        [ActivityIDs.CONTEXT]: import("$lib/components/layout/activity-sidebars/ContextSidebar.svelte"),
        [ActivityIDs.EXTENSIONS]: import("$lib/components/layout/activity-sidebars/ExtensionsSidebar.svelte"),
    };
</script>

{#if uiStack.sidebarOpen}
    <aside class="w-64 bg-sidebar border-r border-border-subtle flex flex-col overflow-hidden transition-all">
        <div class="p-4 text-[10px] font-bold text-text/40 uppercase tracking-widest border-b border-border-subtle">
            {uiStack.activeActivity && ActivityVisual[uiStack.activeActivity]?.label || 'Sin actividad activa'}
        </div>
        <div class="flex-1 overflow-y-auto">
            {#if uiStack.activeActivity && SidebarByActivity[uiStack.activeActivity]}
                {#await SidebarByActivity[uiStack.activeActivity] then Module}
                    <svelte:component this={Module.default} />
                {/await}
            {/if}
        </div>
    </aside>
{/if}