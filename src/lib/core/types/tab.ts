import type { ActivityID } from "./work";

/** Estructura base para cualquier elemento que pueda ocupar una pestaña */
export interface BaseTab {
    id: string;    // Ruta del archivo o ID único de actividad
    label: string; // Texto a mostrar en la pestaña
    icon: any;     // Componente de icono (Lucide)
    color: string; // Clase de color para la UI
}

/**
 * Define la estructura de un archivo cualquiera.
 */
export interface FileNode extends BaseTab{
    path: string;
    name: string;
    is_dir: boolean;
    extension: string;
}

/**
 * Define la estructura de un archivo de código.
 */
export interface CodeTab extends BaseTab, FileNode {
    content: string;
    language: 'rust' | 'javascript' | 'typescript' | 'markdown' | 'json';
    isDirty: boolean;
    type: 'code';
}

/** Pestañas de herramientas internas (Blueprint, Forge, Pulse) */
export interface ActivityTab extends BaseTab {
    type: ActivityID;
}

/** Pestañas para archivos binarios o no editables (Imágenes, PDFs) */
export interface AssetTab extends BaseTab, FileNode {
    type: 'asset';
}

/** Pestañas genéricas para extensiones */
export interface GenericTab extends BaseTab {
    type: 'other';
}

/** Unión discriminada para el control de flujo en el Workbench */
export type Tab = ActivityTab | CodeTab | AssetTab | GenericTab;

export type GenericFile = (AssetTab | CodeTab) & { children?: GenericFile[]; }