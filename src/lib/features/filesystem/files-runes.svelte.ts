import { FSService } from "../../core/services/filesystem-service";
import { tabsStack } from "../workbench/tabs/tabs-runes.svelte";
import { FileCode, File as FileIcon, Folder } from "lucide-svelte";
import type { AssetTab, CodeTab, GenericFile } from "$lib/core/types/tab";

/**
 * FileStack orquesta el estado del sistema de archivos y la visibilidad del explorador.
 */
class FileStack {
    files = $state<GenericFile[]>([]);
    rootPath = $state<string | null>(null);
    /** Controla la visibilidad de la barra de búsqueda en la UI del Sidebar */
    isSearching = $state(false);

    /** Lee el directorio y ordena los resultados (Directorios primero) */
    async openFolder(path: string) {
        // 1. Recibimos los datos puros de Rust
        const rawFiles = await FSService.readFolder(path);
        // 2. Los transformamos y ordenamos para que la UI los entienda
        this.files = this._processAndSortNodes(rawFiles);
        this.rootPath = path;
    }

    /**
     * Transforma los FileNodes de Rust en GenericFiles (agrega icon, label, etc)
     * y los ordena (carpetas primero).
     */
    private _processAndSortNodes(nodes: any[]): GenericFile[] {
        if (!nodes) return [];

        return nodes
            .map(node => {
                const enrichedNode: GenericFile = {
                    ...node,
                    label: node.name,
                    icon: node.is_dir ? Folder : FileIcon, // Icono por defecto
                    color: node.is_dir ? "text-accent/60" : "text-text/40",
                    children: node.children ? this._processAndSortNodes(node.children) : (node.is_dir ? [] : undefined)
                };
                return enrichedNode;
            })
            .sort((a, b) => {
                if (a.is_dir && !b.is_dir) return -1;
                if (!a.is_dir && b.is_dir) return 1;
                return a.name.localeCompare(b.name);
            });
    }

    async openFile(node: GenericFile) {
        if (node.is_dir) return;
        const ext = node.name.split('.').pop()?.toLowerCase() || '';
        const isText = ['rs','js','ts','svelte','json','md','css','html','txt'].includes(ext);

        if (isText) {
            const content = await FSService.readFile(node.path);
            const tab: CodeTab = { 
                ...node, 
                id: node.path, 
                type: 'code', 
                label: node.name, 
                icon: FileCode, 
                color: 'text-blue-400', 
                content, 
                language: 'javascript',
                isDirty: false 
            };
            tabsStack.openTab(tab);
        } else {
            const tab: AssetTab = { 
                ...node, id: node.path, type: 'asset', label: node.name, 
                icon: FileIcon, color: 'text-gray-400' 
            };
            tabsStack.openTab(tab);
        }
    }

    async saveCurrentFile() {
        const active = tabsStack.activeTab;
        if (active?.type === 'code') {
            await FSService.saveFile(active.id, (active as CodeTab).content);
            (active as CodeTab).isDirty = false;
        }
    }
}

export const fileStack = new FileStack();