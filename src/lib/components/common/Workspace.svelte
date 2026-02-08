<script lang="ts">
    import { tabsStack } from "$lib/runes/tabs.svelte";
    import TabHeader from "$lib/components/common/TabHeader.svelte";
    import CodeEditor from "$lib/components/editor/CodeEditor.svelte";

    const demoCode = `/**
        * MOD0 SYSTEM v1.0.4 - Core Engine
        * -------------------------------------------
        * Este es un ejemplo extenso para validar el 
        * comportamiento del scroll y la integración de Runes.
        */

        import { configStack } from "$lib/runes/config.svelte";
        import { uiState } from "$lib/runes/ui.svelte";

        class Mod0Engine {
            version = "1.0.4";
            status = $state("initializing");
            logs = $state([]);

            constructor() {
                this.init();
            }

            async init() {
                this.log("Buscando configuración de Tauri...");
                try {
                    // Simulación de carga
                    await new Promise(r => setTimeout(r, 1000));
                    this.status = "ready";
                    this.log("Sistema listo para operar.");
                } catch (err) {
                    this.log("Error de inicialización: " + err.message);
                }
            }

            log(msg) {
                const time = new Date().toLocaleTimeString();
                this.logs.push(\`[\${time}] \${msg}\`);
                console.log(\`[MOD0] \${msg}\`);
            }

            toggleActivity(id) {
                if (ActivityIDs[id]) {
                    uiState.currentActivity = id;
                    this.log("Cambiando a actividad: " + id);
                }
            }
        }

        // Inicialización global
        export const engine = new Mod0Engine();

        /**
         * El sistema ahora permite manejar múltiples UIs 
         * basándose en el ActivityID proporcionado.
         */
        function checkLayoutIntegrity() {
            const panels = ["sidebar", "terminal", "editor"];
            return panels.every(p => document.querySelector(\`#panel-\${p}\`));
        }

        // Event Listeners para el modo oscuro
        $effect(() => {
            const theme = configStack.current.theme;
            engine.log("Aplicando variaciones cromáticas para: " + theme);
        });
        `;
</script>

<div class="flex flex-col h-full w-full overflow-hidden bg-main">
    <nav class="flex h-9 bg-sidebar/20 border-b border-border-subtle overflow-x-auto no-scrollbar shrink-0">
        {#each tabsStack.openTabs as tab}
            <button class="contents" onclick={() => tabsStack.activeTabId = tab.id}>
                <TabHeader 
                    label={tab.label}
                    icon={tab.icon}
                    iconColor={tab.color}
                    active={tabsStack.activeTabId === tab.id}
                    isModified={tab.type === 'code'}
                    onClose={() => tabsStack.closeTab(tab.id)}
                />
            </button>
        {/each}
        <div class="flex-1 border-r border-border-subtle"></div>
    </nav>

    <div class="flex-1 relative min-h-0 bg-main">
        {#if tabsStack.activeTab}
            {#if tabsStack.activeTab.type === 'code'}
                <CodeEditor content={tabsStack.activeTab.content || demoCode} />
            {:else}
                <div class="flex items-center justify-center h-full">
                    <p class="text-[10px] font-mono uppercase tracking-[0.5em] text-text/20">
                        UI: {tabsStack.activeTab.type}
                    </p>
                </div>
            {/if}
        {:else}
            <div class="flex flex-col items-center justify-center h-full gap-4">
                <h1 class="text-4xl font-black opacity-5 select-none">MOD0</h1>
                <p class="text-[10px] text-text/20 uppercase tracking-widest font-bold">Sin archivos abiertos</p>
            </div>
        {/if}
    </div>
</div>