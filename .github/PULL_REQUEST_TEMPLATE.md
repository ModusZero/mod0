## Qué cambia

<!-- Resume el cambio en 1-3 líneas y enlaza el issue si existe (Closes #123). -->

## Por qué

<!-- El problema o la necesidad. -->

## Cómo se probó

- [ ] `npm run lint` y `npm run check`
- [ ] `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` y `cargo test` en `src-tauri`
- [ ] Probado a mano con `npm run tauri dev` (indica con qué CLI y sistema operativo)

## Checklist

- [ ] Sigo el estilo del proyecto (Prettier 110 columnas, 4 espacios, `#lib/`, runes `*.svelte.ts`)
- [ ] Textos nuevos en `es.json` **y** `en.json`
- [ ] Actualicé `docs/PLAN.md` (y `docs/CONTEXT.md` si cambia la arquitectura) y el `CHANGELOG.md`
- [ ] Si cambia la interfaz: adjunto captura (`scripts/capture-screenshots.py`)
