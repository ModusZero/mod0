# Sesiones

## Crear una sesión

`Ctrl+Shift+N` (o el botón **Nueva sesión**). Campos:

- **Nombre**: se usa también para el worktree y la rama.
- **CLI**: un perfil de _Ajustes → Perfiles de CLI_.
- **Aislamiento**: _Misma carpeta_ (dos sesiones pueden pisarse archivos) o **Worktree propio**: crea una rama
  (`mod0/<nombre>`) y una carpeta hermana del proyecto. Ideal para varias sesiones en paralelo.
- **Prompt inicial** (opcional): se pasa a la CLI como argumento y se aplana a una línea.

## Moverte entre sesiones

- `Alt+1…9` salta a la sesión _n_ (en orden de creación).
- `Ctrl+Shift+A` salta a la siguiente que te necesita (bloqueada o por revisar).
- El panel de la derecha muestra todas, siempre visible; doble clic abre su thinking graph.

## Detener, reanudar, eliminar

En cada tarjeta: **Detener**, **Reanudar** (con Claude Code retoma la conversación con `--resume`), **Renombrar** y
**Eliminar** (te pregunta si quieres borrar también el worktree: los cambios sin commit se pierden).

## Persistencia

Al cerrar MOD0, las sesiones quedan como _finalizadas_ con su historial de pasos; al abrirlo puedes reanudarlas.
