import { invoke } from "@tauri-apps/api/core";
import { TAURI_COMMANDS } from "$lib/constants";

export interface AppConfig {
    theme: string;
    language: string;
    last_project_path: string | null;
}

class ConfigStack {
    current = $state<AppConfig>({
        theme: "dark",
        language: "es",
        last_project_path: null
    });

    constructor() {
        this.init();
    }

    async init() {
        try {
            // Uso de constante: autocompletado y seguridad
            const savedConfig = await invoke<AppConfig>(TAURI_COMMANDS.GET_CONFIG);
            this.current = savedConfig;
            this.applyTheme();
        } catch (e) {
            console.warn("Modo Web: Usando configuración por defecto.");
        }
    }

    async update(newValues: Partial<AppConfig>) {
        this.current = { ...this.current, ...newValues };
        this.applyTheme();
        try {
            // Uso de constante para actualizar
            await invoke(TAURI_COMMANDS.UPDATE_CONFIG, { newConfig: this.current });
        } catch (e) {
            console.error("No se pudo guardar la configuración.");
        }
    }

    private applyTheme() {
        if (typeof document !== 'undefined') {
            document.body.className = this.current.theme;
        }
    }
}

export const configStack = new ConfigStack();