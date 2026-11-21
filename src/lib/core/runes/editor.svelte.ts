import { ActivityIDs } from '#lib/core/constants/work-config';
import { fsService } from '#lib/core/services/fs-service';
import { workStack } from '#lib/core/runes/work-modes.svelte';
import { settingsStack } from '#lib/core/runes/settings.svelte';
import { toastStack } from '#lib/core/runes/toast-runes.svelte';
import type { EditorTab } from '#lib/core/types/tab';
import { basename } from '#lib/core/utils/format';

/** Archivos abiertos en el editor. */
class EditorStack {
    tabs = $state<EditorTab[]>([]);
    activePath = $state<string | null>(null);
    private saveTimer: ReturnType<typeof setTimeout> | undefined;

    active = $derived(this.tabs.find((tab) => tab.path === this.activePath) ?? null);

    isDirty(tab: EditorTab): boolean {
        return tab.content !== tab.saved;
    }

    /** Abre un archivo en el editor (y cambia a esa actividad). */
    async open(path: string) {
        workStack.setActivity(ActivityIDs.EDITOR);
        this.activePath = path;
        if (this.tabs.some((tab) => tab.path === path)) return;

        try {
            const content = await fsService.read(path);
            this.tabs.push({ path, name: basename(path), content, saved: content, error: null });
        } catch (error) {
            toastStack.error(error);
            this.activePath = this.tabs.at(-1)?.path ?? null;
        }
    }

    setContent(path: string, content: string) {
        const tab = this.tabs.find((existing) => existing.path === path);
        if (!tab) return;
        tab.content = content;

        // Guardado automático: espera a que se deje de escribir.
        if (settingsStack.config.autoSave) {
            clearTimeout(this.saveTimer);
            this.saveTimer = setTimeout(() => void this.save(path), 1000);
        }
    }

    async save(path: string | null = this.activePath) {
        const tab = this.tabs.find((existing) => existing.path === path);
        if (!tab || !this.isDirty(tab)) return;

        try {
            await fsService.write(tab.path, tab.content);
            tab.saved = tab.content;
        } catch (error) {
            toastStack.error(error);
        }
    }

    close(path: string) {
        const index = this.tabs.findIndex((tab) => tab.path === path);
        if (index === -1) return;

        this.tabs.splice(index, 1);
        if (this.activePath === path) this.activePath = this.tabs[Math.max(0, index - 1)]?.path ?? null;
    }
}

export const editorStack = new EditorStack();
