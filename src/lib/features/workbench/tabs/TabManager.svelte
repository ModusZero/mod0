<script lang="ts">
    import { tabsStack } from "$lib/core/runes/tabs.svelte";
    import TabHeader from "./TabHeader.svelte";
    import { flip } from "svelte/animate";

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
                onclick={() => tabsStack.navigate(tab.id)}>
                <TabHeader 
                    label={tab.label}
                    icon={tab.icon}
                    iconColor={tab.color}
                    active={tabsStack.activeTabId === tab.id}
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