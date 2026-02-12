/* Este rune gestiona el estado de los paneles de la UI (configuración, terminal, sidebar) */
class ToggleUIStack {
    /** * @type {boolean} Estado de expansión del panel de configuración
     */
    settingsOpen: boolean = $state(false);

    /** * @type {boolean} Estado de expansión del panel de terminal
     */
    terminalOpen: boolean = $state(true);
    
    /** * @type {boolean} Estado de expansión del panel lateral
     */
    sidebarOpen: boolean = $state(true);

    /* Métodos para alternar cada panel */
    toggleSettings(): void { this.settingsOpen = !this.settingsOpen; }
    toggleTerminal(): void { this.terminalOpen = !this.terminalOpen; }
    toggleSidebar(): void { this.sidebarOpen = !this.sidebarOpen; }
}

/* Instancia única del estado de UI para toda la aplicación. */
export const toggleUIStack = new ToggleUIStack();