import { invoke } from "@tauri-apps/api/core";
import type { AppConfig } from "$lib/types/config";
import { TAURI_COMMANDS } from "$lib/constants/tauri-commands";

class ConfigStack {
    current = $state<AppConfig>({
        theme: "light",
        language: "es",
        last_project_path: null
    });

    constructor() {
        this.init();
    }

    async init() {
        try {
            const savedConfig = await invoke<AppConfig>(TAURI_COMMANDS.GET_CONFIG);
            this.current = savedConfig;
            this.applyTheme();
            this.applyLanguage();
        } catch (e) {
            console.warn("Modo Web: Usando configuración por defecto.");
            this.applyTheme();
        }
    }

    async update(newValues: Partial<AppConfig>) {
        const keys = Object.keys(newValues) as Array<keyof AppConfig>;
        
        keys.forEach(key => {
            (this.current as any)[key] = newValues[key];
        });

        if (keys.includes('theme')) this.applyTheme();
        if (keys.includes('language')) this.applyLanguage();

        try {
            await invoke(TAURI_COMMANDS.UPDATE_CONFIG, { newConfig: this.current });
        } catch (e) {
            console.error("Error al guardar:", e);
        }
    }

    private applyTheme() {
        if (typeof document === 'undefined') return;
        const root = document.documentElement;
        root.classList.toggle('dark', this.current.theme === 'dark');
        root.style.colorScheme = this.current.theme;
    }

    private applyLanguage() {
        if (typeof document === 'undefined') return;
        document.documentElement.lang = this.current.language;
    }
}

export const configStack = new ConfigStack();