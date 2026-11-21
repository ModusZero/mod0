import { jumpToAttention } from '#lib/core/registry/actions';
import { ActivityIDs } from '#lib/core/constants/work-config';
import { sessionsStack } from '#lib/core/runes/sessions.svelte';
import { settingsStack } from '#lib/core/runes/settings.svelte';
import { toggleUIStack } from '#lib/core/runes/toggle-ui.svelte';
import { workStack } from '#lib/core/runes/work-modes.svelte';
import type { ActivityID } from '#lib/core/types/work';

const ACTIVITY_SHORTCUTS: ActivityID[] = [
    ActivityIDs.SESSIONS,
    ActivityIDs.THINKING_GRAPH,
    ActivityIDs.KANBAN,
    ActivityIDs.FILES,
    ActivityIDs.REVIEW,
    ActivityIDs.EDITOR,
];

/** Atajo que la app maneja aunque el foco esté dentro de la terminal. */
type Binding = { test: (event: KeyboardEvent) => boolean; run: () => void; global: boolean };

const ctrl = (event: KeyboardEvent) => (event.ctrlKey || event.metaKey) && !event.altKey;
const key = (event: KeyboardEvent) => event.key.toLowerCase();

const bindings: Binding[] = [
    // Sesiones
    { test: (e) => e.altKey && !e.ctrlKey && /^[1-9]$/.test(e.key), run: () => {}, global: true },
    { test: (e) => ctrl(e) && e.shiftKey && key(e) === 'a', run: jumpToAttention, global: true },
    {
        test: (e) => ctrl(e) && e.shiftKey && key(e) === 'n',
        run: () => toggleUIStack.openNewSession(),
        global: true,
    },
    {
        test: (e) => ctrl(e) && !e.shiftKey && key(e) === 'n',
        run: () => toggleUIStack.openNewSession(),
        global: false,
    },

    // Navegación
    {
        test: (e) => ctrl(e) && e.shiftKey && key(e) === 'p',
        run: () => toggleUIStack.openPalette(),
        global: true,
    },
    {
        test: (e) => ctrl(e) && !e.shiftKey && key(e) === 'k',
        run: () => toggleUIStack.openPalette(),
        global: false,
    },
    {
        test: (e) => ctrl(e) && e.shiftKey && key(e) === 'o',
        run: () => toggleUIStack.openPalette('files'),
        global: true,
    },
    {
        test: (e) => ctrl(e) && !e.shiftKey && key(e) === 'p',
        run: () => toggleUIStack.openPalette('files'),
        global: false,
    },
    {
        test: (e) => ctrl(e) && e.key === ',',
        run: () => workStack.setActivity(ActivityIDs.SETTINGS),
        global: true,
    },
    { test: (e) => ctrl(e) && !e.shiftKey && /^[1-6]$/.test(e.key), run: () => {}, global: true },

    // Paneles
    { test: (e) => ctrl(e) && e.key === '`', run: toggleUIStack.toggleTerminal, global: true },
    { test: (e) => ctrl(e) && e.shiftKey && key(e) === 'b', run: toggleUIStack.toggleSidebar, global: true },
    { test: (e) => ctrl(e) && e.shiftKey && key(e) === 'r', run: toggleUIStack.toggleRail, global: true },

    // Zoom de la interfaz
    {
        test: (e) => ctrl(e) && (e.key === '=' || e.key === '+'),
        run: () => settingsStack.zoom(0.05),
        global: true,
    },
    { test: (e) => ctrl(e) && e.key === '-', run: () => settingsStack.zoom(-0.05), global: true },
    {
        test: (e) => ctrl(e) && e.key === '0',
        run: () => void settingsStack.save({ uiScale: 1 }),
        global: true,
    },
];

function insideTerminal(event: KeyboardEvent): boolean {
    return event.target instanceof Element && event.target.closest('.xterm') !== null;
}

/** ¿La terminal debe ceder esta tecla a la aplicación? */
export function isAppShortcut(event: KeyboardEvent): boolean {
    return bindings.some((binding) => binding.global && binding.test(event));
}

export function handleHotkey(event: KeyboardEvent) {
    const inTerminal = insideTerminal(event);

    // Alt+1..9: saltar a la n-ésima sesión.
    if (event.altKey && !event.ctrlKey && /^[1-9]$/.test(event.key)) {
        event.preventDefault();
        sessionsStack.focusIndex(Number(event.key));
        return;
    }

    // Ctrl+1..6: cambiar de actividad.
    if (ctrl(event) && !event.shiftKey && /^[1-6]$/.test(event.key)) {
        event.preventDefault();
        workStack.setActivity(ACTIVITY_SHORTCUTS[Number(event.key) - 1]);
        return;
    }

    const match = bindings.find((binding) => binding.test(event) && (binding.global || !inTerminal));
    if (match) {
        event.preventDefault();
        match.run();
    }
}
