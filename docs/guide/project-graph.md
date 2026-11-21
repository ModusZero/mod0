# Mapa del proyecto

Una forma de ver la base de código que **ni el árbol ni una lista de archivos permiten**: a grandes rasgos, a la
escala que elijas.

## Qué muestra

- **Módulos y archivos**: las carpetas se agregan en un nodo (con su número de archivos); **doble clic** abre una
  carpeta en sus hijos y puedes llegar hasta archivos sueltos. El control **Nivel** abre o cierra todo hasta una
  profundidad. Las flechas son _imports_; su grosor, cuántos.
- **Dónde trabaja cada sesión**: anillos del color de la sesión (pulsan mientras la IA trabaja).
- **Qué cambió**: los nodos con archivos modificados se resaltan con sus letras `M` modificado, `A` añadido, `U` sin
  seguimiento, `D` borrado (las mismas que ves en el árbol de archivos).
- **Impacto**: el botón ⚡ resalta, con un borde discontinuo, los módulos o archivos que **dependen** (directa o
  indirectamente) de lo que cambió la sesión enfocada: el radio de alcance del cambio.

Se analizan TS/JS, Svelte, Vue, Astro, Python y Rust. Las dependencias se calculan por heurística (imports, `$lib`/`#lib`,
`mod`/`use crate`): úsalas como mapa, no como verdad absoluta.

## Cuándo usarlo y cuándo no

- **Sí**: entender una base de código por primera vez, ver dónde trabajan las sesiones y estimar el alcance de un cambio.
- **No**: para revisar. Para eso están la **lista o el árbol de archivos cambiados** de [Revisión](https://github.com/ModusZero/mod0/blob/main/docs/guide/review.md)
  y las letras M/U/A/D en el árbol de archivos.
