# Changelog

Formato basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/) y versionado semántico.

## [0.3.0] - 2026-11-16

"Cualquier CLI": proveedores, plan de la sesión y un mapa del proyecto explorable.

### Añadido

- **Perfiles**: plantillas para Claude Code con OpenRouter, Z.ai, Qwen/DashScope, DeepSeek o endpoint propio, y para
  Aider, OpenCode, Qwen Code, Codex, Gemini y un Codespace remoto (`gh`). Comprobación de que la CLI está instalada y de
  las variables de entorno definidas. Valores `$NOMBRE` leídos del entorno del sistema (las claves no se guardan).
- **Thinking Graph como diagrama** de fases con flechas y reintentos (además de la cronología) y **plan** de la sesión
  (`TodoWrite`) con progreso en la tarjeta y en la barra lateral.
- **Mapa del proyecto expandible**: abre carpetas hasta archivos con doble clic o con el control de nivel; resalta los
  archivos cambiados (M/A/U/D) y calcula el **impacto** (qué depende de lo que cambió la sesión).
- **Árbol de archivos** con letras M/A/U/D y marca de carpetas con cambios de la sesión enfocada.
- **Revisión** con lista plana o árbol por carpetas; selección automática del primer archivo.
- Guardado automático opcional del editor (pendiente de la configuración antigua).
- Documentación para personas en `docs/` (guías, primeros pasos, proveedores, problemas típicos), `llms.txt`.
- Configuración de Cargo para conexiones lentas (`src-tauri/.cargo/config.toml`).

### Cambiado

- Paquetes npm de Tauri fijados a la misma versión menor que los crates (`~2.11`), para evitar el aviso de versiones
  desalineadas.
- `fs_graph` recibe las carpetas expandidas en vez de una profundidad.
- Hoja de ruta: 0.3 Cualquier CLI · 0.4 Triggers · 0.5 Insight · 0.6 Remoto y conectores.

## [0.2.0] - 2026-11-10

Reescritura completa: de IDE genérico y comercial a **mission control** para el flujo personal de trabajo con
sesiones de CLI con IA en paralelo.

### Añadido

- Sesiones con terminal real (PTY, `portable-pty`), perfiles de CLI (BYOK/IA local por variables de entorno),
  worktrees aislados, reanudación (`--resume`) y persistencia entre reinicios.
- Observación sin tokens de Claude Code: hooks HTTP locales + lectura incremental del transcript.
- Mission Control, panel de atención, Thinking Graph, grafo de dependencias del proyecto, revisión con diff y commit,
  Kanban ligado a sesiones, editor CodeMirror 6, paleta de comandos y atajos.
- Avisos de escritorio y webhooks (plantillas ntfy, Telegram, WhatsApp/CallMeBot, JSON).
- Interfaz en español e inglés; temas oscuro/claro con acento configurable y baja fatiga visual.
- Instalador NSIS moderno (modo usuario, imágenes de marca) en lugar de MSI.
- Documentación (`docs/CONTEXT.md`, `docs/PLAN.md`), CI/CD, plantillas de GitHub y guía de contribución.

### Cambiado

- Stack actualizado: SvelteKit 3 (configuración en el plugin de Vite, `#lib/` en vez de `$lib/`), Vite 8,
  TypeScript 6, Tailwind 4 con `@tailwindcss/vite`, `@lucide/svelte`, xterm 6, Tauri 2.

### Eliminado

- Chat, artefactos, contexto, workflow, plugins, LSP, SQLite/sqlx y el sistema de extensiones: la CLI ya cubre esas
  funciones y el IDE era un envoltorio comercial genérico que no compilaba.
