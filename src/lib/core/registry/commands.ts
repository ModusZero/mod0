import { Bell, FolderOpen, Moon, PanelLeft, PanelRight, Plus, SquareTerminal } from '@lucide/svelte';
import { ActivityByWork, ActivityVisual, WorkSectionIDs } from '#lib/core/constants/work-config';
import { t } from '#lib/core/i18n/utils.svelte';
import { jumpToAttention, openFolder, openSession } from '#lib/core/registry/actions';
import { sessionsStack } from '#lib/core/runes/sessions.svelte';
import { settingsStack } from '#lib/core/runes/settings.svelte';
import { toggleUIStack } from '#lib/core/runes/toggle-ui.svelte';
import { workStack } from '#lib/core/runes/work-modes.svelte';
import type { PaletteItem } from '#lib/core/types/menu';

/** Comandos de la paleta (se reconstruyen al abrirla para reflejar el estado actual). */
export function buildCommands(): PaletteItem[] {
    const items: PaletteItem[] = [
        {
            id: 'new-session',
            label: t('session.new'),
            hint: 'Ctrl+Shift+N',
            icon: Plus,
            run: () => toggleUIStack.openNewSession(),
        },
        {
            id: 'attention',
            label: t('rail.nextAttention'),
            hint: 'Ctrl+Shift+A',
            icon: Bell,
            run: jumpToAttention,
        },
        { id: 'open-folder', label: t('project.open'), icon: FolderOpen, run: openFolder },
    ];

    for (const section of Object.values(WorkSectionIDs)) {
        for (const activity of ActivityByWork[section]) {
            const visual = ActivityVisual[activity];
            items.push({
                id: `go-${activity}`,
                label: `${t('palette.go')} ${t(visual.label)}`,
                hint: visual.shortcut,
                icon: visual.icon,
                run: () => workStack.setActivity(activity),
            });
        }
    }

    sessionsStack.sessions.forEach((session, index) => {
        items.push({
            id: `session-${session.id}`,
            label: `${t('palette.session')}: ${session.name}`,
            hint: index < 9 ? `Alt+${index + 1}` : undefined,
            keywords: `${session.profileName} ${session.branch ?? ''} ${session.status}`,
            run: () => openSession(session.id),
        });
    });

    items.push(
        {
            id: 'toggle-terminal',
            label: t('header.terminal'),
            hint: 'Ctrl+`',
            icon: SquareTerminal,
            run: toggleUIStack.toggleTerminal,
        },
        {
            id: 'toggle-sidebar',
            label: t('palette.toggleSidebar'),
            hint: 'Ctrl+Shift+B',
            icon: PanelLeft,
            run: toggleUIStack.toggleSidebar,
        },
        {
            id: 'toggle-rail',
            label: t('header.rail'),
            hint: 'Ctrl+Shift+R',
            icon: PanelRight,
            run: toggleUIStack.toggleRail,
        },
        {
            id: 'toggle-theme',
            label: t('palette.toggleTheme'),
            icon: Moon,
            run: () => void settingsStack.save({ theme: settingsStack.dark ? 'light' : 'dark' }),
        },
    );
    return items;
}
