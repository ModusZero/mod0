import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { GraphDelta, Session } from '#lib/core/types/session';

/** Eventos que el backend emite hacia el frontend (ver `core/events.rs`). */
export const eventService = {
    onSession: (handler: (session: Session) => void): Promise<UnlistenFn> =>
        listen<Session>('session:upsert', (event) => handler(event.payload)),

    onSessionRemoved: (handler: (id: string) => void): Promise<UnlistenFn> =>
        listen<string>('session:removed', (event) => handler(event.payload)),

    onGraphDelta: (handler: (delta: GraphDelta) => void): Promise<UnlistenFn> =>
        listen<GraphDelta>('graph:delta', (event) => handler(event.payload)),

    onFsChanged: (handler: (cwd: string) => void): Promise<UnlistenFn> =>
        listen<{ cwd: string }>('fs:changed', (event) => handler(event.payload.cwd)),
};
