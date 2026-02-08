<script lang="ts">
    import { workStack } from "$lib/core/runes/work-runes.svelte";
    import { toggleUIStack } from "$lib/core/runes/toggle-ui.svelte";
    import { ActivityIDs, ActivityVisual } from "$lib/core/constants/work-config";
    import type { ActivityID } from "$lib/core/types/work";

    const systemsImportRoute = '$lib/systems';
    const blueprintImportRoute = `${systemsImportRoute}/blueprint`;
    const forgeImportRoute = `${systemsImportRoute}/forge`;
    const pulseImportRoute = `${systemsImportRoute}/pulse`;
    const workspaceImportRoute = `${systemsImportRoute}/workspace`;

    const SidebarByActivity: Record<ActivityID, any> = {
        [ActivityIDs.CHAT]: import(`${blueprintImportRoute}/chat/ChatSidebar.svelte`),
        [ActivityIDs.ARTIFACTS]: import(`${blueprintImportRoute}/artifacts/ArtifactsSidebar.svelte`),
        
        [ActivityIDs.THINKING_GRAPH]: import(`${forgeImportRoute}/thinking-graph/ThinkingGraphSidebar.svelte`),
        [ActivityIDs.KANBAN]: import(`${forgeImportRoute}/kanban/KanbanSidebar.svelte`),
        [ActivityIDs.TESTING]: import(`${forgeImportRoute}/testing/TestingSidebar.svelte`),

        [ActivityIDs.FILES]: import(`${pulseImportRoute}/files/FilesSidebar.svelte`),
        [ActivityIDs.SOURCE_CONTROL]: import(`${pulseImportRoute}/source-control/SourceControlSidebar.svelte`),

        [ActivityIDs.WORKFLOW]: import(`${workspaceImportRoute}/workflow/WorkflowSidebar.svelte`),
        [ActivityIDs.CONTEXT]: import(`${workspaceImportRoute}/context/ContextSidebar.svelte`),
        [ActivityIDs.EXTENSIONS]: import(`${workspaceImportRoute}/extensions/ExtensionsSidebar.svelte`),
    };
</script>

{#if toggleUIStack.sidebarOpen}
    <aside class="w-64 bg-sidebar border-r border-border-subtle flex flex-col overflow-hidden transition-all">
        <div class="p-4 text-[10px] font-bold text-text/40 uppercase tracking-widest border-b border-border-subtle">
            {workStack.activeActivity && ActivityVisual[workStack.activeActivity]?.label || 'Sin actividad activa'}
        </div>
        <div class="flex-1 overflow-y-auto">
            {#if workStack.activeActivity && SidebarByActivity[workStack.activeActivity]}
                {#await SidebarByActivity[workStack.activeActivity] then Module}
                    <svelte:component this={Module.default} />
                {/await}
            {/if}
        </div>
    </aside>
{/if}