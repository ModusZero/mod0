import type { EditorFile } from '$lib/types/file-editor';

/* El EditorStack centraliza el estado de los archivos abiertos en el editor de código. */
class EditorStack {
    /* Lista de archivos actualmente abiertos en el editor */
    tabs = $state<EditorFile[]>([]);

    /* ID del archivo actualmente activo en el editor */
    activeFileId = $state<string | null>(null);

    /* Getter para obtener el archivo activo basado en el ID */
    activeFile = $derived(
        this.tabs.find(t => t.id === this.activeFileId) || null
    );

    /* Métodos para abrir y cerrar archivos, y actualizar su contenido */
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

/* Instancia única del estado del editor para toda la aplicación. */
export const editorStack = new EditorStack();