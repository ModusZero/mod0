# Proveedores y CLI

MOD0 **no implementa ningún proveedor de IA**: envuelve una CLI. Tu clave y tu modelo siguen siendo tuyos. Hay dos
caminos, según cuánta observación quieras.

> Los nombres de modelo y los endpoints de cada proveedor cambian con frecuencia. Lo de esta página es lo que indicaban
> sus documentaciones en noviembre de 2026: verifícalo antes de usarlo.

## Camino 1: Claude Code con otro proveedor (observación completa)

Claude Code puede apuntar a **cualquier endpoint compatible con la API de Anthropic** mediante variables de entorno.
Así mantienes todo lo que ofrece MOD0 (hooks, thinking graph, plan, estados exactos) con otros modelos.

| Plantilla de MOD0              | `ANTHROPIC_BASE_URL`                                 | Clave (variable del sistema) |
| ------------------------------ | ---------------------------------------------------- | ---------------------------- |
| Claude Code · OpenRouter       | `https://openrouter.ai/api`                          | `OPENROUTER_API_KEY`         |
| Claude Code · Z.ai (GLM)       | `https://api.z.ai/api/anthropic`                     | `ZAI_API_KEY`                |
| Claude Code · Qwen (DashScope) | `https://dashscope-intl.aliyuncs.com/apps/anthropic` | `DASHSCOPE_API_KEY`          |
| Claude Code · DeepSeek         | `https://api.deepseek.com/anthropic`                 | `DEEPSEEK_API_KEY`           |
| Claude Code · endpoint propio  | tu gateway (LiteLLM, proxy local…)                   | la que definas               |

Cada plantilla rellena `ANTHROPIC_BASE_URL` y `ANTHROPIC_AUTH_TOKEN=$VARIABLE`. Con OpenRouter además se deja
`ANTHROPIC_API_KEY` **vacía a propósito**. Para elegir el modelo, añade en el perfil las variables
`ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_OPUS_MODEL` y `ANTHROPIC_DEFAULT_HAIKU_MODEL` (o `ANTHROPIC_MODEL`)
con el identificador del modelo de ese proveedor.

**Límites**: OpenRouter solo garantiza el funcionamiento con modelos de Anthropic; con otros puede fallar, sobre todo si
el modelo es débil usando herramientas. Instalar Claude Code: `npm install -g @anthropic-ai/claude-code`.

## Camino 2: otras CLI (estado por actividad)

| CLI        | Comando    | Instalación (referencia)                               | Configuración                                                                 |
| ---------- | ---------- | ------------------------------------------------------ | ----------------------------------------------------------------------------- |
| Aider      | `aider`    | `python -m pip install aider-install && aider-install` | `OPENROUTER_API_KEY` y `--model openrouter/<modelo>`                          |
| OpenCode   | `opencode` | `npm install -g opencode-ai`                           | dentro de la CLI (`opencode auth login`)                                      |
| Qwen Code  | `qwen`     | `npm install -g @qwen-code/qwen-code`                  | variables estilo OpenAI (`OPENAI_API_KEY`, `OPENAI_BASE_URL`, `OPENAI_MODEL`) |
| Codex CLI  | `codex`    | `npm install -g @openai/codex`                         | dentro de la CLI                                                              |
| Gemini CLI | `gemini`   | `npm install -g @google/gemini-cli`                    | dentro de la CLI                                                              |

Con estas CLI obtienes la terminal, el estado inferido por actividad (_trabajando_ mientras hay salida reciente, _en
reposo_ tras 2,5 s de silencio), la revisión, los worktrees, el kanban, el mapa del proyecto y los avisos. **No** hay
thinking graph, plan ni estado _esperando_ exacto: dependen de los hooks de Claude Code.

## Claves: `$NOMBRE`

En el perfil escribe `CLAVE=$VARIABLE`: MOD0 lee `VARIABLE` de las variables de entorno **de tu sistema** al lanzar la
sesión, así la clave no se guarda en ningún archivo. En Windows:

```powershell
setx OPENROUTER_API_KEY "sk-or-..."   # reinicia MOD0 después
```

En _Ajustes → Perfiles de CLI_ cada perfil muestra ✓ si el comando está instalado y avisa de las variables sin definir.
Si prefieres, configura la CLI como siempre desde la terminal de la sesión: MOD0 no se interpone.

## Probar modelos gratuitos (p. ej. los `:free` de OpenRouter o NVIDIA)

Sirven para tareas acotadas, pero **no hay garantía** de que un modelo pequeño o gratuito sea fiable en un bucle
agéntico (llamadas a herramientas encadenadas, ediciones largas). Cómo probarlos con criterio:

1. Crea un perfil con el modelo (plantilla de Claude Code · OpenRouter, o Aider con `--model openrouter/<id>`).
2. Lanza **dos sesiones con la misma tarea**, cada una en su worktree y con un modelo distinto, y compara los diffs en
   _Revisión_. Es exactamente el caso para el que sirven los worktrees.
3. Fíjate en errores de herramientas, reintentos en el diagrama y contexto consumido.
4. **Privacidad**: los modelos gratuitos suelen tener límites de uso y políticas de datos distintas (algunos usan los
   prompts para entrenar). No envíes código privado de terceros sin leer la política del proveedor.
