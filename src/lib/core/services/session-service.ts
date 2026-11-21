import { call } from '#lib/core/services/ipc';
import type { CreateSessionRequest, Session, ThoughtNode } from '#lib/core/types/session';

export const sessionService = {
    list: () => call<Session[]>('session_list'),
    create: (request: CreateSessionRequest) => call<Session>('session_create', { request }),
    restart: (id: string) => call<Session>('session_restart', { id }),
    stop: (id: string) => call<void>('session_stop', { id }),
    remove: (id: string, removeWorktree: boolean) => call<void>('session_remove', { id, removeWorktree }),
    rename: (id: string, name: string) => call<Session>('session_rename', { id, name }),
    setReviewed: (id: string, reviewed: boolean) => call<Session>('session_set_reviewed', { id, reviewed }),
    graph: (id: string) => call<ThoughtNode[]>('session_graph', { id }),
    refresh: (id: string) => call<void>('session_refresh', { id }),
};
