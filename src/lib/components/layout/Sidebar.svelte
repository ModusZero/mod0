<script lang="ts">
    import { workStack } from "$lib/core/runes/work-modes.svelte";
    import { toggleUIStack } from "$lib/core/runes/toggle-ui.svelte";
    import { ActivityIDs, ActivityVisual } from "$lib/core/constants/work-config";
    import type { ActivityID } from "$lib/core/types/work";
    import ResizablePanel from "../common/ResizablePanel.svelte";

    const SidebarByActivity: Record<ActivityID, () => Promise<any>> = {
        [ActivityIDs.CHAT]: () => import('$lib/systems/blueprint/chat/ChatSidebar.svelte'),
        [ActivityIDs.ARTIFACTS]: () => import('$lib/systems/blueprint/artifacts/ArtifactsSidebar.svelte'),
        
        [ActivityIDs.THINKING_GRAPH]: () => import('$lib/systems/forge/thinking-graph/ThinkingGraphSidebar.svelte'),
        [ActivityIDs.KANBAN]: () => import('$lib/systems/forge/kanban/KanbanSidebar.svelte'),
        [ActivityIDs.TESTING]: () => import('$lib/systems/forge/testing/TestingSidebar.svelte'),

        [ActivityIDs.FILES]: () => import('$lib/systems/pulse/files/FilesSidebar.svelte'),
        [ActivityIDs.SOURCE_CONTROL]: () => import('$lib/systems/pulse/source-control/SourceControlSidebar.svelte'),

        [ActivityIDs.WORKFLOW]: () => import('$lib/systems/workspace/workflows/WorkflowsSidebar.svelte'),
        [ActivityIDs.CONTEXT]: () => import('$lib/systems/workspace/context/ContextSidebar.svelte'),
        [ActivityIDs.PLUGINS]: () => import('$lib/systems/workspace/plugins/PluginsSidebar.svelte'),
    };
</script>

{#if toggleUIStack.sidebarOpen}
    <ResizablePanel 
        direction="horizontal" 
        side="left" 
        initialSize={256} 
        minSize={160} 
        maxSize={600}
    >
        <aside class="h-full w-full bg-sidebar border-r border-border-subtle flex flex-col overflow-hidden">
            <div class="p-4 text-[10px] font-bold text-text/40 uppercase tracking-widest border-b border-border-subtle shrink-0">
                {workStack.activeActivity && ActivityVisual[workStack.activeActivity]?.label || 'Sin actividad activa'}
            </div>

            <div class="flex-1 overflow-y-auto">
                {#if workStack.activeActivity && SidebarByActivity[workStack.activeActivity]}
                    {#await SidebarByActivity[workStack.activeActivity]()}
                        <div class="p-4 animate-pulse text-[10px] text-text/20">Cargando...</div>
                    {:then Module}
                        <svelte:component this={Module.default} />
                    {:catch error}
                        <div class="p-4 text-[10px] text-red-400/50">Error al cargar sidebar</div>
                        {console.error("Error loading sidebar for activity", workStack.activeActivity, error)}
                    {/await}
                {/if}
            </div>
        </aside>
    </ResizablePanel>
{/if}