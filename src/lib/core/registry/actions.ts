import { windowService } from '#lib/core/services/window-service';
import { t } from '#lib/core/i18n/utils.svelte';
import { filesStack } from '#lib/core/runes/files.svelte';
import { sessionsStack } from '#lib/core/runes/sessions.svelte';
import { toastStack } from '#lib/core/runes/toast-runes.svelte';
import { toggleUIStack } from '#lib/core/runes/toggle-ui.svelte';
import { workStack } from '#lib/core/runes/work-modes.svelte';

/** Acciones compartidas entre botones, atajos y la paleta de comandos. */

export async function openFolder() {
    try {
        const path = await windowService.pickFolder(t('project.open'));
        if (path) await filesStack.openProject(path);
    } catch (error) {
        toastStack.error(error);
    }
}

/** Salta a la siguiente sesión que necesita atención y abre la vista adecuada. */
export function jumpToAttention() {
    const session = sessionsStack.focusNextAttention();
    if (!session) {
        toastStack.info(t('toast.nothingPending'));
        return;
    }

    if (session.status === 'review') workStack.setActivity('review');
    else {
        toggleUIStack.terminalOpen = true;
        workStack.setActivity('thinking-graph');
    }
}

/** Enfoca una sesión y, si no se está viendo una vista de sesión, abre su grafo. */
export function openSession(id: string) {
    sessionsStack.focus(id);
    if (!['thinking-graph', 'review', 'sessions'].includes(workStack.activity)) {
        workStack.setActivity('thinking-graph');
    }
}
