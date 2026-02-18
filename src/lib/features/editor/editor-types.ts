import type { CodeExtension } from "$lib/core/types/tab";

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

export type EditorLanguage = 
    'rust' 
    | 'typescript' 
    | 'javascript' 
    | 'svelte' 
    | 'json' 
    | 'markdown' 
    | 'astro'
    | 'ruby'
    | 'c++'
    | 'java'
    | 'html'
    | 'css'
    | 'yaml'
    | 'txt'
    | 'toml'
    | 'plaintext';

export const LANGUAGE_MAP: Record<CodeExtension, EditorLanguage> = {
    'rs': 'rust',
    'ts': 'typescript',
    'js': 'javascript',
    'svelte': 'svelte',
    'json': 'json',
    'md': 'markdown',
    'astro': 'astro',
    'rb': 'ruby',
    'cpp': 'c++',
    'hpp': 'c++',
    'java': 'java',
    'html': 'html',
    'css': 'css',
    'yaml': 'yaml',
    'txt': 'txt',
    'toml': 'toml',
    'other': 'plaintext'
};