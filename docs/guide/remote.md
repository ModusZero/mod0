# Trabajo remoto

Idea: que el agente siga trabajando aunque tu conexión o tu equipo se caigan, ejecutándolo en un servidor (por ejemplo un
**GitHub Codespace** o una máquina por SSH).

## Lo que funciona hoy

Una sesión es una terminal: puedes lanzar una conexión remota como si fuera una CLI.

- Plantilla **Codespace remoto (gh)**: ejecuta `gh codespace ssh`. Dentro del Codespace usa **tmux**
  (`tmux new -A -s mod0`) y lanza allí la CLI: si se corta la conexión, la sesión sigue viva y al reconectar la retomas.
- También sirve un perfil con `ssh usuario@servidor`.

## Límites actuales

- La **revisión**, el **mapa del proyecto** y el **árbol de archivos** trabajan sobre carpetas **locales**: no ven los
  archivos del servidor.
- Los **hooks** de Claude Code apuntan a `127.0.0.1` de tu equipo: en remoto no llegan, así que el estado se infiere por
  actividad (sin thinking graph ni plan).

## Hacia dónde va

El plan (v0.6) es un espacio de trabajo remoto de verdad: ejecutar `git` y leer archivos por SSH y reenviar los hooks con
un túnel inverso, para tener revisión, grafos y observación completa también en remoto. Mira `PLAN.md`.
