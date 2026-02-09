import { 
    Box, 
    Cpu, 
    Zap, 
    Puzzle, 
    Folder, 
    Settings,
    GitGraph, 
    GitBranch, 
    ClipboardList, 
    MessageSquare, 
    TestTube
} from 'lucide-svelte';
import type { ActivityID, DisplayNode, WorkSectionID } from "$lib/core/types/work";

/** Identificadores de secciones operativas del IDE */
export const WorkSectionIDs = {
    BLUEPRINT: 'blueprint',
    FORGE: 'forge',
    PULSE: 'pulse',
    WORKSPACE: 'workspace',
} as const;

/** Identificadores de actividades disponibles en el IDE */
export const ActivityIDs = {
    // Blueprint
    CHAT: 'chat',
    ARTIFACTS: 'artifacts',

    // Forge
    THINKING_GRAPH: 'thinking-graph',
    KANBAN: 'kanban',

    // Pulse
    FILES: 'files',
    TESTING: 'testing',
    SOURCE_CONTROL: 'source-control',

    // Workspace
    CONTEXT: 'context',
    WORKFLOW: 'workflow',
    EXTENSIONS: 'extensions',
};

export const WorkSectionVisual: Record<WorkSectionID, DisplayNode> = {
    [WorkSectionIDs.BLUEPRINT]: { 
        id: WorkSectionIDs.BLUEPRINT,
        label: 'Blueprint', 
        icon: Cpu, 
        color: 'text-blue-400',
        description: 'Design and plan your projects with the Blueprint section, where you can create and manage your project blueprints, visualize dependencies, and organize your work in a structured way.'
    },
    [WorkSectionIDs.FORGE]: { 
        id: WorkSectionIDs.FORGE,
        label: 'Forge', 
        icon: Zap, 
        color: 'text-orange-400',
        description: 'The Forge is where the magic happens. Here you can build, test, and iterate on your projects with powerful tools and integrations that help you bring your ideas to life.'
    },
    [WorkSectionIDs.PULSE]: { 
        id: WorkSectionIDs.PULSE, 
        label: 'Pulse', 
        icon: GitGraph, 
        color: 'text-purple-400', 
        description: 'Monitor and track the progress of your projects in real-time with the Pulse section. Visualize project health, performance metrics, and key performance indicators (KPIs) to stay informed and make data-driven decisions.' 
    },
    [WorkSectionIDs.WORKSPACE]: { 
        id: WorkSectionIDs.WORKSPACE, 
        label: 'Workspace', 
        icon: Settings, 
        color: 'text-gray-400', 
        description: 'Manage your agile & personalized configurations in the IDE Workspace section. Customize your workspace, adjust preferences, and fine-tune your development environment to suit your needs.' 
    }
} as const;

export const ActivityVisual: Record<ActivityID, DisplayNode> = {
    // Blueprint
    [ActivityIDs.CHAT]: {
        id: ActivityIDs.CHAT, 
        label: 'Chat',
        icon: MessageSquare, 
        color: 'text-blue-400',
        description: 'Engage in real-time conversations with your team and AI assistants in the Chat activity. Collaborate, brainstorm, and get instant feedback to keep your projects moving forward.'
    },
    [ActivityIDs.ARTIFACTS]: { 
        id: ActivityIDs.ARTIFACTS, 
        label: 'Artifacts', 
        icon: Box, 
        color: 'text-green-400',
        description: 'Manage and view your project artifacts in the Artifacts activity. Access and organize all the deliverables and components of your projects in one place.'
    },

    // Forge
    [ActivityIDs.THINKING_GRAPH]: { 
        id: ActivityIDs.THINKING_GRAPH, 
        label: 'Thinking Graph', 
        icon: GitBranch, 
        color: 'text-purple-400',
        description: 'Visualize and analyze your project\'s thinking process in the Thinking Graph activity. Understand how ideas evolve and connect to build better projects.' 
    },
    [ActivityIDs.KANBAN]: { 
        id: ActivityIDs.KANBAN, 
        label: 'Kanban Board', 
        icon: ClipboardList, 
        color: 'text-yellow-400', 
        description: 'Manage your workflow and tasks in the Kanban Board activity. Visualize your work in progress, identify bottlenecks, and optimize your team\'s productivity.' 
    },
    [ActivityIDs.TESTING]: { 
        id: ActivityIDs.TESTING, 
        label: 'Testing', 
        icon: TestTube, 
        color: 'text-red-400',
        description: 'Manage and run your project tests in the Testing activity. Execute automated tests, view results, and ensure code quality and reliability.' 
    },
    
    // Pulse
    [ActivityIDs.FILES]: { 
        id: ActivityIDs.FILES, 
        label: 'Files', 
        icon: Folder, 
        color: 'text-gray-400',
        description: 'Browse and manage your project files in the Files activity. View file hierarchies, access metadata, and organize your project assets efficiently.' 
    },
    [ActivityIDs.SOURCE_CONTROL]: { 
        id: ActivityIDs.SOURCE_CONTROL, 
        label: 'Source Control', 
        icon: GitBranch, 
        color: 'text-red-400',
        description: 'Track changes and manage version control in the Source Control activity. Collaborate with team members, review history, and maintain a clean codebase.' 
    },

    // Workspace
    [ActivityIDs.CONTEXT]: { 
        id: ActivityIDs.CONTEXT, 
        label: 'Context', 
        icon: MessageSquare, 
        color: 'text-gray-400',
        description: 'Manage your context and preferences in the Context activity. Customize your workspace and tailor your development environment to suit your needs.' 
    },
    [ActivityIDs.WORKFLOW]: { 
        id: ActivityIDs.WORKFLOW, 
        label: 'Workflow', 
        icon: GitGraph, 
        color: 'text-blue-400',
        description: 'Define and manage your project workflows in the Workflow activity. Create custom workflows that match your team\'s processes and requirements.' 
    },
    [ActivityIDs.EXTENSIONS]: { 
        id: ActivityIDs.EXTENSIONS, 
        label: 'Extensions', 
        icon: Puzzle, 
        color: 'text-green-400',
        description: 'Manage and configure extensions in the Extensions activity. Install, update, and customize extensions to enhance your IDE capabilities.' 
    },
};

export const ActivityByWork: Record<WorkSectionID, ActivityID[]> = {
    [WorkSectionIDs.BLUEPRINT]: [
        ActivityIDs.CHAT,
        ActivityIDs.ARTIFACTS,
    ],
    [WorkSectionIDs.FORGE]: [
        ActivityIDs.THINKING_GRAPH,
        ActivityIDs.TESTING,
        ActivityIDs.KANBAN
    ],
    [WorkSectionIDs.PULSE]: [
        ActivityIDs.FILES,
        ActivityIDs.SOURCE_CONTROL,
    ],
    [WorkSectionIDs.WORKSPACE]: [
        ActivityIDs.CONTEXT,
        ActivityIDs.WORKFLOW,
        ActivityIDs.EXTENSIONS
    ]
};