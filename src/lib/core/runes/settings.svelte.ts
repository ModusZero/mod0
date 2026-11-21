import { defaultConfig } from '#lib/core/constants/default-config';
import { i18n } from '#lib/core/i18n/utils.svelte';
import { configService } from '#lib/core/services/config-service';
import type { AppConfig, AppInfo } from '#lib/core/types/settings';

/** Configuración global (persistida en el backend) y su aplicación visual. */
class SettingsStack {
    config = $state<AppConfig>(structuredClone(defaultConfig));
    info = $state<AppInfo | null>(null);
    loaded = $state(false);
    /** Sección visible en la pantalla de ajustes. */
    section = $state<'profiles' | 'notifications' | 'appearance' | 'about'>('profiles');

    private media: MediaQueryList | null = null;

    dark = $derived(
        this.config.theme === 'dark' || (this.config.theme === 'system' && (this.media?.matches ?? true)),
    );

    async load() {
        const [config, info] = await Promise.all([configService.get(), configService.appInfo()]);
        this.config = config;
        this.info = info;
        this.loaded = true;
        this.apply();

        this.media = window.matchMedia('(prefers-color-scheme: dark)');
        this.media.addEventListener('change', () => this.apply());
    }

    async save(patch: Partial<AppConfig>) {
        this.config = await configService.update({ ...$state.snapshot(this.config), ...patch });
        this.apply();
    }

    /** Aplica tema, acento, escala y movimiento al documento. */
    apply() {
        const { theme, accent, uiScale, reduceMotion, language } = this.config;
        const dark =
            theme === 'dark' ||
            (theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);
        const root = document.documentElement;

        root.classList.toggle('dark', dark);
        root.classList.toggle('reduce-motion', reduceMotion);
        root.dataset.accent = accent;
        root.style.setProperty('--ui-scale', String(uiScale));
        root.lang = language;
        i18n.language = language;
    }

    zoom(delta: number) {
        const next = Math.min(1.4, Math.max(0.8, Math.round((this.config.uiScale + delta) * 100) / 100));
        void this.save({ uiScale: next });
    }
}

export const settingsStack = new SettingsStack();
