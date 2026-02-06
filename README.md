# `Mod0` | The Architect’s Interface

<table width="100%" border="0">
  <tr>
    <td>
      <h3>[ES]</h3>
      <p>
        Mod0 es un IDE Autónomo, Agnóstico y Agéntico diseñado para el paradigma <strong>Mind-to-Action</strong>. Desarrollado por <strong>Modus Zero</strong>, permite liderar enjambres de IA con precisión quirúrgica, eliminando la fricción entre la intención y la ejecución.
      </p>
      <hr size="1" style="border:none; border-top:1px solid #30363d;" />
      <h3>[EN]</h3>
      <p>
        Mod0 is an Autonomic, Agnostic, and Agentic IDE built for the <strong>Mind-to-Action</strong> paradigm. Developed by <strong>Modus Zero</strong>, it enables developers to lead AI swarms with surgical precision, removing friction between intent and execution.
      </p>
    </td>
    <td align="right" width="250">
      <picture>
        <source media="(prefers-color-scheme: dark)" alt="logo-dark" srcset="https://github.com/user-attachments/assets/c2a5b10c-2cab-4b0c-b3aa-964726381a3b">
        <source media="(prefers-color-scheme: light)" alt="logo" srcset="https://github.com/user-attachments/assets/08ee9e0c-f334-46e3-971b-bf82b6e67e54">
        <img alt="logo-unified" src="https://github.com/user-attachments/assets/3f7a3e88-7dbb-4527-a9f9-b05c37361573" width="250"/>
      </picture>
    </td>
  </tr>
</table>

## 🌐 Index / Índice

