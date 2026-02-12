/**
 * Configuración de la selección actual del editor.
 */
export interface EditorSelection {
    from: number;
    to: number;
    line: number;
    column: number;
}

/**
 * Ajustes visuales del componente de edición.
 */
export interface EditorSettings {
    fontSize: number;
    tabSize: number;
    lineNumbers: boolean;
    wordWrap: boolean;
    minimap: boolean;
}