import { Channel } from '@tauri-apps/api/core';
import { call } from '#lib/core/services/ipc';

export const terminalService = {
    /**
     * Conecta la terminal de una sesión. Devuelve el historial reciente; lo posterior llega
     * por `onData` (bytes crudos).
     */
    async attach(id: string, onData: (bytes: Uint8Array) => void): Promise<Uint8Array> {
        const channel = new Channel<ArrayBuffer>();
        const queue: Uint8Array[] = [];
        let ready = false;

        channel.onmessage = (message) => {
            const bytes = new Uint8Array(message);
            if (ready) onData(bytes);
            else queue.push(bytes);
        };

        const history = new Uint8Array(await call<ArrayBuffer>('term_attach', { id, channel }));
        // El historial siempre se pinta antes que los mensajes que llegaron mientras tanto.
        queueMicrotask(() => {
            ready = true;
            queue.splice(0).forEach(onData);
        });
        return history;
    },

    detach: (id: string) => call<void>('term_detach', { id }),
    write: (id: string, data: string) => call<void>('term_write', { id, data }),
    resize: (id: string, rows: number, cols: number) => call<void>('term_resize', { id, rows, cols }),
};
