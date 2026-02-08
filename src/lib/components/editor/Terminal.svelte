<script lang="ts">
    import { toggleUIStack } from "$lib/runes/toggle-ui.svelte";
    import { X, Terminal as TerminalIcon } from "lucide-svelte";

    let height = $state(200);
    let isResizing = $state(false);

    function startResizing(e: MouseEvent) {
        e.preventDefault();
        isResizing = true;
        document.body.style.cursor = 'ns-resize';
    }

    function stopResizing() {
        if (!isResizing) return;
        isResizing = false;
        document.body.style.cursor = 'default';
    }

    function onMouseMove(e: MouseEvent) {
        if (!isResizing) return;
        
        const newHeight = window.innerHeight - e.clientY;
        
        if (newHeight > 60 && newHeight < window.innerHeight - 100) {
            height = newHeight;
        }
    }
</script>

<svelte:window onmousemove={onMouseMove} onmouseup={stopResizing} />

{#if toggleUIStack.terminalOpen}
    {#if isResizing}
        <div class="fixed inset-0 z-999 cursor-ns-resize"></div>
    {/if}

    <section 
        style="height: {height}px"
        class="flex flex-col bg-main border-t border-border-subtle relative shrink-0 select-none"
    >
        <button 
            aria-label="resizable panel"
            onmousedown={startResizing}
            class="absolute -top-1 left-0 right-0 h-2 cursor-ns-resize z-100 transition-colors hover:bg-accent/30 active:bg-accent"
        ></button>

        <header class="flex items-center justify-between px-4 py-1.5 bg-sidebar/40 border-b border-border-subtle shrink-0">
            <div class="flex items-center gap-2 text-[10px] font-bold text-text/60 uppercase tracking-tight">
                <TerminalIcon size={12} /> <span>Terminal</span>
            </div>
            <button 
                onclick={() => toggleUIStack.terminalOpen = false} 
                class="text-text/40 hover:text-text cursor-pointer p-0.5"
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