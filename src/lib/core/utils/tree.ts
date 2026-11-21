import type { ChangedFile } from '#lib/core/types/files';

export interface ChangeTreeNode {
    /** Ruta completa de la carpeta (o del archivo). */
    path: string;
    /** Nombre mostrado; las carpetas con un solo hijo carpeta se fusionan (`src/auth`). */
    name: string;
    file: ChangedFile | null;
    children: ChangeTreeNode[];
    /** Cantidad de archivos cambiados dentro (solo carpetas). */
    count: number;
}

/** Agrupa los archivos cambiados por carpeta, como la vista en árbol del control de versiones de VS Code. */
export function buildChangeTree(files: ChangedFile[]): ChangeTreeNode[] {
    const root: ChangeTreeNode = { path: '', name: '', file: null, children: [], count: 0 };

    for (const file of files) {
        let cursor = root;
        const parts = file.path.split('/');

        parts.forEach((part, index) => {
            cursor.count++;
            const path = parts.slice(0, index + 1).join('/');
            let next = cursor.children.find((child) => child.path === path);

            if (!next) {
                next = {
                    path,
                    name: part,
                    file: index === parts.length - 1 ? file : null,
                    children: [],
                    count: 0,
                };
                cursor.children.push(next);
            }
            cursor = next;
        });
        cursor.count = 1;
    }

    const compact = (node: ChangeTreeNode): ChangeTreeNode => {
        node.children = node.children.map(compact);
        // Fusiona cadenas de carpetas con un único hijo carpeta.
        while (!node.file && node.children.length === 1 && !node.children[0].file && node.path !== '') {
            const only = node.children[0];
            node.name = `${node.name}/${only.name}`;
            node.path = only.path;
            node.children = only.children;
        }
        node.children.sort((a, b) => Number(!!a.file) - Number(!!b.file) || a.name.localeCompare(b.name));
        return node;
    };

    return compact(root).children;
}
