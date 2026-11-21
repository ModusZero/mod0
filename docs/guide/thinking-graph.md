# Thinking Graph

El razonamiento de una sesión como **grafo** en lugar de un muro de texto.

## Dos vistas

- **Diagrama** (por defecto): cada turno se resume en **fases** encadenadas con flechas. Pasos consecutivos del mismo
  tipo se agrupan en una caja (_Lectura ×4_, _Edición ×3_, _Comando ×2_…). Así se ve de lejos el patrón
  explorar → editar → verificar → reintentar → listo. Las llamadas en paralelo y los reintentos tras un fallo (una
  fase de edición justo después de una que falló) se dibujan como tales. Haz clic en una fase para ver sus pasos.
- **Cronología**: paso a paso, como un historial de git, con las ramas paralelas a la derecha y el _porqué_ de cada paso.

Los turnos antiguos se colapsan solos; _Seguir_ mantiene la vista en lo último que hace la IA.

## El plan

Si la CLI usa su lista de tareas (`TodoWrite` en Claude Code), MOD0 muestra el **plan** en la barra lateral y un
progreso `3/7` en la tarjeta de la sesión: cómo va **desagregando tareas** y cuál está haciendo ahora.

## De dónde salen los datos

De los hooks de Claude Code (`PreToolUse`, `PostToolUse`, `Stop`…) y del transcript (el texto previo a cada
herramienta, los errores y el contexto usado). No cuesta tokens. Con otras CLI no hay thinking graph.

## ¿Una sesión hace una sola cosa a la vez?

Casi siempre es una cadena secuencial; a veces lanza varias herramientas en paralelo (ramas hermanas) o delega en
subagentes. Por eso el diagrama agrupa por fases y la cronología admite ramas.
