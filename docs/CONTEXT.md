# MOD0 · Contexto global del proyecto

> Documento de contexto para Claude (fuente del proyecto) y para cualquier persona que llegue al repo.
> **Se actualiza en cada release** junto con `docs/PLAN.md`. Última actualización: **0.3.0 · 2026-11-16**.

## 1. Qué es MOD0

Una app de escritorio (Tauri 2 + SvelteKit 3/Svelte 5) que funciona como **mission control para sesiones de CLI con IA**
(principalmente Claude Code). No es un IDE clásico: es "una UI con tres tallitas y la CLI".

**Dueño y único usuario objetivo**: Luis (luishrosello), tech lead; trabaja en Windows. Español como idioma de trabajo.

### Por qué existe (el flujo real)

- Trabaja con **8–15 sesiones de Claude abiertas a la vez** (llegó a 15 durante 5+ horas). Su límite no es escribir
  código sino _seguirle el hilo_ a varios flujos en paralelo.
- El "human in the loop" ya no es tirar prompts: es **revisar, orquestar y decidir**. Las CLI muestran texto que
  nadie lee de verdad; los IDE clásicos traen funciones que no se tocan.
- Con Claude trabajando 24/7 hace falta: ver en todo momento **en qué archivo trabaja, qué piensa y quién te necesita**,
  saltar entre sesiones, revisar cómodo y recibir un **aviso al móvil** cuando algo requiere atención o cuando todo terminó.

### Decisiones de producto (no revertir sin hablar)

1. **Ya no es comercial.** Nació como IDE genérico/personalizable/comercial; hoy es herramienta personal. Cualquier
   afirmación comercial (SOC 2, "swarms", cifrado bancario…) es falsa y no debe reaparecer.
2. **Es un envoltorio de la CLI**, no un reemplazo. BYOK e IA local se resuelven con variables de entorno por perfil.
3. **Observar no cuesta tokens**: hooks HTTP de Claude Code + lectura del transcript. Nunca llamar a un modelo para observar.
4. **Baja fatiga visual/cognitiva**: superficies mate, colores de estado desaturados, un movimiento por estado, sin blur
   ni transparencia de ventana, nada que parpadee sin motivo, `prefers-reduced-motion` respetado.
5. **Mantener el editor** (CodeMirror 6) para revisar y retocar, pero sin competir con VS Code.
6. El **landing** (`moduszero.github.io/landing`) se conserva; **`pricing` no se toca** (puede que se use algún día).
7. **Agnóstico a la CLI/proveedor, pero Claude Code primero.** Luis aún no paga Claude Code: la vía principal es usar la CLI
   de Claude con **endpoints compatibles** (OpenRouter, Z.ai, Qwen/DashScope, DeepSeek, gateway propio) para conservar los
   hooks; las demás CLI (Aider, OpenCode, Qwen Code, Codex, Gemini) funcionan con estado inferido. Las claves se
   referencian como **`$NOMBRE`** (se leen del entorno del sistema al lanzar); no se construye un asistente de
   proveedores: "keep it simple", la configuración fina se hace desde la propia CLI.
8. **Documentación para humanos en `docs/`** (el landing renderiza `docs/README.md` y `docs/getting-started.md` desde
   `ModusZero/mod0`). **Sin wiki de GitHub**: es otro repo, no se revisa en PR y las IAs lo leen peor. `CONTEXT.md` y
   `PLAN.md` son para la IA; `README.md`, `getting-started.md` y `guide/*` para personas. Hay un `llms.txt` en la raíz.
9. **Mapa del proyecto**: sirve para orientarse en una base de código nueva, ver dónde trabaja cada sesión y estimar el
   **impacto** de un cambio; **no** para revisar (para eso, lista/árbol de cambios).
10. **Hoja de ruta**: 0.2 Mission Control · 0.3 Cualquier CLI · 0.4 Triggers · 0.5 Insight · 0.6 Remoto y conectores (MCP).

### KPIs que merecen atención (por sesión)

