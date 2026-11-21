import { reviewService } from '#lib/core/services/review-service';
import { sessionsStack } from '#lib/core/runes/sessions.svelte';
import { toastStack } from '#lib/core/runes/toast-runes.svelte';
import type { ChangedFile, FilePair } from '#lib/core/types/files';
import { loadLocal, saveLocal } from '#lib/core/utils/storage';

/** Revisión de los cambios de la sesión enfocada contra su commit base. */
class ReviewStack {
    changes = $state<ChangedFile[]>([]);
    selectedPath = $state<string | null>(null);
    pair = $state<FilePair | null>(null);
    loading = $state(false);
    committing = $state(false);
    /** Cómo se listan los archivos cambiados: plano o agrupados por carpeta. */
    view = $state<'list' | 'tree'>(loadLocal('review.view', 'list'));

    /** Archivos marcados como revisados por sesión. */
    reviewed = $state<Record<string, string[]>>(loadLocal('review.files', {}));

    totals = $derived({
        additions: this.changes.reduce((sum, file) => sum + file.additions, 0),
        deletions: this.changes.reduce((sum, file) => sum + file.deletions, 0),
    });

    setView(view: 'list' | 'tree') {
        this.view = view;
        saveLocal('review.view', view);
    }

    isReviewed(path: string): boolean {
        const id = sessionsStack.focusedId;
        return !!id && (this.reviewed[id] ?? []).includes(path);
    }

    toggleReviewed(path: string) {
        const id = sessionsStack.focusedId;
        if (!id) return;

        const list = this.reviewed[id] ?? [];
        this.reviewed[id] = list.includes(path) ? list.filter((item) => item !== path) : [...list, path];
        saveLocal('review.files', $state.snapshot(this.reviewed));
    }

    /** `selectFile = false` solo actualiza la lista (sin leer el diff), p. ej. para marcar el árbol de archivos. */
    async reload(selectFile = true) {
        const id = sessionsStack.focusedId;
        if (!id) {
            this.changes = [];
            this.pair = null;
            return;
        }

        this.loading = true;
        try {
            this.changes = await reviewService.changes(id);
            if (!selectFile) return;
            // Mantiene el archivo abierto; si ya no existe, salta al primero para revisar sin clics extra.
            const target = this.changes.find((file) => file.path === this.selectedPath) ?? this.changes[0];
            if (target) await this.select(target.path);
            else {
                this.pair = null;
                this.selectedPath = null;
            }
        } catch (error) {
            this.changes = [];
            toastStack.error(error);
        } finally {
            this.loading = false;
        }
    }

    async select(path: string) {
        const id = sessionsStack.focusedId;
        const file = this.changes.find((change) => change.path === path);
        if (!id || !file) return;

        this.selectedPath = path;
        try {
            this.pair = await reviewService.file(id, path, file.status);
        } catch (error) {
            this.pair = null;
            toastStack.error(error);
        }
    }

    async commit(message: string): Promise<boolean> {
        const id = sessionsStack.focusedId;
        if (!id) return false;

        this.committing = true;
        try {
            const sha = await reviewService.commit(id, message);
            toastStack.success(`Commit ${sha}`);
            await this.reload();
            return true;
        } catch (error) {
            toastStack.error(error);
            return false;
        } finally {
            this.committing = false;
        }
    }
}

export const reviewStack = new ReviewStack();
