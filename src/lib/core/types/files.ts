export interface DirEntryInfo {
    name: string;
    path: string;
    isDir: boolean;
}

export interface GraphNode {
    id: string;
    label: string;
    files: number;
    bytes: number;
    /** `dir`: carpeta agregada (se puede expandir). `file`: archivo individual. */
    kind: 'dir' | 'file';
}

export interface GraphEdge {
    from: string;
    to: string;
    weight: number;
}

/** Grafo de dependencias: las carpetas expandidas se abren en sus hijos, el resto queda agregado. */
export interface DepGraph {
    maxDepth: number;
    fileCount: number;
    /** Todas las carpetas con código analizado (para expandir hasta un nivel). */
    dirs: string[];
    nodes: GraphNode[];
    edges: GraphEdge[];
}

export interface GitInfo {
    isRepo: boolean;
    branch: string | null;
}

/** Estado de un archivo cambiado: M modificado, A añadido, D borrado, U sin seguimiento. */
export type ChangeStatus = 'M' | 'A' | 'D' | 'U';

export interface ChangedFile {
    path: string;
    status: ChangeStatus;
    additions: number;
    deletions: number;
}

export interface FilePair {
    path: string;
    status: ChangeStatus;
    original: string;
    modified: string;
}