* **[Español (Especificaciones y Features)](#description-es)**
* **[English (Specifications & Features)](#description-en)**
* **[Manual (Instalación)](#bilingual-manual)**

---

<br id="description-es">

# 🇪🇸 Versión en Español

## Tecnologías

| Componente | Tecnología | Propósito |
| --- | --- | --- |
| **Logic Core** | **Rust** | Orquestación de grafos, indexación SQLite y bridge con el SO. |
| **Desktop Shell** | **Tauri v2** | Seguridad nativa, alto rendimiento y footprint mínimo (<20MB). |
| **UI Framework** | **Svelte 5 (Runes)** | Interfaz reactiva de ultra-baja latencia para visualización de datos. |
| **Protocolo** | **Native MCP** | Integración nativa con Model Context Protocol sin intermediarios. |
| **Protocolo** | **Native LSP** | Múltiples intérpretes de lenguajes y tecnologías sin necesidad de plugins externos. |

---

## Principales Características

### Mind-to-Action UI (Blueprint Mode)
* **Comunicación Usuario-IA:** A diferencia de un chat, no es para "leer" las respuestas de la IA (nadie las lee realmente de todas formas), escribe prompts y obtén respuestas simples y eficientes usando artefactos (dibujos simples, diagramas, tablas, etc) para transmitir la información sin un solo scroll y en un parrafo como máximo (los devs entendemos mejor 10 diagramas que un párrafo). Esta es accesible y completamente conectada con cualquier seccion del IDE.
* **Planeación & Artifacts:** Espacio dedicado a diagramas ERD, de arquitectura o flujo (dependiendo de la tarea) o incluso un canvas de prototipos UI, todo antes de tocar una sola línea de código, completamente generado por IA pero igualmente customizable y traducible a código.
* **Grafo de pensamiento & Tablero Kanban:** Visualiza el razonamiento, acciones y decisiones del agente en tiempo real (permite tambien controlar decisiones en caso de bifurcaciones o posibles opciones, tú sigues teniendo el control siempre que lo decidas) y visualiza/gestiona tareas en un tablero kanban.

### Ingeniería de Control (Forge Mode)
* **Ghost Runner Sandboxing:** Uso de worktrees/branches junto con un sandbox agresivo en para validar lógica y tests (TDD) antes de la implementación real.
* **Sesiones de Agentes:** Worktrees/Branches gestionadas por agentes para multitasking masivo sin conflictos en el workspace, el límite es tu capacidad de control.
* **Inferencia-por-Dificultad:** Estrategia de asignación automática de modelos basada en dificultad de tasks para eficiencia de tokens.

### Codebase Management 2.0 (Pulse Mode)
* **Vista Híbrida:** Alterna entre Explorador de Archivos tradicional y Árbol de Archivos Visual usando grafos de dependencias.
* **Control de Versiones Visual:** Incluido en el File Tree Visual, gestiona commits y ramas visualmente; entiende el impacto de cada cambio en lugar de leer Pull Requests kilométricas (también con una sección apartada para Control de Versiones con la UI tradicional).
* **Orientado a Local / BYOK:** Conexión personalizada con modelos cloud (BYOK) y self-host. Soporte total para modelos locales, trabajo offline y privacidad absoluta.

### Contexto Agéntico
* **Biblioteca de Skills Integrada :** Deja de instalar o gestionar skills manualmente en tu proyecto, conecta tu repositorio de skills a la base de datos, aunque sean miles de skills la IA los usará eficientemente.
* **Gestión de Contexto:** Gestiona las skills activas (si quieres optimizar y controlar aún más y minimizar posibles alucinaciones) puedes activar o desactivar las que usas, como si fueran extensiones o plugins, no más skills como deps!
* **Aprendizaje Reforzado Nativo:** Capacidad nativa de utilizar el TDD para aprender de casos de uso anteriores sin necesidad de configuraciones extra, todo en un entorno controlado (tú decides si guarda el conocimiento en tu pc para todos tus proyectos o si es solo por repositorio para total privacidad).

> [!TIP]
> ### Aprovecha de la mejor forma el entorno sin cambiar drásticamente tu flujo de trabajo
> * **Editor de código tradicional:** Cómo olvidar esto, la IA hará la mayoría pero siempre es bueno poder revisar o editar cambios pequeños, la idea no es que sea la forma principal de trabajo, pero no puede faltar de ninguna forma.
> * **Flujos de trabajo personalizables:** Parecido a flujos de CI/CD, permite controlar la forma en que trabaja la IA, si usar o no generación de artefactos, si prefieres multi-tasking y/o control total, grado de libertad concedido a la IA para decisiones técnicas, si prefieres que la IA haga el control de versiones o avise de vez en cuando para que revises y subas cambios. Dependiendo de tu rol en el proyecto y dependiendo de tu forma de trabajar todo cambia, tú defines tu estilo.
> * **Plugins o extensiones:** Soporte para extensiones basadas en WASM usando Rust, Go, JavaScript o C/C++.

---

<br id="description-en">

# 🇺🇸 English Version

## Technologies

| Component | Technology | Purpose |
| --- | --- | --- |
| **Logic Core** | **Rust** | Graph orchestration, SQLite indexing, and OS bridge. |
| **Desktop Shell** | **Tauri v2** | Native security, high performance, and minimal footprint (<20MB). |
| **UI Framework** | **Svelte 5 (Runes)** | Ultra-low latency reactive interface for data visualization. |
| **Protocol** | **Native MCP** | Native Model Context Protocol integration without intermediaries. |
| **Protocol** | **Native LSP** | Multiple language and technology interpreters without the need for external plugins. |

---

## Key Features

### Mind-to-Action UI (Blueprint Mode)
* **User-AI Communication:** Unlike a chat, this is not for "reading" AI responses (no one really reads them anyway); write prompts and get simple, efficient responses using artifacts (simple drawings, diagrams, tables, etc.) to convey information without a single scroll and in one paragraph at most (devs understand 10 diagrams better than a paragraph). This is accessible and fully connected to any section of the IDE.
* **Planning & Artifacts:** Dedicated space for ERD, architecture, or flow diagrams (depending on the task) or even a UI prototype canvas—all before touching a single line of code. Fully AI-generated but equally customizable and translatable to code.
* **Thought Graph & Kanban Board:** Visualize agent reasoning, actions, and decisions in real-time (also allows for decision control in case of bifurcations or potential options; you remain in control whenever you choose) and visualize/manage tasks on a kanban board.

### Control Engineering (Forge Mode)
* **Ghost Runner Sandboxing:** Use of worktrees/branches along with an aggressive sandbox to validate logic and tests (TDD) before actual implementation.
* **Agent Sessions:** Agent-managed worktrees/branches for massive multitasking without workspace conflicts; the limit is your control capacity.
* **Inference-by-Difficulty:** Automated model assignment strategy based on task difficulty for token efficiency.

### Codebase Management 2.0 (Pulse Mode)
* **Hybrid View:** Switch between a traditional File Explorer and a Visual File Tree using dependency graphs.
* **Visual Source Control:** Included in the Visual File Tree; manage commits and branches visually. Understand the impact of each change instead of reading kilometer-long Pull Requests (also features a separate section for Version Control with a traditional UI).
* **Local-First / BYOK:** Custom connection with cloud models (BYOK) and self-hosting. Full support for local models, offline work, and absolute privacy.

### Agentic Context
* **Integrated Skills Library:** Stop installing or managing skills manually in your project; connect your skills repository to the database. Even with thousands of skills, the AI will use them efficiently.
* **Context Management:** Manage active skills (to further optimize, control, and minimize potential hallucinations); you can activate or deactivate the ones you use, as if they were extensions or plugins—no more skills as deps!
* **Native Reinforcement Learning:** Native ability to use TDD to learn from previous use cases without extra configurations, all within a controlled environment (you decide if knowledge is saved on your PC for all projects or per repository for total privacy).

> [!TIP]
> ### Use the best of the environment without drastically changing your workflow
> * **Traditional Code Editor:** Can't forget this—the AI will do most of the work, but it's always good to be able to review or edit small changes. It’s not intended as the primary way of working, but it’s absolutely essential.
> * **Customizable Workflows:** Similar to CI/CD flows, this allows you to control how the AI works—whether or not to use artifact generation, if you prefer multi-tasking and/or total control, the degree of freedom granted to the AI for technical decisions, and whether the AI handles version control or prompts you to review and push changes. Everything changes depending on your role and your way of working; you define your style.
> * **Plugins or Extensions:** Support for WASM-based extensions using Rust, Go, JavaScript, or C/C++.

---

<br id="bilingual-manual">

## 📖 Manual

```bash
# 1. Instalar dependencias | Install deps
npm install

# 2. Iniciar entorno de desarrollo (Tauri + Svelte) | Start Dev
npm run tauri dev

# 3. Construir ejecutable nativo | Build native app
npm run tauri build
```

<p align="center"> <sub>Autónomo|Autonomic • Agnóstico|Agnostic • Agéntico|Agentic • 2026 • <strong>Mod0 por|by Modus Zero</strong></sub> </p>
