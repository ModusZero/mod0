import { FSService } from "../../../../core/services/fs-service";
import { tabsStack } from "../../../workbench/tabs/tabs-runes.svelte";
import { FileCode, File as FileIcon, Folder, ImageIcon, FileText } from "lucide-svelte";
import type { AssetExtension, AssetTab, CodeExtension, CodeTab, GenericFile, Tab } from "$lib/core/types/tab";
import { settingsStack } from "../../../settings/settings-runes.svelte";
import { SvelteSet } from "svelte/reactivity";
import { LANGUAGE_MAP, type EditorLanguage } from "../../../editor/editor-types";
import { ASSET_EXTENSIONS } from '../../../../core/types/tab';

class FileStack {
    files = $state<GenericFile[]>([]);
    rootPath = $state<string | null>(null);
    expandedPaths = $state(new SvelteSet<string>());
    recentFiles = $state(new SvelteSet<GenericFile>());
    previewTabId = $state<string | null>(null);
    unsavedChanges = $state<Record<string, string>>({});
    isSearching = $state(false);
    searchMode = $state<'files' | 'content'>('files');

    constructor() {
        const saved = localStorage.getItem('mod0_expanded_folders');
        if (saved) {
            try {
                this.expandedPaths = new SvelteSet(JSON.parse(saved));
            } catch {
                this.expandedPaths = new SvelteSet();
            }
        }

        $effect.root(() => {
            $effect(() => {
                localStorage.setItem('mod0_expanded_folders', JSON.stringify([...this.expandedPaths]));
            });
        });

        $effect.root(() => {
            $effect(() => {
                // Observamos cambios en unsavedChanges
                const paths = Object.keys(this.unsavedChanges);
                if (paths.length === 0) return;

                // Consultamos settings (Infrastructure ready)
                const autoSaveEnabled = settingsStack.current.auto_save ?? false; 

                if (autoSaveEnabled) {
                    // Aquí iría la lógica de debounce para no colapsar el disco
                    this.saveAllDirtyFiles();
                }
            });
        });
    }

    // --- Gestión de UI ---

    toggleFolder(path: string) {
        if (this.expandedPaths.has(path)) {
            this.expandedPaths.delete(path);
        } else {
            this.expandedPaths.add(path);
        }
    }

    expandAll() {
        const traverse = (nodes: GenericFile[]) => {
            nodes.forEach(n => {
                if (n.is_dir) {
                    this.expandedPaths.add(n.path);
                    if (n.children) traverse(n.children);
                }
            });
        };
        traverse(this.files);
    }

    collapseAll() {
        this.expandedPaths.clear();
    }

    addToHistory(file: GenericFile) {
        if (!this.recentFiles.has(file))
            this.recentFiles.add(file);
    }

    clearHistory() {
        this.recentFiles.clear();
    }

    // --- Lógica de Pestañas y Caché ---

    promotePreview(path: string) {
        if (this.previewTabId === path) {
            this.previewTabId = null;
        }
    }

    updateFileContent(path: string, newContent: string) {
        this.unsavedChanges[path] = newContent;
        this.promotePreview(path);
        
        const tab = tabsStack.openTabs.find(t => t.id === path);
        if (tab && tab.type === 'code') {
            (tab as CodeTab).content = newContent;
            (tab as CodeTab).isDirty = true;
        }
    }

    async openFile(node: GenericFile, isPreview = true) {
        if (node.is_dir) return;

        // Si ya está abierta, solo navegamos
        const activeTab = tabsStack.openTabs.find(t => t.id === node.path);
        if (activeTab) {
            tabsStack.navigate(node.path);
            if (!isPreview) this.promotePreview(node.path);
            return;
        }

        const ext = node.path.split('.').pop()?.toLowerCase() || '';

        const editorLang = LANGUAGE_MAP[ext as CodeExtension] as EditorLanguage;
        const isAsset = ASSET_EXTENSIONS.has(ext as AssetExtension);

        let newTab: Tab;

        if (editorLang) {
        // --- Caso: ARCHIVO DE CÓDIGO ---
            const content = this.unsavedChanges[node.path] ?? await FSService.readFile(node.path);
            newTab = {
                ...node,
                id: node.path,
                type: 'code',
                label: node.name,
                icon: FileCode,
                color: 'text-blue-400',
                content,
                language: editorLang,
                isDirty: !!this.unsavedChanges[node.path]
            } as CodeTab;
        } else if (isAsset) {
            // --- Caso: ASSET (IMAGEN/BINARIO) ---
            newTab = {
                ...node,
                id: node.path,
                type: 'asset',
                label: node.name,
                icon: ImageIcon,
                color: 'text-purple-400'
            } as AssetTab;
        } else {
            // --- Caso: ARCHIVO GENÉRICO (Tratar como texto plano por defecto) ---
            const content = this.unsavedChanges[node.path] ?? await FSService.readFile(node.path);
            newTab = {
                ...node,
                id: node.path,
                type: 'code',
                label: node.name,
                icon: FileText,
                color: 'text-gray-400',
                content,
                language: 'plaintext',
                isDirty: !!this.unsavedChanges[node.path]
            } as CodeTab;
        }

        // Manejo de Preview
        if (isPreview) {
            if (this.previewTabId) tabsStack.closeTab(this.previewTabId);
            this.previewTabId = newTab.id;
        } else {
            this.previewTabId = null;
        }

        tabsStack.openTab(newTab);
        this.addToHistory(node);
    }