Estado (`starting/working/waiting/review/idle/exited`) · acción actual + archivo · **tiempo en estado** · plan (`TodoWrite`) · turnos ·
cambios (+/− líneas, archivos) · errores de herramientas · **contexto usado** (tokens del último mensaje vs límite).
`waiting` (bloqueada esperando una decisión) y `review` (terminó con cambios) son las que mandan en el panel de atención.

## 2. Arquitectura

```text
 Claude Code (PTY)  ──hooks HTTP──▶  hook_server (127.0.0.1:puerto, token)  ──▶ SessionManager::on_hook
        │                                                                          │  reduce(): estado + grafo + avisos
        └──escribe──▶ transcript.jsonl ──lectura incremental──▶ TranscriptCursor ──┘
                                                                                   │ emit
   Frontend (Svelte 5 runes) ◀── eventos session:upsert · graph:delta · fs:changed ┘
        ▲  invoke(comandos)      ◀── Channel<bytes> (salida cruda del PTY) ── xterm.js
```

### Backend (`src-tauri/src`, Rust 2024, Tauri 2)

| Módulo                    | Responsabilidad                                                                                                                                                                                                                   |
| ------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `config.rs`               | `AppConfig` (serde `camelCase`, todos los campos con valor por defecto) + `ConfigStore` (RwLock + JSON atómico).                                                                                                                  |
| `core/`                   | `AppError` (se serializa como texto), nombres/payloads de eventos, `AppState` y `Paths`.                                                                                                                                          |
| `infrastructure/`         | `storage` (JSON atómico, hash estable), `process` (`blocking`, `command` sin ventana en Windows, `now_ms`).                                                                                                                       |
| `features/sessions/`      | `model` (Session, estados, KPIs, **plan**), `profile` (perfiles de CLI; valores `$NOMBRE` resueltos en `launcher`), `launcher` (shell + comillas + línea de comandos), `manager` (registro de sesiones y **reductor de hooks**).  |
| `features/terminal/`      | `pty` (portable-pty; historial 1 MiB; `attach` atómico historial+canal), `ring_buffer`.                                                                                                                                           |
| `features/observer/`      | `hook_server` (tiny_http), `hooks_settings` (JSON de `--settings`), `transcript` (cursor JSONL), `graph` (thinking graph).                                                                                                        |
| `features/workspace/`     | `tree` (carga perezosa, respeta `.gitignore`), `io` (leer/escribir con límites), `deps` (grafo de dependencias **expandible**: `graph(root, expanded)` abre las carpetas indicadas hasta archivos; `impact` = recorrido inverso). |
| `features/git/`           | Usa el binario `git` (sin libgit2): `changes` vs commit base, `file_pair`, `commit_all`, worktrees.                                                                                                                               |
| `features/notifications/` | Escritorio (plugin) + webhook (`ureq`) con plantillas `{title} {message} {session} {status}` y `_url`/`_json`.                                                                                                                    |
| `features/kanban/`        | Persistencia atómica del tablero por proyecto (la forma del JSON la define el frontend).                                                                                                                                          |
| `commands/`               | Puente IPC (async; trabajo bloqueante a `spawn_blocking`).                                                                                                                                                                        |

**Reductor de estado** (`manager::reduce`, función pura probada): `UserPromptSubmit → working` · `PreToolUse/PostToolUse →
working` (+ nodos del grafo, archivos tocados) · `Notification(permission_prompt|elicitation_dialog) → waiting`
(`idle_prompt` se ignora) · `Stop → review` si hubo ediciones en el turno, si no `idle` · `SessionEnd → idle`.
Las CLI **sin hooks** (`observed=false`) infieren `working/idle` por silencio del PTY (2,5 s); el ticker (1 s) también
dispara el aviso "todo terminó" y, si los hooks no llegan en 20 s, degrada a inferencia.

**Hooks**: se pasa `--settings <data>/hooks/<sesión>.json` con eventos _estables_ (`SessionStart, SessionEnd,
UserPromptSubmit, PreToolUse, PostToolUse, Notification, Stop, SubagentStop, PreCompact`), cada uno
`{"type":"http","url":"http://127.0.0.1:PUERTO/hook/<sesión>/<token>","timeout":5}`. El servidor responde `{}` al instante.
Se evitan eventos nuevos para que versiones antiguas no rechacen el archivo.

