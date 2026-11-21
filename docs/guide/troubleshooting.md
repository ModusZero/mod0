# Solución de problemas

## `npm run tauri dev`: «Found version mismatched Tauri packages»

Los paquetes npm de Tauri y los crates de Rust deben coincidir en versión mayor y **menor**. El proyecto los fija a la
misma (npm con `~`, Rust con `Cargo.lock`). Si actualizas uno, actualiza el otro:

```bash
cd src-tauri && cargo update -p tauri -p tauri-plugin-dialog -p tauri-plugin-opener -p tauri-plugin-os
cd .. && npm install @tauri-apps/api@latest @tauri-apps/cli@latest @tauri-apps/plugin-dialog@latest \
  @tauri-apps/plugin-opener@latest @tauri-apps/plugin-os@latest
```

## Cargo: «[28] Timeout was reached (Operation too slow…)» al descargar crates

Es un problema de **conexión**, no del proyecto: Cargo descarga unas 400 dependencias y aborta si la velocidad baja de
10 bytes/s durante 30 s. Qué hacer:

- `src-tauri/.cargo/config.toml` ya sube el tiempo límite y los reintentos (`timeout = 300`, `low-speed-limit = 1`,
  `retry = 10`, sin multiplexado).
- **Descarga primero con buena conexión** (datos del móvil, otra red) y compila después sin red:
  `cd src-tauri && cargo fetch` y luego `cargo build --offline`.
- Reintenta: lo ya descargado queda en caché y cada intento avanza.
- Un _mirror_ de crates.io (`[source]` en `~/.cargo/config.toml`) también ayuda si tu red lo bloquea.

## «No se encontró `claude` en el PATH»

Instala la CLI (`npm install -g @anthropic-ai/claude-code`) y comprueba que su carpeta está en el `PATH` del sistema.
Tras cambiar el `PATH` o variables de entorno **reinicia MOD0**: las lee al arrancar.

## La sesión se queda en «Iniciando» y no aparece el thinking graph

MOD0 espera 20 s los hooks de Claude Code; si no llegan, pasa a inferir el estado por actividad. Comprueba:
_Ajustes → Acerca de → Observador_ (debe mostrar un puerto), que el perfil tenga **hooks** activados, y que tu
configuración de Claude Code no restrinja los hooks HTTP (`allowedHttpHookUrls`).

## Variables de entorno sin definir

El perfil muestra «Variables de entorno sin definir: X». Define `X` en el sistema (`setx X "valor"` en Windows) y
reinicia MOD0.

## La terminal se ve mal o con letra enorme

xterm usa WebGL en la sesión visible. Si en tu equipo (p. ej. con escala del 125 % o 150 %) se ve distorsionado,
cámbialo en un issue indicando tarjeta gráfica y escala; el plan es desactivar WebGL por defecto si hace falta.

## Windows pide confirmación al instalar

Los instaladores no están firmados. SmartScreen puede avisar la primera vez: _Más información → Ejecutar de todas formas_.
