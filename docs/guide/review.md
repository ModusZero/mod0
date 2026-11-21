# Revisión

El trabajo real del humano es **revisar**. Cada sesión se compara contra el **commit con el que empezó** (no solo contra
`HEAD`), así ves todo lo que hizo aunque haya hecho commits por su cuenta.

## Lista de archivos cambiados

Panel lateral, con estado `M`/`A`/`D`/`U`, líneas +/− y una marca de **revisado** por archivo. Dos vistas:

- **Lista plana**: todos los archivos con su carpeta.
- **Árbol por carpetas**: como el control de versiones de un editor, con carpetas compactadas (`src/auth`) y el número
  de archivos cambiados en cada una.

Al abrir la pestaña se selecciona el primer archivo para revisar sin clics extra.

## Diff

Lado a lado (CodeMirror), con lo no modificado colapsado. Desde ahí puedes **abrir el archivo en el editor** para
retocar (con _Guardado automático_ opcional en Ajustes).

## Aprobar y hacer commit

- **Marcar revisada** marca la sesión como revisada (sale de _Por revisar_).
- **Commit** hace `git add -A` + `git commit` con tu mensaje (por defecto, el nombre de la sesión) y marca la sesión
  como revisada.
