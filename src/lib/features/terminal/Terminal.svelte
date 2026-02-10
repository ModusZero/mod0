<script lang="ts">
    import { X, Terminal as TerminalIcon } from "lucide-svelte";
    import { toggleUIStack } from "$lib/core/runes/toggle-ui.svelte";
    import ResizablePanel from "$lib/components/common/ResizablePanel.svelte";
</script>

{#if toggleUIStack.terminalOpen}
    <ResizablePanel 
        direction="vertical" 
        side="bottom" 
        initialSize={200} 
        minSize={80}
        maxSize={600}
    >
        <section 
            class="flex flex-col bg-main border-t border-border-subtle h-full w-full relative shrink-0 select-none"
        >
            <header class="flex items-center justify-between px-4 py-1.5 bg-sidebar/40 border-b border-border-subtle shrink-0">
                <div class="flex items-center gap-2 text-[10px] font-bold text-text/60 uppercase tracking-tight">
                    <TerminalIcon size={12} /> <span>Terminal</span>
                </div>
                <button 
                    onclick={() => toggleUIStack.terminalOpen = false} 
                    class="text-text/40 hover:text-text cursor-pointer p-0.5 transition-colors"
                >
                    <X size={14} />
                </button>
            </header>

            <div class="p-4 font-mono text-xs text-text/80 space-y-1 overflow-y-auto flex-1 custom-terminal-scroll select-text">
                <p class="text-accent">mod0@system:~$ <span class="text-text">npm run dev</span></p>
                <p class="text-green-500 font-bold"> > mod0-ide@1.0.4 dev</p>
                
                <div class="flex gap-2 items-center">
                    <span class="text-accent">mod0@system:~$</span>
                    <input 
                        type="text" 
                        class="bg-transparent outline-none flex-1 border-none p-0 text-text" 
                        spellcheck="false"
                    />
                </div>
            </div>
        </section>
    </ResizablePanel>
{/if}

<style>
    .custom-terminal-scroll::-webkit-scrollbar {
        width: 6px;
    }
    .custom-terminal-scroll::-webkit-scrollbar-track {
        background: transparent;
    }
    .custom-terminal-scroll::-webkit-scrollbar-thumb {
        background: rgba(128, 128, 128, 0.2);
        border-radius: 10px;
    }
    .custom-terminal-scroll::-webkit-scrollbar-thumb:hover {
        background: rgba(128, 128, 128, 0.4);
    }
</style>