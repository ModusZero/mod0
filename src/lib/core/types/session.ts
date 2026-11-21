/** Estado operativo de una sesión; determina cuánta atención merece. */
export type SessionStatus = 'starting' | 'working' | 'waiting' | 'review' | 'idle' | 'exited';

/** Familia de una acción de la IA (define icono y color en el grafo). */
export type ActionFamily =
    'read' | 'write' | 'exec' | 'search' | 'web' | 'agent' | 'plan' | 'think' | 'other';

export interface NowInfo {
    kind: ActionFamily;
    label: string;
    target: string | null;
}

/** Indicadores que realmente merecen atención. */
export interface SessionKpis {
    turns: number;
    toolCalls: number;
    errors: number;
    filesChanged: number;
    additions: number;
    deletions: number;
    contextTokens: number;
}

/** Punto del plan (checklist) que la IA mantiene con `TodoWrite`. */
export interface PlanItem {
    content: string;
    status: 'pending' | 'in_progress' | 'completed';
}

export interface Session {
    id: string;
    name: string;
    projectPath: string;
    cwd: string;
    branch: string | null;
    baseSha: string | null;
    worktree: boolean;
    profileId: string;
    profileName: string;
    status: SessionStatus;
    statusSince: number;
    createdAt: number;
    now: NowInfo | null;
    kpis: SessionKpis;
    plan: PlanItem[];
    model: string | null;
    lastMessage: string | null;
    claudeSessionId: string | null;
    transcriptPath: string | null;
    touched: string[];
    exitCode: number | null;
    reviewed: boolean;
    colorIdx: number;
    observed: boolean;
}

export interface CreateSessionRequest {
    name: string;
    profileId: string;
    projectPath: string;
    worktree: boolean;
    branch?: string;
    prompt?: string;
}

export type NodeKind = 'prompt' | 'tool' | 'notice' | 'stop';
export type NodeStatus = 'running' | 'ok' | 'error';

/** Un paso del razonamiento de la IA (nodo del thinking graph). */
export interface ThoughtNode {
    id: string;
    turn: number;
    seq: number;
    kind: NodeKind;
    family: ActionFamily;
    tool: string | null;
    label: string;
    detail: string | null;
    intent: string | null;
    target: string | null;
    status: NodeStatus;
    parent: string | null;
    ts: number;
    durationMs: number | null;
}

export interface GraphDelta {
    sessionId: string;
    nodes: ThoughtNode[];
}
