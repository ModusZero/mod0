# Conceptos

## Sesión

Una **sesión** es una terminal real (un PTY) que ejecuta tu shell y, dentro, una CLI de IA, en una carpeta de trabajo:
la del proyecto o un **worktree** propio. MOD0 puede tener 8–15 sesiones a la vez y recuerda las que cerraste para que
puedas **reanudarlas** (con Claude Code, usando `--resume`).

## Estados

| Estado          | Significado                                 | Qué hacer                      |
| --------------- | ------------------------------------------- | ------------------------------ |
| **Trabajando**  | La IA está ejecutando pasos.                | Nada: déjala.                  |
| **Esperando**   | Bloqueada: pidió un permiso o una decisión. | Atenderla ya (`Ctrl+Shift+A`). |
| **Por revisar** | Terminó un turno **con cambios**.           | Revisar lo que hizo.           |
| **En reposo**   | Sin nada pendiente.                         | —                              |
| **Finalizada**  | El proceso terminó.                         | Reanudar o eliminar.           |

_Esperando_ y _Por revisar_ mandan en el panel de atención.

## Indicadores por sesión

Estado · acción actual y archivo · **tiempo en ese estado** · turnos · líneas +/− y archivos cambiados · errores de
herramientas · **contexto usado** (tokens del último mensaje frente al límite) · progreso del **plan** si la CLI lo usa.

## Cómo observa MOD0 (sin gastar tokens)

- **Claude Code**: se lanza con `--settings <archivo>` que registra _hooks HTTP_ hacia un servidor local
  (`127.0.0.1`, puerto aleatorio y token por ejecución). Los hooks **se suman** a los tuyos. El _porqué_ de cada paso sale
  del **transcript** que Claude Code ya escribe (lectura incremental, solo lectura).
- **Otras CLI**: el estado se **infiere de la actividad de la terminal** (salida reciente = trabajando). No hay thinking
  graph, plan ni estados exactos.

Observar **nunca** llama a un modelo.
