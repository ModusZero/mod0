<script lang="ts">
    import { settingsStack } from "$lib/features/settings/settings-runes.svelte";
    import darkLogo from "$lib/assets/logo-dark.svg";
    import lightLogo from "$lib/assets/logo-light.svg";
    import MainMenu from "$lib/features/workbench/controls/MainMenu.svelte";
    import GoTo from "$lib/features/workbench/tabs/GoTo.svelte";
    import Omnibar from "$lib/features/workbench/command-center/Omnibar.svelte";
    import LayoutConfig from "$lib/features/workbench/controls/LayoutConfig.svelte";
    import SystemControls from "$lib/features/workbench/controls/SystemControls.svelte";
    import { type } from '@tauri-apps/plugin-os';

    const osType = type();
</script>

<header 
    class="header-drag h-11 border-b border-border-subtle flex items-center justify-between bg-main/80 dark:bg-main/30 backdrop-blur-2xl relative z-50 select-none w-full shadow-sm"
>
    <div class="flex items-center gap-2 flex-1 h-full pointer-events-none {osType === 'macos' ? 'pl-20' : 'pl-3'}">
        <div class="flex items-center gap-2 pointer-events-auto">
            <img 
                src={settingsStack.current.theme === 'dark' ? darkLogo : lightLogo} 
                alt="Logo" 
                class="h-5 w-5 opacity-70 hover:opacity-100 hover:scale-105 transition-all duration-300" 
            />
            <MainMenu />
        </div>
    </div>

    <div class="flex items-center justify-center gap-4 flex-2 h-full pointer-events-none">
        <div class="flex items-center gap-4 pointer-events-auto w-full max-w-2xl justify-center">
			<GoTo />

			<div class="w-full max-w-md">
                <Omnibar />
            </div>

			<LayoutConfig />
        </div>
    </div>

    <div class="flex items-center justify-end flex-1 h-full pointer-events-none">
        <div class="pointer-events-auto h-full flex items-center">
            <SystemControls />
        </div>
    </div>
</header>

<style>
    .header-drag {
        -webkit-app-region: drag;
    }

    :global(.no-drag), 
    header :global(button), 
    header :global(input),
    header :global(img) {
        -webkit-app-region: no-drag;
        pointer-events: auto;
    }
</style>