**Persistencia** (carpeta de config/datos de Tauri): `config.json`, `sessions.json` (al reiniciar las sesiones quedan
`exited` y se reanudan con `--resume {session}`), `graphs/<sesión>.json`, `hooks/<sesión>.json`, `boards/<hash>.json`.

### Frontend (`src`, Svelte 5 runes, Tailwind 4)

- `lib/core/types` · `constants` (`work-config`, `family-config`, `default-config`, `webhook-presets`) · `runes`
  (singletons con estado: `sessionsStack`, `graphStack`, `filesStack`, `editorStack`, `reviewStack`, `kanbanStack`,
  `settingsStack`, `toggleUIStack`, `workStack`, `toastStack`) · `services` (un archivo por dominio sobre `invoke`) ·
  `registry` (`actions`, `commands` de la paleta) · `i18n` (`t()` reactivo, es/en planos) · `utils`.
- **Layout**: `Header` (marca, proyecto, paleta, contadores, controles de ventana propios) · `ActivityBar` · `Sidebar`
  (según actividad) · `Workspace` (vista + dock de terminal redimensionable) · `AttentionRail` (derecha) · `Footer`.
- **Secciones/actividades** (`ActivityIDs`): _Forge_ = `sessions` (Mission Control), `thinking-graph`, `kanban` ·
  _Pulse_ = `files` (árbol + grafo), `review`, `editor` · _Workspace_ = `settings`. `systems/<sección>/<actividad>/`
  tiene `XMainSpace` y `XSidebar`; las piezas viven en `features/system-atoms/…`.
- **Terminal**: una instancia xterm por sesión, creada al verla por primera vez; WebGL solo en la visible; el historial
  y los bytes en vivo llegan por `term_attach` (retorno = historial, `Channel` = vivo, sin duplicados).
- **Thinking graph**: vista **diagrama** (por defecto; `buildDiagram` agrupa pasos consecutivos en fases y marca reintentos) y
  **cronología** (`layoutGraph`, estilo `git log`); el **plan** sale de `TodoWrite`. **Grafo del proyecto**: `force-layout.ts`
  (determinista, ajustado a la vista), nodos `dir`/`file` expandibles, letras M/A/U/D e **impacto**. **Revisión**: lista plana
  o árbol por carpetas (`core/utils/tree.ts`).
- **Atajos** globales en `features/workbench/hotkeys.ts` (dentro de la terminal solo `Ctrl+Shift+…` y `Alt+n`).

### Contratos IPC

Comandos: `app_info` `notify_test` `profile_check` · `config_get/update/set_project` · `session_list/create/restart/stop/remove/rename/
set_reviewed/graph/refresh` · `term_attach/detach/write/resize` · `fs_list_dir/list_files/read/write/create_file/
create_dir/rename/delete/graph/impact` · `git_info` `review_changes/file/commit` · `kanban_load/save`.
Eventos: `session:upsert` (Session completa) · `session:removed` (id) · `graph:delta` (`{sessionId,nodes}`) ·
`fs:changed` (`{cwd}`; **sin watchers**: se emite tras actividad de las sesiones y al recuperar el foco).

## 3. Convenciones (ver también `CONTRIBUTING.md`)

- **Formato**: Prettier (`semi`, `singleQuote`, `trailingComma: all`, `printWidth: 110`, `tabWidth: 4`) y EditorConfig
  (lf, 4 espacios, sin espacios finales salvo `.md`); rustfmt `max_width = 110`. **Siempre** formatear antes de entregar.
- Organización por carpetas descrita arriba; JSDoc/comentarios en español; claves i18n planas `area.clave`.
- Nombres: `*.svelte.ts` para runes, `XMainSpace/XSidebar` para sistemas, `*-service.ts`, tipos en `core/types`.
- Rust: errores con `AppResult`, structs serde `camelCase`, lógica pura separada para poder probarla.