    async saveCurrentFile() {
        const active = tabsStack.activeTab;
        if (active?.type === 'code') {
            const file = active as CodeTab;
            await FSService.saveFile(file.path, file.content);
            delete this.unsavedChanges[file.path];
            file.isDirty = false;
        }
    }

    async saveAllDirtyFiles() {
        for (const path in this.unsavedChanges) {
            await FSService.saveFile(path, this.unsavedChanges[path]);
            delete this.unsavedChanges[path];
            
            const tab = tabsStack.openTabs.find(t => t.id === path);
            if (tab && tab.type === 'code') (tab as CodeTab).isDirty = false;
        }
    }

    // --- CRUD de Sistema de Archivos ---

    async refresh() {
        if (this.rootPath) await this.openFolder(this.rootPath);
    }

    async createNewFile(parentPath?: string) {
        const name = prompt("File name:");
        if (!name) return;
        const targetDir = parentPath || this.rootPath;
        const path = `${targetDir}/${name}`;
        await FSService.createFile(path);
        await this.refresh();
        // Abrir automáticamente el archivo recién creado
        await this.openFile({ path, name, is_dir: false, extension: name.split('.').pop() || '' } as any, false);
    }

    async createNewFolder(parentPath?: string) {
        const name = prompt("Folder name:");
        if (!name) return;
        const targetDir = parentPath || this.rootPath;
        const path = `${targetDir}/${name}`;
        await FSService.createDir(path);
        if (targetDir) this.expandedPaths.add(targetDir);
        await this.refresh();
    }

    async rename(node: GenericFile) {
        const newName = prompt("Rename to:", node.name);
        if (!newName || newName === node.name) return;
        
        const parentDir = node.path.substring(0, node.path.lastIndexOf('/'));
        const newPath = `${parentDir}/${newName}`;
        
        await FSService.renameFile(node.path, newPath);
        
        // Actualizar Tabs y Caché
        const tabIndex = tabsStack.openTabs.findIndex(t => t.id === node.path);
        if (tabIndex !== -1) {
            const tab = tabsStack.openTabs[tabIndex];
            tab.id = newPath;
            tab.label = newName;
            (tab as any).path = newPath;
            if (this.previewTabId === node.path) this.previewTabId = newPath;
        }

        if (this.unsavedChanges[node.path]) {
            this.unsavedChanges[newPath] = this.unsavedChanges[node.path];
            delete this.unsavedChanges[node.path];
        }

        await this.refresh();
    }

    async delete(node: GenericFile) {
        if (!confirm(`Delete ${node.name}?`)) return;

        await FSService.deleteFile(node.path);
        
        // Si el archivo estaba en el Set de expandidos (si era dir)
        this.expandedPaths.delete(node.path);
        
        // Cerrar tab si existe
        tabsStack.closeTab(node.path);
        delete this.unsavedChanges[node.path];
        if (this.previewTabId === node.path) this.previewTabId = null;
        
        await this.refresh();
    }

    async openFolder(path: string) {
        try {
            const rawFiles = await FSService.readFolder(path);
            this.files = this._processAndSortNodes(rawFiles);
            this.rootPath = path;
            settingsStack.update({ last_project_path: path });
        } catch (err) {
            console.error("FS Error:", err);
        }
    }

    // --- Internals ---

    private _getLanguage(ext: string): EditorLanguage {
        const map: Record<string, EditorLanguage> = {
            'rs': 'rust',
            'ts': 'typescript',
            'js': 'javascript',
            'svelte': 'svelte',
            'json': 'json',
            'md': 'markdown'
        };
        return map[ext] || 'javascript';
    }

    private _processAndSortNodes(nodes: any[]): GenericFile[] {
        if (!nodes) return [];
        return nodes.map(node => {
            const ext = node.name.split('.').pop()?.toLowerCase() || '';
            const commonProps = {
                ...node,
                label: node.name,
                extension: ext,
                icon: node.is_dir ? Folder : FileIcon,
                color: node.is_dir ? "text-accent/60" : "text-text/40",
            };

            if (node.is_dir) {
                return {
                    ...commonProps,
                    children: node.children ? this._processAndSortNodes(node.children) : []
                };
            }

            return commonProps;
        }).sort((a, b) => (b.is_dir ? 1 : 0) - (a.is_dir ? 1 : 0) || a.name.localeCompare(b.name));
    }
}

export const fileStack = new FileStack();