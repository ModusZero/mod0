import { loadLocal, saveLocal } from '#lib/core/utils/storage';

export interface NewSessionPreset {
    name?: string;
    profileId?: string;
    prompt?: string;
    taskId?: string;
}

/** Estado de visibilidad y tamaño de los paneles del IDE. */
class ToggleUIStack {
    sidebarOpen = $state(loadLocal('ui.sidebarOpen', true));
    railOpen = $state(loadLocal('ui.railOpen', true));
    terminalOpen = $state(loadLocal('ui.terminalOpen', true));
    sidebarWidth = $state(loadLocal('ui.sidebarWidth', 288));
    terminalHeight = $state(loadLocal('ui.terminalHeight', 300));

    paletteOpen = $state(false);
    paletteMode = $state<'commands' | 'files'>('commands');
    newSessionOpen = $state(false);
    newSessionPreset = $state<NewSessionPreset>({});

    toggleSidebar = () => this.persist('sidebarOpen', (this.sidebarOpen = !this.sidebarOpen));
    toggleRail = () => this.persist('railOpen', (this.railOpen = !this.railOpen));
    toggleTerminal = () => this.persist('terminalOpen', (this.terminalOpen = !this.terminalOpen));

    setSidebarWidth(width: number) {
        this.sidebarWidth = Math.min(520, Math.max(220, Math.round(width)));
        saveLocal('ui.sidebarWidth', this.sidebarWidth);
    }

    setTerminalHeight(height: number) {
        this.terminalHeight = Math.min(900, Math.max(140, Math.round(height)));
        saveLocal('ui.terminalHeight', this.terminalHeight);
    }

    openPalette(mode: 'commands' | 'files' = 'commands') {
        this.paletteMode = mode;
        this.paletteOpen = true;
    }

    openNewSession(preset: NewSessionPreset = {}) {
        this.newSessionPreset = preset;
        this.newSessionOpen = true;
    }

    private persist(key: string, value: boolean) {
        saveLocal(`ui.${key}`, value);
    }
}

export const toggleUIStack = new ToggleUIStack();
