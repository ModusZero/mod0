# Documentación de MOD0

MOD0 es **mission control para tus sesiones de CLI con IA**: corres varias sesiones en paralelo (Claude Code y otras) y
ves de un vistazo qué hace cada una, en qué archivo trabaja, qué está pensando y cuál te necesita. No reemplaza tu CLI:
la envuelve y le pone una cabina encima.

> MOD0 es una herramienta personal, abierta y **no comercial**. Esta documentación es la que usa quien quiera
> probarla, estudiarla o contribuir. Si buscas el contexto que usa la IA del proyecto, mira `CONTEXT.md` y `PLAN.md`.

## Empezar

- [Primeros pasos](https://github.com/ModusZero/mod0/blob/main/docs/getting-started.md): requisitos, instalación y tu primera sesión.
- [Solución de problemas](https://github.com/ModusZero/mod0/blob/main/docs/guide/troubleshooting.md): errores típicos de instalación y de uso.

## Guías

| Guía                                                                                         | Para qué sirve                                                                                        |
| -------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| [Conceptos](https://github.com/ModusZero/mod0/blob/main/docs/guide/concepts.md)              | Sesiones, estados, indicadores y cómo observa MOD0 sin gastar tokens.                                 |
| [Sesiones](https://github.com/ModusZero/mod0/blob/main/docs/guide/sessions.md)               | Crear, aislar con worktrees, reanudar y saltar entre sesiones.                                        |
| [Thinking Graph](https://github.com/ModusZero/mod0/blob/main/docs/guide/thinking-graph.md)   | El razonamiento de una sesión como diagrama o cronología, y su plan.                                  |
| [Mapa del proyecto](https://github.com/ModusZero/mod0/blob/main/docs/guide/project-graph.md) | Grafo expandible por niveles, cambios resaltados e impacto.                                           |
| [Revisión](https://github.com/ModusZero/mod0/blob/main/docs/guide/review.md)                 | Diff por sesión, lista o árbol, marcar revisado y commit.                                             |
| [Kanban](https://github.com/ModusZero/mod0/blob/main/docs/guide/kanban.md)                   | Tareas que lanzan sesiones y se mueven solas.                                                         |
| [Proveedores y CLI](https://github.com/ModusZero/mod0/blob/main/docs/guide/providers.md)     | Claude Code con otros modelos, Aider, OpenCode, Qwen Code, Codex, Gemini, claves y modelos gratuitos. |
| [Avisos](https://github.com/ModusZero/mod0/blob/main/docs/guide/notifications.md)            | Escritorio y webhooks (WhatsApp, Telegram, ntfy).                                                     |
| [Trabajo remoto](https://github.com/ModusZero/mod0/blob/main/docs/guide/remote.md)           | Sesiones en Codespaces o por SSH y sus límites actuales.                                              |
| [Arquitectura](https://github.com/ModusZero/mod0/blob/main/docs/guide/architecture.md)       | Cómo funciona por dentro, para quien quiera contribuir.                                               |

## Para IA

El archivo [llms.txt](https://github.com/ModusZero/mod0/blob/main/llms.txt) resume dónde está cada cosa para que otras IAs
puedan consultar el proyecto.
