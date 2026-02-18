import { SettingsService } from '../../core/services/settings-service';
import { fileStack } from '../system-atoms/pulse/files/files-runes.svelte';
import type { Settings } from "./settings-types";

class SettingsStack {
    current = $state<Settings>({
        theme: "dark",
        language: "es",
        last_project_path: null,
        auto_save: false
    });

    constructor() {
        this.init();
    }

    async init(): Promise<void> {
        try {
            const savedConfig = await SettingsService.getConfig();
            this.current = savedConfig;
            
            this.applyTheme();
            this.applyLanguage();

            if (this.current.last_project_path) {
                console.log("Restaurando último workspace:", this.current.last_project_path);
                fileStack.openFolder(this.current.last_project_path);
            }
        } catch (e) {
            console.warn("Modo Web o Error: Usando configuración por defecto.");
            this.applyTheme();
        }
    }

    async update(newValues: Partial<Settings>): Promise<void> {
        // Actualizamos el estado reactivo
        Object.assign(this.current, newValues);

        if (newValues.theme) this.applyTheme();
        if (newValues.language) this.applyLanguage();

        try {
            await SettingsService.updateConfig(this.current);
        } catch (e) {
            console.error("Error al guardar en Rust:", e);
        }
    }

    private applyTheme(): void {
        if (typeof document === 'undefined') return;
        const root = document.documentElement;
        document.body.classList.add('theme-transitioning');
        root.classList.toggle('dark', this.current.theme === 'dark');
        root.style.colorScheme = this.current.theme;
        setTimeout(() => document.body.classList.remove('theme-transitioning'), 150);
    }

    private applyLanguage(): void {
        if (typeof document === 'undefined') return;
        document.documentElement.lang = this.current.language;
    }
}

export const settingsStack = new SettingsStack();