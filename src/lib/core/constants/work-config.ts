import {
    Network,
    LayoutGrid,
    SquareKanban,
    FolderTree,
    FileDiff,
    FileCode,
    Settings,
    Zap,
    GitFork,
} from '@lucide/svelte';
import type { ActivityID, DisplayNode, WorkSectionID } from '#lib/core/types/work';

/** Identificadores de secciones operativas del IDE */
export const WorkSectionIDs = {
    FORGE: 'forge',
    PULSE: 'pulse',
    WORKSPACE: 'workspace',
} as const;

/** Identificadores de actividades disponibles en el IDE */
export const ActivityIDs = {
    // Forge: orquestar sesiones
    SESSIONS: 'sessions',
    THINKING_GRAPH: 'thinking-graph',
    KANBAN: 'kanban',

    // Pulse: entender y revisar el proyecto
    FILES: 'files',
    REVIEW: 'review',
    EDITOR: 'editor',

    // Workspace: configuración
    SETTINGS: 'settings',
} as const;

export const WorkSectionVisual: Record<WorkSectionID, DisplayNode> = {
    [WorkSectionIDs.FORGE]: { id: WorkSectionIDs.FORGE, label: 'section.forge', icon: Zap },
    [WorkSectionIDs.PULSE]: { id: WorkSectionIDs.PULSE, label: 'section.pulse', icon: GitFork },
    [WorkSectionIDs.WORKSPACE]: { id: WorkSectionIDs.WORKSPACE, label: 'section.workspace', icon: Settings },
};

export const ActivityVisual: Record<ActivityID, DisplayNode> = {
    [ActivityIDs.SESSIONS]: {
        id: ActivityIDs.SESSIONS,
        label: 'activity.sessions',
        icon: LayoutGrid,
        shortcut: 'Ctrl+1',
    },
    [ActivityIDs.THINKING_GRAPH]: {
        id: ActivityIDs.THINKING_GRAPH,
        label: 'activity.thinking',
        icon: Network,
        shortcut: 'Ctrl+2',
    },
    [ActivityIDs.KANBAN]: {
        id: ActivityIDs.KANBAN,
        label: 'activity.kanban',
        icon: SquareKanban,
        shortcut: 'Ctrl+3',
    },
    [ActivityIDs.FILES]: {
        id: ActivityIDs.FILES,
        label: 'activity.files',
        icon: FolderTree,
        shortcut: 'Ctrl+4',
    },
    [ActivityIDs.REVIEW]: {
        id: ActivityIDs.REVIEW,
        label: 'activity.review',
        icon: FileDiff,
        shortcut: 'Ctrl+5',
    },
    [ActivityIDs.EDITOR]: {
        id: ActivityIDs.EDITOR,
        label: 'activity.editor',
        icon: FileCode,
        shortcut: 'Ctrl+6',
    },
    [ActivityIDs.SETTINGS]: {
        id: ActivityIDs.SETTINGS,
        label: 'activity.settings',
        icon: Settings,
        shortcut: 'Ctrl+,',
    },
};

export const ActivityByWork: Record<WorkSectionID, ActivityID[]> = {
    [WorkSectionIDs.FORGE]: [ActivityIDs.SESSIONS, ActivityIDs.THINKING_GRAPH, ActivityIDs.KANBAN],
    [WorkSectionIDs.PULSE]: [ActivityIDs.FILES, ActivityIDs.REVIEW, ActivityIDs.EDITOR],
    [WorkSectionIDs.WORKSPACE]: [ActivityIDs.SETTINGS],
};

/** Actividades que tienen panel lateral propio. */
export const ActivitiesWithSidebar: ActivityID[] = [
    ActivityIDs.SESSIONS,
    ActivityIDs.THINKING_GRAPH,
    ActivityIDs.FILES,
    ActivityIDs.REVIEW,
    ActivityIDs.SETTINGS,
];

/** Colores de estado de sesión (clases de Tailwind sobre los tokens de `app.css`). */
export const StatusStyle = {
    starting: { text: 'text-st-idle', bg: 'bg-st-idle', ring: 'ring-st-idle' },
    working: { text: 'text-st-working', bg: 'bg-st-working', ring: 'ring-st-working' },
    waiting: { text: 'text-st-waiting', bg: 'bg-st-waiting', ring: 'ring-st-waiting' },
    review: { text: 'text-st-review', bg: 'bg-st-review', ring: 'ring-st-review' },
    idle: { text: 'text-st-idle', bg: 'bg-st-idle', ring: 'ring-st-idle' },
    exited: { text: 'text-st-exited', bg: 'bg-st-exited', ring: 'ring-st-exited' },
} as const;

/** Prioridad de atención: menor número = más urgente. */
export const StatusPriority = {
    waiting: 0,
    review: 1,
    working: 2,
    starting: 3,
    idle: 4,
    exited: 5,
} as const;
