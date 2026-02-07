/**
 * @fileoverview Tipos fundamentales para el sistema de navegación y modos de Mod0.
 */

/** Modos principales de operación del IDE */
export type IDEMode = 'BLUEPRINT' | 'FORGE' | 'PULSE';

/** IDs de actividades disponibles en el ActivityBar */
export type ActivityID = 'explorer' | 'search' | 'git' | 'architecture' | 'forge-tools' | 'metrics';

/** Estructura de una actividad vinculada a un modo */
export interface Activity {
    id: ActivityID;
    icon: string;
    label: string;
    mode: IDEMode;
}