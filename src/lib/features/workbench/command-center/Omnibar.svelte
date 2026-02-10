<script lang="ts">
    import { Search } from "lucide-svelte";
    import { workStack } from "$lib/core/runes/work-runes.svelte";
    import { WorkSectionVisual } from "$lib/core/constants/work-config";
    import FloatingPanel from "$lib/components/common/low-level/FloatingPanel.svelte";
    import ModeSelector from "./ModeSelector.svelte";
    import ThemeSelector from "./ThemeSelector.svelte";
    import { fly, fade } from 'svelte/transition';

    let isOpen = $state(false);
    let view = $state<'commands' | 'themes' | 'modes'>('commands');
    let step = $state(0); 
    let showGlint = $state(false);
    let searchTerm = $state("");
    let anchorEl = $state<HTMLElement | null>(null);
    let searchInput = $state<HTMLInputElement | null>(null);
    
    const current = $derived(WorkSectionVisual[workStack.mode]);

    const commands = Array(20).fill(null).map((_, i) => ({
        icon: i % 2 === 0 ? "📄" : "⚙️",
        label: i === 1 ? "Change Theme..." : `Command Option ${i + 1}`,
        shortcut: i % 3 === 0 ? "Ctrl P" : null,
        action: i === 1 ? () => (view = 'themes') : () => console.log(`Action ${i}`)
    }));

    function handleOpen() {
        isOpen = true;
        view = 'commands';
        
        // El secreto: Step 1 ocurre casi instantáneo para preparar la forma
        setTimeout(() => { 
            step = 1; 
            showGlint = true;
        }, 50);

        setTimeout(() => { 
            step = 2; 
            searchInput?.focus();
        }, 150); 

        setTimeout(() => { showGlint = false; }, 800);
    }

    function handleClose() {
        step = 0;
        showGlint = false;
        setTimeout(() => { 
            isOpen = false; 
            view = 'commands'; 
            searchTerm = ""; 
        }, 200);
    }
</script>