## 4. Stack y versiones (verificado el 2026-11-10)

**Tauri fijado**: los paquetes npm (`~2.11`) y los crates (`Cargo.lock`) deben coincidir en minor; al actualizar, ambos a la vez.
SvelteKit **3.1** (sin `svelte.config.js`: la config va en `sveltekit({...})` dentro de `vite.config.ts`; `$lib` fue
reemplazado por **`#lib`** → `package.json#imports` + `tsconfig.paths`), Svelte 5.57, Vite 8, TypeScript **~6.0**
(svelte-check/kit aún no admiten TS 7), Tailwind 4.3 (`@tailwindcss/vite`), `@lucide/svelte` (el nombre `Trash`, no
`Trash2`), xterm 6 + addon-fit/webgl, CodeMirror 6 (+ merge, language-data), Tauri 2 + plugins dialog/notification/
opener/os/window-state, `portable-pty` 0.9, `tiny_http`, `ureq` 3, `ignore`, `regex`. Landing: **Astro 7** (Node ≥ 22.23.4),
`@astrojs/svelte` 9 + React 7, Tailwind vía `@tailwindcss/vite`.

Instalador: **NSIS** en modo usuario con imágenes de marca (`src-tauri/installer/*.bmp`); ya no se genera MSI.

## 5. Landing (`moduszero/landing`, Astro)

Pantalla de producto `components/products/Mod0.astro` rehecha: hero con captura real, filosofía, **recorrido interactivo**
(`components/ide/*.svelte`: maquetas vivas portadas de los componentes del IDE con datos de ejemplo y simulación
compartida), sección "observa sin tokens", features, workflow, galería (`public/images/ide/{es,en}`), compilar y FAQ/
roadmap. Textos en `i18n/locales/{es,en}.json` → `products.mod0.*`. **Rendimiento**: fondo de Vanta/three.js (CDN, blur
de pantalla completa) sustituido por un canvas 2D que se pausa; cursor personalizado solo con ratón y con `quickTo`.
Cuando cambie la UI del IDE hay que **regenerar capturas** y **actualizar las maquetas** (ver `docs/PLAN.md`).

## 6. Verificación y límites conocidos

- **Verificado**: `cargo clippy` limpio y 22 pruebas unitarias de Rust (reductor de hooks y plan, grafo, transcript, deps
  expandibles e impacto, quoting, referencias `$VAR`, notificaciones, git slug); `svelte-check` 0 errores; build del frontend; build del landing (31 páginas);
  renderizado de las pantallas con un backend simulado (`scripts/`).
- **No verificado** (entorno sin GUI ni Claude Code): arranque completo de la ventana Tauri, ConPTY en Windows, hooks
  HTTP con tu versión real de Claude Code, `--settings`, notificaciones nativas y el instalador NSIS. Probar primero.
- Límites de diseño: sin búsqueda de contenido en archivos; sin LSP; los estados exactos, el grafo y el plan requieren hooks
  (solo Claude Code); el trabajo remoto (SSH/Codespaces) solo ofrece terminal (revisión y grafos son locales); el prompt inicial se aplana a una línea; los perfiles/webhooks se guardan en texto plano; los
  nodos del grafo se limitan a 1200 por sesión.

## 7. Cómo trabajar este proyecto con Claude

1. Entrega por **release**: carpetas completas listas para reemplazar (`src`, `src-tauri`, config, docs) en zip, con el
   formato ya aplicado (Prettier/rustfmt) y verificadas (check, build, tests).
2. En **cada release** actualizar `docs/CONTEXT.md` (si cambia algo estructural), `docs/PLAN.md` (tareas, registro),
   `CHANGELOG.md` y las versiones (`package.json`, `Cargo.toml`, `tauri.conf.json`). La skill `mod0-release-sync`
   automatiza y comprueba esto.
3. No introducir dependencias pesadas sin motivo; preferir lo simple (p. ej. `git` CLI en vez de libgit2).
4. Ser honesto con lo no verificado y anotarlo en `PLAN.md`.
