import type { CliProfile } from '#lib/core/types/settings';

export type PresetGroup = 'claude' | 'other';

export interface ProfilePreset {
    id: string;
    group: PresetGroup;
    profile: Omit<CliProfile, 'id'>;
    /** Cómo instalar la CLI (referencia; verifica siempre con la documentación oficial). */
    install?: string;
    /** Nota breve (es/en) que se muestra al elegir la plantilla. */
    note: { es: string; en: string };
}

const claude = (name: string, env: Record<string, string>): Omit<CliProfile, 'id'> => ({
    name,
    command: 'claude',
    args: [],
    env,
    hooks: true,
    resumeArgs: ['--resume', '{session}'],
});

const other = (name: string, command: string, env: Record<string, string> = {}, args: string[] = []) => ({
    name,
    command,
    args,
    env,
    hooks: false,
    resumeArgs: [] as string[],
});

/**
 * Plantillas de perfiles. Las claves nunca se escriben aquí: se referencian con `$NOMBRE`, que MOD0 lee de las
 * variables de entorno del sistema al lanzar la sesión (defínelas una vez, p. ej. con `setx` en Windows).
 */
export const profilePresets: ProfilePreset[] = [
    {
        id: 'claude',
        group: 'claude',
        profile: claude('Claude Code', {}),
        install: 'npm install -g @anthropic-ai/claude-code',
        note: {
            es: 'La CLI oficial. Observación completa: hooks, thinking graph, plan y estados exactos.',
            en: 'The official CLI. Full observation: hooks, thinking graph, plan and exact states.',
        },
    },
    {
        id: 'claude-openrouter',
        group: 'claude',
        profile: claude('Claude Code · OpenRouter', {
            ANTHROPIC_BASE_URL: 'https://openrouter.ai/api',
            ANTHROPIC_AUTH_TOKEN: '$OPENROUTER_API_KEY',
            ANTHROPIC_API_KEY: '',
        }),
        install: 'npm install -g @anthropic-ai/claude-code',
        note: {
            es: 'Usa Claude Code con modelos de OpenRouter. Define OPENROUTER_API_KEY en el sistema. OpenRouter garantiza el funcionamiento con modelos de Anthropic; con otros puede fallar.',
            en: 'Claude Code through OpenRouter. Set OPENROUTER_API_KEY in your system. OpenRouter guarantees it with Anthropic models; others may not work.',
        },
    },
    {
        id: 'claude-zai',
        group: 'claude',
        profile: claude('Claude Code · Z.ai (GLM)', {
            ANTHROPIC_BASE_URL: 'https://api.z.ai/api/anthropic',
            ANTHROPIC_AUTH_TOKEN: '$ZAI_API_KEY',
        }),
        install: 'npm install -g @anthropic-ai/claude-code',
        note: {
            es: 'Endpoint compatible de Z.ai para modelos GLM. Define ZAI_API_KEY. El mapeo de modelos se ajusta con ANTHROPIC_DEFAULT_*_MODEL.',
            en: "Z.ai's compatible endpoint for GLM models. Set ZAI_API_KEY. Map models with ANTHROPIC_DEFAULT_*_MODEL.",
        },
    },
    {
        id: 'claude-qwen',
        group: 'claude',
        profile: claude('Claude Code · Qwen (DashScope)', {
            ANTHROPIC_BASE_URL: 'https://dashscope-intl.aliyuncs.com/apps/anthropic',
            ANTHROPIC_AUTH_TOKEN: '$DASHSCOPE_API_KEY',
        }),
        install: 'npm install -g @anthropic-ai/claude-code',
        note: {
            es: 'Endpoint compatible de Alibaba (DashScope) para Qwen. Define DASHSCOPE_API_KEY.',
            en: "Alibaba's (DashScope) compatible endpoint for Qwen. Set DASHSCOPE_API_KEY.",
        },
    },
    {
        id: 'claude-deepseek',
        group: 'claude',
        profile: claude('Claude Code · DeepSeek', {
            ANTHROPIC_BASE_URL: 'https://api.deepseek.com/anthropic',
            ANTHROPIC_AUTH_TOKEN: '$DEEPSEEK_API_KEY',
        }),
        install: 'npm install -g @anthropic-ai/claude-code',
        note: {
            es: 'Endpoint compatible de DeepSeek. Define DEEPSEEK_API_KEY.',
            en: "DeepSeek's compatible endpoint. Set DEEPSEEK_API_KEY.",
        },
    },
    {
        id: 'claude-gateway',
        group: 'claude',
        profile: claude('Claude Code · endpoint propio / local', {
            ANTHROPIC_BASE_URL: 'http://localhost:4000',
            ANTHROPIC_AUTH_TOKEN: '$MY_GATEWAY_KEY',
        }),
        install: 'npm install -g @anthropic-ai/claude-code',
        note: {
            es: 'Para un gateway propio (LiteLLM, un proxy local, Ollama detrás de un traductor…). Ajusta la URL y la variable de la clave.',
            en: 'For your own gateway (LiteLLM, a local proxy, Ollama behind a translator…). Adjust the URL and key variable.',
        },
    },
    {
        id: 'aider',
        group: 'other',
        profile: other('Aider', 'aider', { OPENROUTER_API_KEY: '$OPENROUTER_API_KEY' }, [
            '--model',
            'openrouter/<modelo>',
        ]),
        install: 'python -m pip install aider-install && aider-install',
        note: {
            es: 'Sin hooks: el estado se infiere por actividad. Cambia <modelo> por uno de OpenRouter (admite modelos :free).',
            en: 'No hooks: state is inferred from activity. Replace <modelo> with an OpenRouter model (:free models work).',
        },
    },
    {
        id: 'opencode',
        group: 'other',
        profile: other('OpenCode', 'opencode'),
        install: 'npm install -g opencode-ai',
        note: {
            es: 'Sin hooks. Configura proveedores dentro de la propia CLI (opencode auth login).',
            en: 'No hooks. Configure providers inside the CLI itself (opencode auth login).',
        },
    },
    {
        id: 'qwen-code',
        group: 'other',
        profile: other('Qwen Code', 'qwen', {
            OPENAI_API_KEY: '$DASHSCOPE_API_KEY',
            OPENAI_BASE_URL: 'https://dashscope-intl.aliyuncs.com/compatible-mode/v1',
            OPENAI_MODEL: 'qwen3-coder-plus',
        }),
        install: 'npm install -g @qwen-code/qwen-code',
        note: {
            es: 'Sin hooks. Variables estilo OpenAI; ajusta el modelo según tu cuenta.',
            en: 'No hooks. OpenAI-style variables; adjust the model for your account.',
        },
    },
    {
        id: 'codex',
        group: 'other',
        profile: other('Codex CLI', 'codex'),
        install: 'npm install -g @openai/codex',
        note: { es: 'Sin hooks.', en: 'No hooks.' },
    },
    {
        id: 'gemini',
        group: 'other',
        profile: other('Gemini CLI', 'gemini'),
        install: 'npm install -g @google/gemini-cli',
        note: { es: 'Sin hooks.', en: 'No hooks.' },
    },
    {
        id: 'codespace',
        group: 'other',
        profile: other('Codespace remoto (gh)', 'gh', {}, ['codespace', 'ssh']),
        install: 'gh auth login',
        note: {
            es: 'Terminal remota en un Codespace: la sesión sobrevive a cortes si usas tmux dentro. La revisión y el grafo de archivos solo ven carpetas locales.',
            en: 'Remote terminal in a Codespace: the session survives drops if you use tmux inside. Review and file graph only see local folders.',
        },
    },
];
