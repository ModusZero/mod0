<script lang="ts">
    import { tabsStack } from "$lib/features/workbench/tabs/tabs-runes.svelte";
    import CodeEditor from "$lib/features/editor/CodeEditor.svelte";
    import TabManager from "$lib/features/workbench/tabs/TabManager.svelte";
    import TabSwitcher from "$lib/features/workbench/tabs/TabSwitcher.svelte";
    import { type CodeTab } from "$lib/core/types/tab";
    import { settingsStack } from "$lib/features/settings/settings-runes.svelte";
    import darkLogo from "$lib/assets/logo-dark.svg";
    import lightLogo from "$lib/assets/logo-light.svg";
</script>

<div class="flex flex-col h-full w-full overflow-hidden bg-main">
    {#if tabsStack.activeTab}
        <nav class="flex h-9 bg-sidebar/20 border-b border-border-subtle shrink-0">
            <TabManager />
            <div class="flex-1 border-border-subtle"></div>
        </nav>

        <div class="flex-1 relative min-h-0 bg-main">
            <TabSwitcher>
                {#if tabsStack.activeTab.type === 'code'}
                    <CodeEditor tab={(tabsStack.activeTab as CodeTab)} />
                {:else}
                    <div class="flex flex-col items-center justify-center h-full gap-2">
                        <div class="p-4 rounded-full bg-accent/5">
                            <svelte:component this={tabsStack.activeTab.icon} size={32} class={tabsStack.activeTab.color} />
                        </div>
                        <p class="text-[10px] font-mono uppercase tracking-[0.5em] text-text/20">
                            Activity: {tabsStack.activeTab.label}
                        </p>
                    </div>
                {/if}
                
                </TabSwitcher>
            </div>
        {:else}
            <div class="flex flex-col items-center justify-center h-full gap-4">
                <h1 class="text-4xl font-black opacity-50 select-none tracking-tighter">MOD0</h1>

                <img 
                    src={settingsStack.current.theme === 'dark' ? darkLogo : lightLogo} 
                    alt="Logo" 
                    class="h-40 w-40 opacity-40" 
                />

                <div class="flex flex-col items-center gap-1">
                    <p class="text-[10px] text-text/50 uppercase tracking-widest font-bold">Sin archivos abiertos</p>
                    <p class="text-[9px] text-text/30 font-mono">Presiona Ctrl+P para buscar</p>
                </div>
            </div>
        {/if}
</div>