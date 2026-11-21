import { kanbanService } from '#lib/core/services/kanban-service';
import type { Session } from '#lib/core/types/session';
import type { KanbanColumnID, KanbanTask } from '#lib/core/types/kanban';

export const kanbanColumns: KanbanColumnID[] = ['backlog', 'doing', 'review', 'done'];

/** Tablero de tareas del proyecto abierto; las tareas ligadas a una sesión siguen su estado. */
class KanbanStack {
    tasks = $state<KanbanTask[]>([]);
    project = $state<string | null>(null);

    private saveTimer: ReturnType<typeof setTimeout> | undefined;

    async load(project: string | null) {
        if (project === this.project) return;
        this.project = project;
        this.tasks = project ? ((await kanbanService.load(project))?.tasks ?? []) : [];
    }

    inColumn(column: KanbanColumnID): KanbanTask[] {
        return this.tasks.filter((task) => task.column === column);
    }

    add(title: string, notes = '', column: KanbanColumnID = 'backlog'): KanbanTask {
        const task: KanbanTask = {
            id: crypto.randomUUID().slice(0, 8),
            title: title.trim(),
            notes: notes.trim(),
            column,
            sessionId: null,
            createdAt: Date.now(),
        };
        this.tasks.push(task);
        this.queueSave();
        return task;
    }

    update(id: string, patch: Partial<Omit<KanbanTask, 'id'>>) {
        const task = this.tasks.find((existing) => existing.id === id);
        if (!task) return;
        Object.assign(task, patch);
        this.queueSave();
    }

    move(id: string, column: KanbanColumnID) {
        this.update(id, { column });
    }

    remove(id: string) {
        this.tasks = this.tasks.filter((task) => task.id !== id);
        this.queueSave();
    }

    /** Mueve las tareas ligadas a sesiones según el estado actual de estas. */
    sync(sessions: Session[]) {
        let changed = false;

        for (const task of this.tasks) {
            if (!task.sessionId) continue;

            const session = sessions.find((existing) => existing.id === task.sessionId);
            if (!session) {
                task.sessionId = null;
                changed = true;
                continue;
            }

            const target: KanbanColumnID | null =
                session.status === 'working' || session.status === 'waiting' || session.status === 'starting'
                    ? 'doing'
                    : session.status === 'review'
                      ? 'review'
                      : null;

            if (target && task.column !== target && task.column !== 'done') {
                task.column = target;
                changed = true;
            }
        }
        if (changed) this.queueSave();
    }

    private queueSave() {
        clearTimeout(this.saveTimer);
        this.saveTimer = setTimeout(() => {
            if (this.project) void kanbanService.save(this.project, { tasks: $state.snapshot(this.tasks) });
        }, 400);
    }
}

export const kanbanStack = new KanbanStack();
