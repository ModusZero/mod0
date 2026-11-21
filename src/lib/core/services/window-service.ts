import { getCurrentWindow } from '@tauri-apps/api/window';
import { ask, open } from '@tauri-apps/plugin-dialog';
import { openPath, openUrl } from '@tauri-apps/plugin-opener';

export const windowService = {
    minimize: () => getCurrentWindow().minimize(),
    toggleMaximize: () => getCurrentWindow().toggleMaximize(),
    close: () => getCurrentWindow().close(),

    /** Abre el selector nativo de carpetas. */
    async pickFolder(title: string): Promise<string | null> {
        const selected = await open({ directory: true, multiple: false, title });
        return typeof selected === 'string' ? selected : null;
    },

    /** Confirmación nativa (sí/no). */
    confirm: (message: string, title: string) => ask(message, { title, kind: 'warning' }),

    openPath: (path: string) => openPath(path),
    openUrl: (url: string) => openUrl(url),
};
