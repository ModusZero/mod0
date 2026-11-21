# MOD0 · Plan actual

> Tareas vivas y registro de releases. **Claude lo actualiza en cada release** (ver `docs/CONTEXT.md` §7).
> Versión actual: **0.3.0** · Última actualización: **2026-11-20**.

## Estado en una línea

0.3.0 entregada: proveedores/CLI agnóstico, plan de la sesión, thinking graph como diagrama, mapa del proyecto expandible
con impacto, revisión en lista/árbol, documentación para personas. **Sigue pendiente la verificación en una máquina real
con Claude Code**: es lo primero antes de añadir más funciones.

## Ahora (v0.3.1 · verificación en máquina real) — prioridad alta

- [ ] `npm run tauri dev` en Windows (la compilación de Rust necesita buena conexión la primera vez: ver
      `docs/guide/troubleshooting.md`). La ventana abre, la barra de título propia y el redimensionado funcionan.
- [ ] Sesión con el perfil **Claude Code**: la CLI arranca dentro del PTY (ConPTY), el estado pasa a `idle`/`working` y el
      «Observador» del pie está en verde. Comprobar que **`--settings` + hooks HTTP** funcionan con tu versión.
- [ ] Un turno completo: thinking graph (diagrama y cronología), **plan** (`TodoWrite`), `waiting` al pedir un permiso,
      `review` al terminar con ediciones, aviso de escritorio y webhook.
- [ ] **Claude Code con otro proveedor** (plantilla OpenRouter/Z.ai…): comprobar que arranca con las variables `$NOMBRE`
      y que los hooks siguen llegando.
- [ ] Probar una CLI sin hooks (Aider/OpenCode) y el estado inferido por actividad.
- [ ] Worktrees: crear, revisar (diff contra el commit base, lista y árbol) y eliminar con su worktree; reanudar (`--resume`).
- [ ] Mapa del proyecto en un repo real: expandir, cambios M/U/A/D e impacto; revisar el tiempo en una base de código grande.
- [ ] xterm con WebGL al 125 % / 150 % en Windows (si falla, desactivar WebGL por defecto).
- [ ] Instalador NSIS: aspecto, instalación por usuario, desinstalación.
- [ ] Verificar los endpoints y variables de cada plantilla de proveedor con las claves reales (cambian con frecuencia).
- [ ] Anotar aquí cualquier ajuste necesario y corregirlo.

## Siguiente (v0.4 · Triggers)

- [ ] Triggers: lanzar una sesión cuando ocurre un evento (release nuevo, CI que falla, ticket).
- [ ] Plantillas de prompt por tarea del Kanban.
- [ ] Búsqueda de contenido en archivos (`ignore` + regex), si hace falta.

## Backlog

- [ ] v0.5 _Insight_: coste por sesión, tendencia de contexto, historial de lo que hizo cada sesión.
- [ ] v0.6 _Remoto y conectores_: espacio de trabajo remoto (git y archivos por SSH, túnel inverso para los hooks) para
      Codespaces/SSH; conectores MCP (Jira, Notion) para planear fuera del IDE; acompañante móvil para aprobaciones.
- [ ] Observación exacta para más CLI (p. ej. si OpenCode/Codex exponen eventos).
- [ ] Guardar secretos de webhooks en el llavero del sistema.
- [ ] Reordenar sesiones (arrastrar) y agruparlas por proyecto; terminales en mosaico.
- [ ] Documentación de `docs/` también en inglés.
- [ ] Landing: actualizar maquetas (`components/ide/`) y capturas cada vez que cambie la UI.

## Decisiones recientes

| Fecha      | Decisión                                                                                                       |
| ---------- | -------------------------------------------------------------------------------------------------------------- |
| 2026-11-10 | El proyecto deja de ser comercial; se rehace para el flujo personal (mission control de sesiones de CLI).      |
| 2026-11-10 | Observación por hooks HTTP + transcript (cero tokens); sin hooks se infiere por actividad del PTY.             |
| 2026-11-10 | `git` CLI en vez de libgit2; sin SQLite ni extensiones; sin watchers de archivos.                              |
| 2026-11-10 | SvelteKit 3: configuración en `vite.config.ts` y alias `#lib` (en vez de `$lib`).                              |
| 2026-11-20 | Agnóstico: Claude Code con endpoints compatibles primero; claves como `$NOMBRE`; sin asistente de proveedores. |
| 2026-11-20 | Thinking graph como diagrama de fases por defecto; plan de `TodoWrite` visible.                                |
| 2026-11-20 | Mapa del proyecto: expandible hasta archivos, con cambios e impacto; la revisión usa lista/árbol, no el grafo. |
| 2026-11-20 | Sin wiki de GitHub: la documentación vive en `docs/` (la renderiza el landing) y hay `llms.txt`.               |
| 2026-11-20 | Paquetes npm de Tauri fijados al mismo minor que los crates (evita el aviso de versiones desalineadas).        |

## Deuda técnica conocida

- Sin pruebas del frontend (solo `svelte-check` y render con backend simulado).
- `docs/` era antes un clon del wiki (el `.gitignore` lo ignoraba): ahora es parte del repo; mueve el clon del wiki fuera
  y desactiva la wiki en GitHub si no la vas a usar.
- El landing instala `oxc-transform-react` por el peer de `@astrojs/react` 7 y `@lucide/svelte` requiere `noExternal` en Vite.
- Las plantillas de proveedores son referencias: los endpoints y nombres de modelo cambian y deben comprobarse.

## Registro de releases

### 0.3.0 · 2026-11-20 · "Cualquier CLI"

**IDE**: plantillas de perfiles (Claude Code + OpenRouter/Z.ai/Qwen/DeepSeek/gateway; Aider, OpenCode, Qwen Code, Codex,
Gemini, Codespace), comprobación de CLI y de variables `$NOMBRE`; plan de la sesión; thinking graph como diagrama;
mapa del proyecto expandible con cambios e impacto; árbol de archivos con M/U/A/D; revisión en lista/árbol; auto-guardado;
npm de Tauri alineado con los crates; configuración de Cargo para redes lentas.
**Landing**: maquetas realmente explorables (aprobar, filtrar, arrastrar, expandir, impacto, árbol/lista, commit) y una
historia guionada más larga; cursor personalizado corregido (sin doble puntero y también con ventana estrecha); hoja de
ruta y copy actualizados; `llms.txt`.
**Documentación**: `docs/README.md`, `getting-started.md` y `guide/*` para personas; `llms.txt`.
**Verificación**: clippy limpio, 22 pruebas Rust, svelte-check 0 errores, builds de IDE y landing, render con mock y prueba
de las maquetas con Playwright.
**Pendiente**: lo de la sección «Ahora».

### 0.2.0 · 2026-11-10 · "Mission Control"

**IDE**: reescritura de `src/` y `src-tauri/` (sesiones con PTY real, perfiles de CLI, hooks + transcript, thinking graph,
grafo del proyecto, revisión con diff/commit, Kanban, editor, paleta, atajos, avisos, es/en, temas). Instalador NSIS.
Docs, CI/CD, plantillas de GitHub, CONTRIBUTING, SECURITY, CHANGELOG.
**Landing**: pantalla de producto rehecha (maquetas Svelte, capturas reales, copy no comercial), fondo y cursor
optimizados, Astro 7, `pricing` intacto.
