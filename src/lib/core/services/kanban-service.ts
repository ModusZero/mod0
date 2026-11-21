import { call } from '#lib/core/services/ipc';
import type { KanbanBoard } from '#lib/core/types/kanban';

export const kanbanService = {
    load: (project: string) => call<KanbanBoard | null>('kanban_load', { project }),
    save: (project: string, board: KanbanBoard) => call<void>('kanban_save', { project, board }),
};