<div class="relative flex items-center justify-center h-9 w-9 mx-auto">
    {#if !isOpen}
        <div class="absolute inset-0 pulse-wave pointer-events-none"></div>
        <div class="absolute inset-0 pulse-wave pointer-events-none anim-delay-300"></div>
    {/if}

    <button 
        bind:this={anchorEl}
        onclick={handleOpen}
        class="badge-trigger relative z-10 flex items-center justify-center w-9 h-9 rounded-full transition-all duration-300 text-text bg-main/80 dark:bg-main/30 border border-white/10 shadow-lg active:scale-95"
        class:opacity-0={isOpen}
    >
        <div class="icon-container"><current.icon size={18} /></div>
    </button>
</div>

{#if isOpen}
    {#if view === 'modes'}
        <ModeSelector {anchorEl} onclose={handleClose} />
    {:else}
        <FloatingPanel {anchorEl} onclose={handleClose} offset={0}>
            <div class="omni-wrapper">
                <div 
                    class="omni-panel-integrated text-text bg-main dark:bg-main border-white/10 shadow-[0_20px_50px_rgba(0,0,0,0.15)] dark:shadow-[0_20px_50px_rgba(0,0,0,0.4)]"
                    class:step-horizontal={step >= 1} 
                    class:step-vertical={step >= 2}
                >
                    {#if showGlint}
                        <div class="katana-glint"></div>
                    {/if}

                    <div class="flex items-center h-12 px-4 gap-3 border-b border-white/5 relative z-20 w-full text-text bg-main/80 dark:bg-main/30 transition-opacity duration-200" 
                         class:opacity-0={step < 1}
                         class:pointer-events-none={step < 1}>
                        <button onclick={() => view = 'modes'} class="text-accent hover:scale-110 active:scale-90 transition-all p-1">
                            <current.icon size={18} />
                        </button>
                        <div class="flex-1 flex items-center gap-2">
                            <Search size={16} class="text-neutral-500" />
                            <input 
                                bind:this={searchInput} 
                                bind:value={searchTerm}
                                placeholder={view === 'themes' ? "Search themes..." : "Type a command..."} 
                                class="bg-transparent border-none outline-none text-[13px] w-full text-text placeholder:italic" 
                            />
                        </div>
                    </div>

                    <div class="command-list-container overflow-y-auto custom-scroll relative z-10">
                        {#if step === 2}
                            <div class="py-1">
                                {#if view === 'commands'}
                                    <div in:fade={{ duration: 150 }}>
                                        {#each commands as cmd, i}
                                            <button onclick={cmd.action} in:fly={{ y: 8, delay: i * 8 }}
                                                class="w-full flex items-center gap-4 px-4 py-2 hover:bg-accent/10 transition-colors group">
                                                <span class="text-base opacity-70 group-hover:opacity-100 transition-transform group-hover:scale-110">{cmd.icon}</span>
                                                <span class="flex-1 text-left text-[12px] text-text/80 group-hover:text-text">{cmd.label}</span>
                                                {#if cmd.shortcut}<span class="text-[10px] font-mono opacity-30 group-hover:opacity-60">{cmd.shortcut}</span>{/if}
                                            </button>
                                        {/each}
                                    </div>
                                {:else if view === 'themes'}
                                    <div in:fade={{ duration: 150 }}>
                                        <ThemeSelector onBack={() => view = 'commands'} onSelect={handleClose} />
                                    </div>
                                {/if}
                            </div>
                        {/if}
                    </div>
                </div>
            </div>
        </FloatingPanel>
    {/if}
{/if}

<style>
    .omni-wrapper { 
        position: absolute; 
        left: 50%; 
        transform: translateX(-50%); 
        width: 600px; 
        display: flex; 
        justify-content: center; 
        pointer-events: none; 
        perspective: 1000px; /* Evita el efecto de papel doblado */
    }

    .omni-panel-integrated { 
        pointer-events: auto; 
        width: 36px; 
        height: 36px; 
        border-radius: 9999px; 
        border: 1px solid rgba(150, 150, 150, 0.2);
        background-clip: padding-box; /* Fix para bordes extraños */
        
        /* Ajuste de transición: border-radius más rápido que el tamaño */
        transition: 
            width 0.2s cubic-bezier(0.4, 0, 0.2, 1), 
            height 0.25s cubic-bezier(0.4, 0, 0.2, 1),
            border-radius 0.12s ease-out; 
            
        overflow: hidden; 
        position: relative;
        transform: translateZ(0); /* Aceleración GPU */
    }

    /* Expansión horizontal */
    .step-horizontal { 
        width: 480px; 
        border-radius: 12px; 
    }
    
    /* Expansión vertical */
    .step-vertical { 
        height: 350px; 
    } 

    .command-list-container { height: calc(350px - 48px); }

    .katana-glint {
        position: absolute;
        inset: -100%;
        background: linear-gradient(
            80deg, 
            transparent 40%, 
            rgba(150, 150, 150, 0.4) 48%, 
            rgba(255, 255, 255, 0.8) 50%, 
            rgba(150, 150, 150, 0.4) 52%, 
            transparent 60%
        );
        filter: blur(2px);
        z-index: 50;
        pointer-events: none;
        animation: slash 2s cubic-bezier(0.2, 0.8, 0.2, 1) forwards;
    }

    @keyframes slash {
        0% { transform: translateX(-50%) translateY(-50%); opacity: 0; }
        20% { opacity: 1; }
        100% { transform: translateX(50%) translateY(50%); opacity: 0; }
    }

    .pulse-wave { 
        border-radius: 9999px; 
        background: var(--accent, #3b82f6); 
        opacity: 0; 
        animation: wave 1.2s infinite; 
        position: absolute; 
        inset: 0; 
    }
    @keyframes wave { 
        0% { transform: scale(1); opacity: 0.5; filter: blur(2px); } 
        100% { transform: scale(2.2); opacity: 0; filter: blur(8px); } 
    }

    .custom-scroll::-webkit-scrollbar { width: 4px; }
    .custom-scroll::-webkit-scrollbar-thumb { 
        background: rgba(155, 155, 155, 0.2); 
        border-radius: 10px; 
    }
</style>