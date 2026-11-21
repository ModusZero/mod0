export type ThemeMode = 'dark' | 'light' | 'system';
export type AccentID = 'blue' | 'teal' | 'violet' | 'amber' | 'rose';
export type Language = 'es' | 'en';

export interface CliProfile {
    id: string;
    name: string;
    command: string;
    args: string[];
    env: Record<string, string>;
    hooks: boolean;
    resumeArgs: string[];
}

export interface WebhookConfig {
    enabled: boolean;
    url: string;
    method: 'GET' | 'POST';
    body: string;
    contentType: string;
}

export interface NotificationConfig {
    desktop: boolean;
    onlyWhenUnfocused: boolean;
    onNeedsYou: boolean;
    onSessionDone: boolean;
    onAllIdle: boolean;
    webhook: WebhookConfig;
}

export interface AppConfig {
    theme: ThemeMode;
    accent: AccentID;
    language: Language;
    uiScale: number;
    reduceMotion: boolean;
    autoSave: boolean;
    terminalFontSize: number;
    contextLimit: number;
    shell: string | null;
    lastProjectPath: string | null;
    recentProjects: string[];
    profiles: CliProfile[];
    defaultProfileId: string;
    notifications: NotificationConfig;
}

export interface AppInfo {
    version: string;
    hookPort: number | null;
    dataDir: string;
    configDir: string;
    shell: string;
    git: string | null;
}

export interface CommandCheck {
    found: boolean;
    path: string | null;
    /** Variables `$NOMBRE` del perfil que no existen en el entorno del sistema. */
    missingEnv: string[];
}
