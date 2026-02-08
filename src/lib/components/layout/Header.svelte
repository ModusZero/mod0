<script lang="ts">
    import { fade } from 'svelte/transition';
    import lightLogo from "$lib/assets/logo-light.svg";
    import darkLogo from "$lib/assets/logo-dark.svg";
    import { settingsStack } from "$lib/features/settings/settings-runes.svelte";
    import { toggleUIStack } from "$lib/core/runes/toggle-ui.svelte";
    import ModeSelector from "../../features/workbench/command-center/ModeSelector.svelte";
    
    const headerOptions: { label: string; func: () => void }[] = [
        { label: "Archivo", func: () => console.log("Archivo") },
        { label: "Editar", func: () => console.log("Editar") },
        { label: "Ver", func: () => console.log("Ver") },
        { label: "Terminal", func: () => toggleUIStack.toggleTerminal() },
        { label: "Opciones", func: () => toggleUIStack.toggleSettings() },
    ];
</script>

<header class="h-10 border-b border-border-subtle flex items-center px-4 bg-sidebar/50 backdrop-blur-md justify-between select-none relative z-50">
    <div class="flex flex-1 justify-start items-center gap-4">
        
        <div class="relative flex h-8 w-8 items-center justify-center overflow-hidden rounded-lg p-0.5 group hover:scale-110 transition-transform duration-300">
            <div class="relative h-full w-full flex items-center justify-center">
                {#if settingsStack.current.theme === 'dark'}
                    <div in:fade={{ duration: 200 }} class="absolute inset-0">
                        <img 
                            src={darkLogo} 
                            alt="Logo Dark" 
                            class="h-full w-full object-contain" 
                            loading="eager" 
                        />
                    </div>
                {:else}
                    <div in:fade={{ duration: 200 }} class="absolute inset-0">
                        <img 
                            src={lightLogo} 
                            alt="Logo Light" 
                            class="h-full w-full object-contain brightness-0" 
                            loading="eager" 
                        />
                    </div>
                {/if}
            </div>
        </div>

        <div class="flex gap-4 text-[11px] font-medium text-text/50">
            {#each headerOptions as option}
                <button
                    onclick={option.func}
                    class="hover:text-text/80 transition-colors cursor-pointer"
                    >
                    {option.label}
                </button>
            {/each}
        </div>
    </div>

    <div class="absolute left-1/2 -translate-x-1/2">
        <ModeSelector />
    </div>

    <div class="flex items-center gap-3">
        <span class="text-[9px] opacity-30 font-black tracking-[0.3em] text-text uppercase">MOD0</span>
        <div class="w-2 h-2 rounded-full bg-accent shadow-[0_0_8px_var(--accent)] animate-pulse"></div>
    </div>
</header>