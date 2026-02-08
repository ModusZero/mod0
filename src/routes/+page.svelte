<script lang="ts">
    import CodeEditor from "$lib/components/editor/CodeEditor.svelte";
    import TabHeader from "$lib/components/common/TabHeader.svelte";
    import { ActivityVisual, ActivityIDs } from "$lib/constants/ui-config";
    import { FileCode } from "lucide-svelte";

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

    // Mock de pestañas abiertas (esto irá a un rune global después)
    let openTabs = $state([
        { id: 'file1', type: 'code', label: 'main.js', icon: FileCode, color: 'text-blue-400', active: true },
        { type: 'activity', ...ActivityVisual[ActivityIDs.CHAT], active: false },
        { type: 'activity', ...ActivityVisual[ActivityIDs.KANBAN], active: false }
    ]);
</script>

<div class="flex flex-col h-full w-full overflow-hidden">
    <nav class="flex h-9 bg-sidebar/40 border-b border-border-subtle overflow-x-auto no-scrollbar">
        {#each openTabs as tab}
            <TabHeader 
                label={tab.label}
                icon={tab.icon}
                iconColor={tab.color}
                active={tab.active}
                isModified={tab.type === 'code'}
                onClose={() => openTabs = openTabs.filter(t => t !== tab)}
            />
        {/each}
        <div class="flex-1"></div>
    </nav>

    <div class="flex-1 relative min-h-0 bg-main">
        {#if openTabs.find(t => t.active)?.type === 'code'}
            <CodeEditor content={demoCode} />
        {:else}
            <div class="flex items-center justify-center h-full text-text/20">
                <p class="text-sm font-mono uppercase tracking-[0.3em]">Activity UI Placeholder</p>
            </div>
        {/if}
    </div>
</div>

<style>
    .no-scrollbar::-webkit-scrollbar {
        display: none;
    }
    .no-scrollbar {
        -ms-overflow-style: none;
        scrollbar-width: none;
    }
</style>