/** Archivo abierto en el editor. */
export interface EditorTab {
    path: string;
    name: string;
    content: string;
    /** Contenido guardado en disco (para detectar cambios sin guardar). */
    saved: string;
    error: string | null;
}
