<script lang="ts">
    import { flip } from "svelte/animate";
    import { type CodeTab } from "$lib/core/types/tab";
    import { tabsStack } from "$lib/features/workbench/tabs/tabs-runes.svelte";
    import TabHeader from "./TabHeader.svelte";

    function handleWheel(e: WheelEvent) {
        if (e.deltaY !== 0) {
            const container = e.currentTarget as HTMLElement;
            container.scrollLeft += e.deltaY;
            e.preventDefault();
        }
    }
</script>

<div 
    onwheel={handleWheel}
    class="h-9 flex items-center bg-sidebar/20 border-b border-border-subtle overflow-x-auto overflow-y-hidden no-scrollbar select-none w-full"
    >
    {#each tabsStack.openTabs as tab (tab.id)}
        <div animate:flip={{ duration: 200 }} class="h-full flex shrink-0">
            <button 
                class="contents"
                onclick={() => tabsStack.navigate(tab.id)}
                >
                <TabHeader 
                    id={tab.id}
                    label={tab.label}
                    icon={tab.icon}
                    iconColor={tab.color}
                    active={tabsStack.activeTabId === tab.id}
                    isModified={tab.type === 'code' && (tab as CodeTab).isDirty} 
                    onClose={() => tabsStack.closeTab(tab.id)}
                    />
            </button>
        </div>
    {/each}
</div>  
      
<style>
    .no-scrollbar::-webkit-scrollbar { display: none; }
    .no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
</style>