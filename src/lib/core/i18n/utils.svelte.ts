import en from '#lib/core/i18n/locales/en.json';
import es from '#lib/core/i18n/locales/es.json';
import type { Language } from '#lib/core/types/settings';

export type TranslationKey = keyof typeof es;

const dictionaries: Record<Language, Record<string, string>> = { es, en };

/** Idioma activo (lo actualiza `settingsStack`). Al ser reactivo, `t()` re-renderiza al cambiarlo. */
export const i18n = $state<{ language: Language }>({ language: 'es' });

/**
 * Traduce una clave. Admite parámetros `{nombre}`. Si falta en el idioma activo usa español
 * y, como último recurso, la propia clave.
 */
export function t(key: TranslationKey | (string & {}), params?: Record<string, string | number>): string {
    const template = dictionaries[i18n.language][key] ?? dictionaries.es[key] ?? key;
    return params
        ? template.replace(/\{(\w+)\}/g, (_, name) => String(params[name] ?? `{${name}}`))
        : template;
}
