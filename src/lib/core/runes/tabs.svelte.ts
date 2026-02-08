import { ActivityIDs, ActivityVisual } from "$lib/core/constants/work-config";
import type { Tab } from "$lib/core/types/tab";
import { FileCode } from "lucide-svelte";

/* El TabsStack centraliza el estado de las pestañas abiertas en el IDE. */
class TabsStack {
    /** * @type {Tab[]} Lista de pestañas actualmente abiertas en el IDE */
    openTabs: Tab[] = $state<Tab[]>([
        { id: 'file1', type: 'code', label: 'main.js', icon: FileCode, color: 'text-blue-400' },
        { type: 'activity', ...ActivityVisual[ActivityIDs.CHAT] } as Tab,
        { type: 'activity', ...ActivityVisual[ActivityIDs.KANBAN] } as Tab,
    ]);

    activeTabId = $state<string | null>(this.openTabs[0].id);

    /* Getter para obtener la pestaña activa basada en el ID */
    get activeTab() {
        return this.openTabs.find(t => t.id === this.activeTabId) || null;
    }

    /* Métodos para abrir y cerrar pestañas */
    openTab(tab: Tab) {
        if (!this.openTabs.find(t => t.id === tab.id)) {
            this.openTabs.push(tab);
        }
        this.activeTabId = tab.id;
    }

    closeTab(id: string) {
        this.openTabs = this.openTabs.filter(t => t.id !== id);
        if (this.activeTabId === id) {
            this.activeTabId = this.openTabs.length > 0 ? this.openTabs[this.openTabs.length - 1].id : null;
        }
    }
}

/* Instancia única del estado de pestañas para toda la aplicación. */
export const tabsStack = new TabsStack();