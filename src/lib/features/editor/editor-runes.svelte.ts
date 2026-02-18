import { tabsStack } from '../workbench/tabs/tabs-runes.svelte';
import { fileStack } from '../system-atoms/pulse/files/files-runes.svelte';
import type { CodeTab } from '$lib/core/types/tab';
import { LanguageDescription } from '@codemirror/language';
import { languages } from '@codemirror/language-data';

class EditorStack {
    /** * Detecta el lenguaje basado en la extensión, igual que VSCode.
     */
    getLanguageData(fileName: string) {
        const desc = LanguageDescription.matchFilename(languages, fileName);
        return desc;
    }

    updateContent(content: string): void {
        const active = tabsStack.activeTab;
        if (active?.type === 'code') {
            const file = active as CodeTab;
            if (file.content !== content) {
                file.content = content;
                file.isDirty = true;
                fileStack.updateFileContent(file.id, content);
            }
        }
    }

    async saveFileChanges(): Promise<void> {
        const active = tabsStack.activeTab;
        if (active?.type === 'code' && (active as CodeTab).isDirty) {
            await fileStack.saveCurrentFile();
            (active as CodeTab).isDirty = false;
        }
    }

    /**
     * Retorna la extensión de lenguaje cargada dinámicamente
     */
    async getDynamicLanguage(fileName: string) {
        const langDesc = this.getLanguageData(fileName);
        if (langDesc) {
            return await langDesc.load();
        }
        return [];
    }
}

export const editorStack = new EditorStack();