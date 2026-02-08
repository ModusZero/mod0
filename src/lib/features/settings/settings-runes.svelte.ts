import { invoke } from "@tauri-apps/api/core";
import { TAURI_COMMANDS } from "$lib/core/constants/tauri-commands";
import type { Settings } from "./settings-types";

class SettingsStack {
    current = $state<Settings>({
        theme: "light",
        language: "es",
        last_project_path: null
    });

    constructor() {
        this.init();
    }

    async init() {
        try {
            const savedConfig = await invoke<Settings>(TAURI_COMMANDS.GET_CONFIG);
            this.current = savedConfig;
            this.applyTheme();
            this.applyLanguage();
        } catch (e) {
            console.warn("Modo Web: Usando configuración por defecto.");
            this.applyTheme();
        }
    }

    async update(newValues: Partial<Settings>) {
        const keys = Object.keys(newValues) as Array<keyof Settings>;
        
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
        
        document.body.classList.add('theme-transitioning');

        root.classList.toggle('dark', this.current.theme === 'dark');
        root.style.colorScheme = this.current.theme;

        requestAnimationFrame(() => {
            setTimeout(() => {
                document.body.classList.remove('theme-transitioning');
            }, 1);
        });
    }

    private applyLanguage() {
        if (typeof document === 'undefined') return;
        document.documentElement.lang = this.current.language;
    }
}

export const settingsStack = new SettingsStack();