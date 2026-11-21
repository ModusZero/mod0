import type { WorkSectionIDs, ActivityIDs } from '#lib/core/constants/work-config';
import type { Component } from 'svelte';

/** Tipo de sección de trabajo del IDE */
export type WorkSectionID = (typeof WorkSectionIDs)[keyof typeof WorkSectionIDs];

/** Tipo de identificador de actividad */
export type ActivityID = (typeof ActivityIDs)[keyof typeof ActivityIDs];

/** Estructura de un nodo visual ({@link WorkSectionID} o {@link ActivityID}) */
export interface DisplayNode {
    id: string;
    /** Clave de i18n de la etiqueta */
    label: string;
    icon: Component<any>;
    /** Atajo de teclado asociado (solo informativo) */
    shortcut?: string;
}
