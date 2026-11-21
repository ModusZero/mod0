import { StatusPriority } from '#lib/core/constants/work-config';
import { eventService } from '#lib/core/services/event-service';
import { sessionService } from '#lib/core/services/session-service';
import type { CreateSessionRequest, Session, SessionStatus } from '#lib/core/types/session';

/** Todas las sesiones de CLI en curso y cuál tiene el foco. */
class SessionsStack {
    sessions = $state<Session[]>([]);
    focusedId = $state<string | null>(null);
    /** Filtro de la vista de sesiones. */
    filter = $state<'all' | 'attention' | 'working' | 'idle'>('all');
    /** Reloj que avanza cada pocos segundos para refrescar duraciones. */
    clock = $state(Date.now());

    focused = $derived(this.sessions.find((session) => session.id === this.focusedId) ?? null);

    counts = $derived.by(() => {
        const counts: Record<SessionStatus, number> = {
            starting: 0,
            working: 0,
            waiting: 0,
            review: 0,
            idle: 0,
            exited: 0,
        };
        for (const session of this.sessions) counts[session.status]++;
        return counts;
    });

    /** Sesiones que piden atención ahora (bloqueadas o por revisar), las más antiguas primero. */
    attention = $derived(
        this.sessions
            .filter((session) => session.status === 'waiting' || session.status === 'review')
            .sort(
                (a, b) =>
                    StatusPriority[a.status] - StatusPriority[b.status] || a.statusSince - b.statusSince,
            ),
    );

    async init(): Promise<() => void> {
        this.sessions = await sessionService.list();
        this.focusedId = this.sessions[0]?.id ?? null;

        const unlisten = await Promise.all([
            eventService.onSession((session) => this.upsert(session)),
            eventService.onSessionRemoved((id) => this.drop(id)),
        ]);
        const timer = setInterval(() => (this.clock = Date.now()), 5000);

        return () => {
            unlisten.forEach((stop) => stop());
            clearInterval(timer);
        };
    }

    upsert(session: Session) {
        const index = this.sessions.findIndex((existing) => existing.id === session.id);
        if (index === -1) this.sessions.push(session);
        else this.sessions[index] = session;
        this.focusedId ??= session.id;
    }

    drop(id: string) {
        this.sessions = this.sessions.filter((session) => session.id !== id);
        if (this.focusedId === id) this.focusedId = this.sessions[0]?.id ?? null;
    }

    focus(id: string) {
        if (this.sessions.some((session) => session.id === id)) this.focusedId = id;
    }

    /** Enfoca la n-ésima sesión (1-based) en orden de creación. */
    focusIndex(position: number) {
        const session = this.sessions[position - 1];
        if (session) this.focusedId = session.id;
    }

    /** Salta a la siguiente sesión que necesita atención. */
    focusNextAttention(): Session | null {
        if (this.attention.length === 0) return null;
        const current = this.attention.findIndex((session) => session.id === this.focusedId);
        const next = this.attention[(current + 1) % this.attention.length];
        this.focusedId = next.id;
        return next;
    }

    async create(request: CreateSessionRequest): Promise<Session> {
        const session = await sessionService.create(request);
        this.upsert(session);
        this.focusedId = session.id;
        return session;
    }

    byId(id: string | null): Session | undefined {
        return id ? this.sessions.find((session) => session.id === id) : undefined;
    }
}

export const sessionsStack = new SessionsStack();
