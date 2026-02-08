import type { WorkSectionIDs } from "$lib/constants/ui-config";
import { ActivityIDs } from '../constants/ui-config';

/** Tipo de modo de trabajo del IDE */
export type WorkSectionID = typeof WorkSectionIDs[keyof typeof WorkSectionIDs];

/** Tipo de identificador de actividad */
export type ActivityID = typeof ActivityIDs[keyof typeof ActivityIDs];

/** Estructura de una nodo desplegable ({@link WorkSectionID} o {@link ActivityID}) */
export interface DisplayNode {
    id: string;
    label: string;
    icon: any;
    color?: string;
    description?: string;
}