import { FSService } from "../services/filesystem-service";
import { tabsStack } from "./tabs.svelte";
import { FileCode } from "lucide-svelte";

export interface FileNode {
    name: string;
    path: string;
    is_dir: boolean;
    children?: FileNode[];
}

class FileStack {
    files = $state<FileNode[]>([]);
    rootPath = $state<string | null>(null);
    isSearching = $state(false);

    async openFolder(path: string) {
        const nodes = await FSService.readFolder(path);
        this.rootPath = path;
        this.files = nodes;
    }

    async openFile(node: FileNode) {
        if (node.is_dir) return;
        const content = await FSService.readFile(node.path);
        
        tabsStack.openTab({
            id: node.path,
            type: 'code',
            label: node.name,
            icon: FileCode,
            color: 'text-blue-400',
            content: content
        });
    }

    async saveCurrentFile() {
        const active = tabsStack.activeTab;
        if (active && active.type === 'code' && active.content !== undefined) {
            await FSService.saveFile(active.id, active.content);
            console.log("Guardado:", active.label);
        }
    }
}

export const fileStack = new FileStack();