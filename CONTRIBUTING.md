# Contribuir a MOD0

Gracias por el interés. MOD0 es una herramienta personal con un objetivo claro: **optimizar el flujo de trabajo con
muchas sesiones de CLI con IA en paralelo**. Las contribuciones que encajan con eso son bienvenidas; si dudas, abre
primero un issue o una discusión.

## Entorno

Requisitos: Node.js 22+, Rust 1.85+, `git` y los [requisitos de Tauri](https://v2.tauri.app/start/prerequisites/).

```bash
npm install
npm run tauri dev     # app completa (frontend + backend)
npm run dev           # solo frontend (necesita el backend de Tauri para funcionar; ver "Capturas")
```

| Comando                                                                    | Qué hace                        |
| -------------------------------------------------------------------------- | ------------------------------- |
| `npm run lint`                                                             | Comprueba el formato (Prettier) |
| `npm run format`                                                           | Aplica el formato               |
| `npm run check`                                                            | Tipos y a11y con `svelte-check` |
| `npm run build`                                                            | Build del frontend (`build/`)   |
| `cargo fmt` / `cargo clippy --all-targets` / `cargo test` (en `src-tauri`) | Formato, lint y pruebas de Rust |

La CI ejecuta todo lo anterior; un PR debe pasarlo.

## Estilo de código

El formato lo imponen las herramientas, no la memoria:

- **Prettier**: punto y coma, comillas simples, comas finales, **110 columnas**, **4 espacios** (`.prettierrc` y
  `.editorconfig`). Los saltos de línea al pasar el ancho los decide Prettier: ejecuta `npm run format`.
- **rustfmt**: `max_width = 110`, 4 espacios (`src-tauri/rustfmt.toml`).
- Fin de línea `lf` (`.gitattributes` lo fuerza aunque trabajes en Windows).
- Comentarios y textos de interfaz en **español**; identificadores y mensajes de commit en inglés o español, con
  coherencia dentro del archivo. Los comentarios explican el _porqué_, no el _qué_.

### Organización

- **Frontend** (`src/lib`): `core/` (types, constants, runes, services, registry, i18n, utils) · `components/`
  (common, layout) · `features/` (funcionalidades y `system-atoms/` con las piezas de cada pantalla) · `systems/`
  (una carpeta por sección → actividad, con `XMainSpace.svelte` y `XSidebar.svelte`).
- El estado vive en clases con runes (`*.svelte.ts`) exportadas como singletons (`sessionsStack`, `filesStack`…).
- Los imports internos usan `#lib/…` (subpath imports de SvelteKit 3; el alias `$lib` está deprecado).
- **Backend** (`src-tauri/src`): `commands/` es solo el puente IPC; la lógica vive en `features/`. Los comandos son
  `async` y mandan el trabajo bloqueante a `infrastructure::process::blocking`.
- Textos nuevos: `src/lib/core/i18n/locales/es.json` **y** `en.json` (claves planas con puntos).

## Añadir un comando IPC

1. Función en `src-tauri/src/commands/<área>.rs` con `#[tauri::command]` y `AppResult<T>`.
2. Registrarla en `tauri::generate_handler![…]` de `src-tauri/src/lib.rs`.
3. Tipos en `src/lib/core/types/` (los structs de Rust usan `camelCase` con serde).
4. Método en el servicio correspondiente de `src/lib/core/services/`.
5. Prueba unitaria en Rust si hay lógica no trivial (parsers, reductores de estado, grafos).

## Actualizar Tauri

Los paquetes npm y los crates deben coincidir en versión mayor y menor (si no, `tauri dev` avisa de «versiones
desalineadas»). Actualiza ambos a la vez; el detalle está en [docs/guide/troubleshooting.md](docs/guide/troubleshooting.md).

## Capturas de pantalla reproducibles

`scripts/mock-tauri.js` simula el backend de Tauri para ver la interfaz en un navegador. Con él,
`scripts/capture-screenshots.py` genera las capturas del README y del landing:

```bash
pip install playwright && playwright install chromium
npm run build
python scripts/capture-screenshots.py docs/screenshots es,en
```

## Commits y PR

- Commits pequeños, con [Conventional Commits](https://www.conventionalcommits.org/es/) (`feat:`, `fix:`, `docs:`…).
- Rellena la plantilla del PR. Si cambias la interfaz, adjunta una captura.
- Actualiza `docs/PLAN.md` (tareas y registro de releases) y el `CHANGELOG.md`. Si cambia la arquitectura, también
  `docs/CONTEXT.md`.

## Publicar una versión

1. Sube la versión en `package.json`, `src-tauri/Cargo.toml` y `src-tauri/tauri.conf.json` (deben coincidir).
2. Mueve lo pendiente de `CHANGELOG.md` a la nueva versión.
3. `git tag vX.Y.Z && git push --tags`: el workflow `release.yml` valida la coherencia y publica un **borrador** de
   release con los instaladores (Windows NSIS, macOS DMG, Linux AppImage/deb). No están firmados.
