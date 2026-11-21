import type { ThoughtNode } from '#lib/core/types/session';

export const ROW_HEIGHT = 34;
export const LANE_WIDTH = 18;

export interface GraphRow {
    type: 'header' | 'node';
    y: number;
    turn: number;
    node?: ThoughtNode;
    lane: number;
    /** Solo en cabeceras: resumen del turno. */
    summary?: { steps: number; files: number; errors: number; running: boolean };
    prompt?: string;
    expanded?: boolean;
}

export interface GraphLink {
    from: { lane: number; y: number };
    to: { lane: number; y: number };
}

export interface GraphLayout {
    rows: GraphRow[];
    links: GraphLink[];
    lanes: number;
    height: number;
}

/**
 * Distribuye los nodos como un historial de git: el tiempo baja, las ramas paralelas
 * (llamadas simultáneas) se abren hacia la derecha. Los turnos antiguos se colapsan.
 */
export function layoutGraph(nodes: ThoughtNode[], isExpanded: (turn: number) => boolean): GraphLayout {
    const byTurn = new Map<number, ThoughtNode[]>();
    for (const node of nodes) {
        const list = byTurn.get(node.turn) ?? [];
        list.push(node);
        byTurn.set(node.turn, list);
    }

    const rows: GraphRow[] = [];
    const links: GraphLink[] = [];
    let y = 0;
    let lanes = 1;

    for (const [turn, turnNodes] of [...byTurn.entries()].sort((a, b) => a[0] - b[0])) {
        const tools = turnNodes.filter((node) => node.kind === 'tool');
        const expanded = isExpanded(turn);

        rows.push({
            type: 'header',
            y,
            turn,
            lane: 0,
            expanded,
            prompt: turnNodes.find((node) => node.kind === 'prompt')?.detail ?? turnNodes[0]?.label ?? '',
            summary: {
                steps: tools.length,
                files: new Set(
                    tools.filter((node) => node.family === 'write' && node.target).map((node) => node.target),
                ).size,
                errors: tools.filter((node) => node.status === 'error').length,
                running: tools.some((node) => node.status === 'running'),
            },
        });
        y += ROW_HEIGHT;
        if (!expanded) continue;

        const laneOf = new Map<string, number>();
        const children = new Map<string, number>();
        const position = new Map<string, { lane: number; y: number }>();

        for (const node of turnNodes) {
            let lane = 0;
            if (node.parent && laneOf.has(node.parent)) {
                const siblings = children.get(node.parent) ?? 0;
                children.set(node.parent, siblings + 1);
                lane = (laneOf.get(node.parent) ?? 0) + siblings;
            }
            laneOf.set(node.id, lane);
            lanes = Math.max(lanes, lane + 1);

            rows.push({ type: 'node', y, turn, node, lane });
            position.set(node.id, { lane, y });

            const parent = node.parent ? position.get(node.parent) : undefined;
            if (parent) links.push({ from: parent, to: { lane, y } });
            y += ROW_HEIGHT;
        }
    }

    return { rows, links, lanes, height: y };
}

export interface Phase {
    id: string;
    family: ThoughtNode['family'];
    nodes: ThoughtNode[];
    /** Alguna herramienta de la fase falló. */
    failed: boolean;
    running: boolean;
    /** Fase de edición que viene justo después de un fallo: se dibuja como reintento. */
    retry: boolean;
}

export interface TurnDiagram {
    turn: number;
    prompt: string;
    phases: Phase[];
    stop: ThoughtNode | null;
    notices: ThoughtNode[];
}

/**
 * Resume cada turno en fases: pasos consecutivos de la misma familia (leer, editar, ejecutar…) se agrupan en
 * una sola caja. Es la vista "de lejos": explorar → editar → verificar → reintentar → listo.
 */
export function buildDiagram(nodes: ThoughtNode[]): TurnDiagram[] {
    const turns = new Map<number, TurnDiagram>();

    for (const node of nodes) {
        const turn = turns.get(node.turn) ?? {
            turn: node.turn,
            prompt: '',
            phases: [],
            stop: null,
            notices: [],
        };
        turns.set(node.turn, turn);

        if (node.kind === 'prompt') turn.prompt = node.detail ?? node.label;
        else if (node.kind === 'stop') turn.stop = node;
        else if (node.kind === 'notice') turn.notices.push(node);
        else {
            const last = turn.phases.at(-1);
            if (last && last.family === node.family && node.family !== 'agent') last.nodes.push(node);
            else
                turn.phases.push({
                    id: `${node.turn}:${node.id}`,
                    family: node.family,
                    nodes: [node],
                    failed: false,
                    running: false,
                    retry: false,
                });
        }
    }

    for (const turn of turns.values()) {
        turn.phases.forEach((phase, index) => {
            phase.failed = phase.nodes.some((node) => node.status === 'error');
            phase.running = phase.nodes.some((node) => node.status === 'running');
            phase.retry = phase.family === 'write' && turn.phases[index - 1]?.failed === true;
        });
    }
    return [...turns.values()].sort((a, b) => a.turn - b.turn);
}
