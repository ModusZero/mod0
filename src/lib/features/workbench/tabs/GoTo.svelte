<script lang="ts">
    import { tabsStack } from "$lib/features/workbench/tabs/tabs-runes.svelte";
    import { ChevronLeft, ChevronRight } from "lucide-svelte";
    
    const btnClass = "p-1 rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed";
</script>

<div class="flex items-center gap-0.5">
    <button 
        onclick={() => tabsStack.goBack()}
        disabled={!tabsStack.canGoBack}
        class="{btnClass} {tabsStack.canGoBack ? 'text-text/60 hover:text-text hover:bg-white/5' : 'text-text/30'}"
    >
        <ChevronLeft size={18} />
    </button>
    <button 
        onclick={() => tabsStack.goForward()}
        disabled={!tabsStack.canGoForward}
        class="{btnClass} {tabsStack.canGoForward ? 'text-text/60 hover:text-text hover:bg-white/5' : 'text-text/30'}"
    >
        <ChevronRight size={18} />
    </button>
</div>