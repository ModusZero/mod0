# Primeros pasos

## Requisitos

- **Node.js 22+** y **Rust 1.85+** ([rustup.rs](https://rustup.rs)).
- Los [requisitos de Tauri](https://v2.tauri.app/start/prerequisites/) de tu sistema (en Windows: WebView2 y las
  herramientas de compilación de C++).
- **git** (la revisión y los worktrees lo usan) y la **CLI de IA** que quieras (por ejemplo Claude Code).

## Instalar desde el código

```bash
git clone https://github.com/ModusZero/mod0.git
cd mod0
npm install
npm run tauri dev      # modo desarrollo
npm run tauri build    # instalador
```

La primera compilación de Rust descarga y compila unas 400 dependencias y puede tardar varios minutos. Si tu conexión es
lenta, consulta [Solución de problemas](https://github.com/ModusZero/mod0/blob/main/docs/guide/troubleshooting.md).

Cuando haya versiones publicadas, los instaladores aparecen en la sección _Releases_ del repositorio (no están firmados:
Windows o macOS pueden mostrar una advertencia la primera vez).

## Tu primera sesión

1. **Abre un proyecto**: el selector de arriba a la izquierda → _Abrir carpeta…_ (o `Ctrl+K` → _Abrir carpeta_).
2. **Revisa los perfiles** en _Ajustes → Perfiles de CLI_. Cada perfil muestra si su comando está instalado (✓) y si
   faltan variables de entorno. Para usar otros proveedores, mira [Proveedores y CLI](https://github.com/ModusZero/mod0/blob/main/docs/guide/providers.md).
3. **Crea una sesión** con `Ctrl+Shift+N`: ponle nombre, elige el perfil y, si quieres aislarla, marca _Worktree propio_.
   Puedes escribir un prompt inicial (se aplana a una línea).
4. **Mira Mission Control**: la sesión aparece como tarjeta y en el panel de atención de la derecha. Con Claude Code
   verás el estado exacto, el archivo que toca y su thinking graph; con otras CLI, el estado se infiere por actividad.
5. **Revisa lo que hizo** en la pestaña _Revisión_ y haz commit desde ahí.

## Atajos que conviene saber

| Atajo               | Acción                             |
| ------------------- | ---------------------------------- |
| `Ctrl+Shift+N`      | Nueva sesión                       |
| `Alt+1…9`           | Saltar a la sesión _n_             |
| `Ctrl+Shift+A`      | Siguiente sesión que te necesita   |
| `Ctrl+K` / `Ctrl+P` | Paleta de comandos / abrir archivo |
| `Ctrl+1…6`          | Cambiar de vista                   |
| ``Ctrl+` ``         | Mostrar u ocultar la terminal      |

Dentro de la terminal solo se interceptan los atajos con `Ctrl+Shift` o `Alt+número`; el resto llega a la CLI.
