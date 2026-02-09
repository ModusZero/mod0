<script lang="ts">
    import { type Snippet } from 'svelte';
    import { tabsStack } from "$lib/core/runes/tabs.svelte";
    import { fly } from "svelte/transition";

    let { children }: { children: Snippet } = $props();

    let lastIdx = $state(tabsStack.openTabs.findIndex(t => t.id === tabsStack.activeTabId));
    let direction = $state(20);

    $effect(() => {
        const newIdx = tabsStack.openTabs.findIndex(t => t.id === tabsStack.activeTabId);
        
        if (newIdx !== lastIdx) {
            direction = newIdx > lastIdx ? 20 : -20;
            lastIdx = newIdx;
        }
    });
</script>

<div class="relative w-full h-full overflow-hidden">
    {#key tabsStack.activeTabId}
        <div 
            in:fly={{ x: direction, duration: 200, opacity: 0 }} 
            class="absolute inset-0 w-full h-full"
        >
            {@render children()}
        </div>
    {/key}
</div>