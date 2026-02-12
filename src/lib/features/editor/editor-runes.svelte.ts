import { tabsStack } from '../workbench/tabs/tabs-runes.svelte';
import { fileStack } from '../filesystem/files-runes.svelte';
import type { CodeTab } from '$lib/core/types/tab';

/**
 * Controlador de operaciones sobre archivos de código abiertos.
 */
class EditorStack {
    /** Sincroniza el buffer de CodeMirror con el Tab activo si es de tipo código */
    updateContent(content: string): void {
        const active = tabsStack.activeTab;
        if (active?.type === 'code') {
            const file = active as CodeTab;

            if (file.content !== content) {
                file.content = content;
                file.isDirty = true;
            }
        }
    }

    /** Persiste los cambios en disco y resetea el flag isDirty */
    async saveFileChanges(): Promise<void> {
        const active = tabsStack.activeTab;
        const activeFileCode = active?.type === 'code'? active as CodeTab : null;

        if (activeFileCode && activeFileCode.isDirty) {
            await fileStack.saveCurrentFile();
            activeFileCode.isDirty = false;
        }
    }
}

export const editorStack = new EditorStack();