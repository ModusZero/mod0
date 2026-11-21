export type KanbanColumnID = 'backlog' | 'doing' | 'review' | 'done';

export interface KanbanTask {
    id: string;
    title: string;
    notes: string;
    column: KanbanColumnID;
    /** Sesión que está trabajando en la tarea (si la hay). */
    sessionId: string | null;
    createdAt: number;
}

export interface KanbanBoard {
    tasks: KanbanTask[];
}
