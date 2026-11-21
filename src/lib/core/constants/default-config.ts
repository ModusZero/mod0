import type { AppConfig } from '#lib/core/types/settings';

/** Valores iniciales mientras el backend entrega la configuración real. */
export const defaultConfig: AppConfig = {
    theme: 'dark',
    accent: 'blue',
    language: 'es',
    uiScale: 1,
    reduceMotion: false,
    autoSave: false,
    terminalFontSize: 13,
    contextLimit: 200_000,
    shell: null,
    lastProjectPath: null,
    recentProjects: [],
    profiles: [],
    defaultProfileId: 'claude',
    notifications: {
        desktop: true,
        onlyWhenUnfocused: true,
        onNeedsYou: true,
        onSessionDone: false,
        onAllIdle: true,
        webhook: {
            enabled: false,
            url: '',
            method: 'POST',
            body: '{message}',
            contentType: 'text/plain; charset=utf-8',
        },
    },
};
