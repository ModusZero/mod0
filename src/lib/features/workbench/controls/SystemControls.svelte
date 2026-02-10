<script lang="ts">
    import { type } from '@tauri-apps/plugin-os';
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { Minus, Square, Copy, X } from "lucide-svelte";

    const osType = type();
    const appWindow = getCurrentWindow();    
    let isMaximized = $state(false);

    $effect(() => {
        let unlisten: (() => void) | undefined;

        const setup = async () => {
            isMaximized = await appWindow.isMaximized();

            const unlistenFn = await appWindow.onResized(async () => {
                isMaximized = await appWindow.isMaximized();
            });
            unlisten = unlistenFn;
        };

        setup();
        return () => { if (unlisten) unlisten(); };
    });

    const handleMinimize = () => appWindow.minimize();
    
    const handleToggleMax = () => appWindow.toggleMaximize();

    const handleClose = () => appWindow.close();

    const btnBase = "h-full px-4 flex items-center justify-center transition-colors text-text/50 hover:text-text hover:bg-black/10! dark:hover:bg-white/10 no-drag";
</script>

{#if osType !== 'macos'}
    <div class="flex items-center h-full no-drag">
        <button 
            type="button"
            aria-label="Minimizar ventana"
            onclick={handleMinimize}
            class={btnBase}
        >
            <Minus size={16} strokeWidth={2} />
        </button>
        
        <button 
            type="button"
            aria-label={isMaximized ? "Restaurar ventana" : "Maximizar ventana"}
            onclick={handleToggleMax} 
            class={btnBase}
        >
            {#if isMaximized}
                <Copy size={14} strokeWidth={2} />
            {:else}
                <Square size={14} strokeWidth={2} />
            {/if}
        </button>

        <button
            type="button"
            aria-label="Cerrar ventana"
            onclick={handleClose} 
            class="{btnBase} hover:bg-red-500/15! hover:text-red-500!"
        >
            <X size={16} strokeWidth={2} />
        </button>
    </div>
{/if}

<style>
    .no-drag {
        -webkit-app-region: no-drag !important;
    }
</style>