import { loadLocal, saveLocal } from '#lib/core/utils/storage';
import { sessionService } from '#lib/core/services/session-service';
import { eventService } from '#lib/core/services/event-service';
import type { GraphDelta, ThoughtNode } from '#lib/core/types/session';

/** Cuántos turnos recientes se muestran expandidos por defecto. */
const EXPANDED_TURNS = 2;

/** Thinking graph de cada sesión: se hidrata una vez y luego solo recibe deltas. */
class GraphStack {
    bySession = $state<Record<string, ThoughtNode[]>>({});
    selectedId = $state<string | null>(null);
    /** `diagram`: fases encadenadas con flechas. `timeline`: cronología paso a paso. */
    view = $state<'diagram' | 'timeline'>(loadLocal('graph.view', 'diagram'));
    follow = $state(true);
    /** Turno al que debe desplazarse el lienzo (lo pide la barra lateral). */
    jumpTurn = $state<number | null>(null);
    /** Turnos que el usuario expandió/colapsó a mano: `sessionId:turno` → expandido. */
    toggled = $state<Record<string, boolean>>({});

    async init(): Promise<() => void> {
        return eventService.onGraphDelta((delta) => this.apply(delta));
    }

    async load(sessionId: string) {
        if (this.bySession[sessionId]) return;
        this.bySession[sessionId] = await sessionService.graph(sessionId);
    }

    apply({ sessionId, nodes }: GraphDelta) {
        const current = [...(this.bySession[sessionId] ?? [])];

        for (const node of nodes) {
            const index = current.findLastIndex((existing) => existing.id === node.id);
            if (index === -1) current.push(node);
            else current[index] = node;
        }
        current.sort((a, b) => a.seq - b.seq);
        this.bySession[sessionId] = current;
    }

    setView(view: 'diagram' | 'timeline') {
        this.view = view;
        saveLocal('graph.view', view);
    }

    nodes(sessionId: string | null): ThoughtNode[] {
        return sessionId ? (this.bySession[sessionId] ?? []) : [];
    }

    /** Últimos pasos de una sesión (para el rastro de las tarjetas). */
    trail(sessionId: string, count = 14): ThoughtNode[] {
        return this.nodes(sessionId)
            .filter((node) => node.kind === 'tool')
            .slice(-count);
    }

    isExpanded(sessionId: string, turn: number, lastTurn: number): boolean {
        return this.toggled[`${sessionId}:${turn}`] ?? turn > lastTurn - EXPANDED_TURNS;
    }

    toggleTurn(sessionId: string, turn: number, lastTurn: number) {
        this.toggled[`${sessionId}:${turn}`] = !this.isExpanded(sessionId, turn, lastTurn);
    }

    selected(sessionId: string | null): ThoughtNode | null {
        return this.nodes(sessionId).find((node) => node.id === this.selectedId) ?? null;
    }
}

export const graphStack = new GraphStack();
