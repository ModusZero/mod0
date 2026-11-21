import type { Component } from 'svelte';

/** Acción disponible en la paleta de comandos. */
export interface PaletteItem {
    id: string;
    label: string;
    hint?: string;
    icon?: Component<any>;
    /** Texto adicional usado solo para filtrar. */
    keywords?: string;
    run: () => void | Promise<void>;
}
