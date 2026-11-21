# Arquitectura (para contribuir)

Versión resumida. El detalle completo, pensado también para IA, está en [`CONTEXT.md`](../CONTEXT.md).

```text
 Claude Code (PTY)  ──hooks HTTP──▶  servidor local 127.0.0.1  ──▶  reductor de estado de la sesión
        └──escribe──▶ transcript.jsonl ──lectura incremental──────────┘            │ eventos
   Frontend (Svelte 5) ◀───── session:upsert · graph:delta · fs:changed ◀───────────┘
        ▲ comandos (invoke) · terminal por canal de bytes crudos
```

- **Backend** (`src-tauri/src`, Rust): `features/` con la lógica (sesiones, terminal, observador, archivos, git,
  avisos, kanban) y `commands/` como puente IPC. El reductor de estado (`sessions/manager.rs::reduce`) es una función
  pura con pruebas.
- **Frontend** (`src`, SvelteKit 3 en modo SPA, Svelte 5 con _runes_): `core/` (tipos, estado, servicios, i18n),
  `components/` (piezas comunes y marco), `features/` y `systems/` (una carpeta por sección → actividad).
- **Terminal**: un PTY por sesión; la salida se guarda en un búfer y solo viaja al frontend mientras haya una terminal
  visible, así 15 sesiones no cuestan nada en la interfaz.
- **Grafos**: el análisis de imports se calcula en Rust y se cachea; el diseño por fuerzas se hace en el frontend.
- **Sin base de datos ni vigilantes de archivos**: JSON atómico y refresco por actividad de las sesiones.

Convenciones y flujo de trabajo: [`CONTRIBUTING.md`](../../CONTRIBUTING.md).
