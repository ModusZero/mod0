import type { Tab } from "$lib/core/types/tab";
import { FileCode, MessageSquare, Terminal } from "lucide-svelte";

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

/** Clase que maneja el estado de las pestañas abiertas, la pestaña activa y el historial de navegación entre pestañas. */
class TabsStack {
    /** Lista de pestañas actualmente abiertas con placeholders */
    openTabs = $state<Tab[]>([
        { id: 'file1', type: 'code', label: 'main.js', icon: FileCode, color: 'text-blue-400', content: demoCode },
        { id: 'chat', type: 'activity', label: 'Chat AI', icon: MessageSquare, color: 'text-purple-400' },
        { id: 'logs', type: 'activity', label: 'System Logs', icon: Terminal, color: 'text-green-400' },
    ]);

    /** Id de la pestaña actualmente activa */
    activeTabId = $state<string | null>('file1');
    
    /** Historial de navegación entre pestañas */
    #history = $state<string[]>(['file1']);

    /** Índice actual en el historial de navegación */
    #pointer = $state(0);

    get activeTab() {
        return this.openTabs.find(t => t.id === this.activeTabId) || null;
    }

    /** Indica si es posible navegar hacia atrás en el historial de pestañas. */
    get canGoBack() { return this.#pointer > 0; }

    /** Indica si es posible navegar hacia adelante en el historial de pestañas. */
    get canGoForward() { return this.#pointer < this.#history.length - 1; }

    /** Navega a una pestaña específica y opcionalmente agrega esta acción al historial de navegación. */
    navigate(id: string, addToHistory = true) {
        if (this.activeTabId === id) return;
        this.activeTabId = id;
        if (addToHistory) {
            this.#history = this.#history.slice(0, this.#pointer + 1);
            this.#history.push(id);
            this.#pointer = this.#history.length - 1;
        }
    }

    /** Navega a la pestaña anterior en el historial de navegación, si es posible. */
    goBack() {
        if (this.canGoBack) {
            this.#pointer--;
            this.activeTabId = this.#history[this.#pointer];
        }
    }

    /** Navega a la pestaña siguiente en el historial de navegación, si es posible. */
    goForward() {
        if (this.canGoForward) {
            this.#pointer++;
            this.activeTabId = this.#history[this.#pointer];
        }
    }

    /** Abre una nueva pestaña y la navega, agregando esta acción al historial de navegación. */
    openTab(tab: Tab) {
        const exists = this.openTabs.find(t => t.id === tab.id);
        if (!exists) {
            this.openTabs = [...this.openTabs, tab];
        }
        this.navigate(tab.id);
    }

    /** Cierra una pestaña específica y actualiza el historial de navegación en consecuencia. */
    closeTab(id: string) {
        const indexToClose = this.openTabs.findIndex(t => t.id === id);
        this.openTabs = this.openTabs.filter(t => t.id !== id);
        this.#history = this.#history.filter(tid => tid !== id);
        this.#pointer = Math.max(0, this.#history.length - 1);

        if (this.activeTabId === id) {
            if (this.openTabs.length > 0) {
                // Navegar a la pestaña de la izquierda si existe, si no a la primera disponible
                const nextTab = this.openTabs[indexToClose - 1] || this.openTabs[0];
                this.activeTabId = nextTab.id;
            } else {
                this.activeTabId = null;
            }
        }
    }
}

/** Instancia global del stack de pestañas para ser utilizada en toda la aplicación. */
export const tabsStack = new TabsStack();