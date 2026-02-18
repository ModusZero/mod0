import type { Tab } from "$lib/core/types/tab";

/**
 * Gestiona el stack de pestañas abiertas y el historial de navegación.
 */
class TabsStack {
    openTabs = $state<Tab[]>([]);
    activeTabId = $state<string | null>(null);
    
    #history = $state<string[]>([]);
    #pointer = $state(0);

    /** Retorna el objeto Tab actualmente enfocado */
    get activeTab(): Tab | null {
        return this.openTabs.find(t => t.id === this.activeTabId) || null;
    }

    /** Abre una nueva pestaña o enfoca una existente */
    openTab(tab: Tab) {
        if (!this.openTabs.find(t => t.id === tab.id)) {
            this.openTabs = [...this.openTabs, tab];
        }
        this.navigate(tab.id);
    }

    /** Cambia el foco y actualiza el historial de navegación */
    navigate(id: string, addToHistory = true) {
        if (this.activeTabId === id) return;
        this.activeTabId = id;
        if (addToHistory) {
            this.#history = this.#history.slice(0, this.#pointer + 1);
            this.#history.push(id);
            this.#pointer = this.#history.length - 1;
        }
    }

    /** Cierra una pestaña y recalcula el foco adyacente */
    closeTab(id: string) {
        const index = this.openTabs.findIndex(t => t.id === id);
        this.openTabs = this.openTabs.filter(t => t.id !== id);
        this.#history = this.#history.filter(tid => tid !== id);
        this.#pointer = Math.max(0, this.#history.length - 1);

        if (this.activeTabId === id && this.openTabs.length > 0) {
            this.activeTabId = this.openTabs[index - 1]?.id || this.openTabs[0].id;
        } else if (this.openTabs.length === 0) {
            this.activeTabId = null;
        }
    }
}

export const tabsStack = new TabsStack();