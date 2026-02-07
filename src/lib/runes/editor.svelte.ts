import type { EditorFile } from '$lib/types/editor';

class EditorStack {
    tabs = $state<EditorFile[]>([]);
    activeFileId = $state<string | null>(null);

    activeFile = $derived(
        this.tabs.find(t => t.id === this.activeFileId) || null
    );

    openFile(file: EditorFile) {
        if (!this.tabs.find(t => t.id === file.id)) {
            this.tabs.push(file);
        }
        this.activeFileId = file.id;
    }

    closeFile(id: string) {
        this.tabs = this.tabs.filter(t => t.id !== id);
        if (this.activeFileId === id) {
            this.activeFileId = this.tabs[0]?.id || null;
        }
    }

    updateContent(content: string) {
        if (this.activeFile) {
            this.activeFile.content = content;
            this.activeFile.isDirty = true;
        }
    }
}

export const editorStack = new EditorStack();