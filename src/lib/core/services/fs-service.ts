import { call } from '#lib/core/services/ipc';
import type { DepGraph, DirEntryInfo } from '#lib/core/types/files';

export const fsService = {
    listDir: (path: string) => call<DirEntryInfo[]>('fs_list_dir', { path }),
    listFiles: (root: string, limit?: number) => call<string[]>('fs_list_files', { root, limit }),
    read: (path: string) => call<string>('fs_read', { path }),
    write: (path: string, content: string) => call<void>('fs_write', { path, content }),
    createFile: (path: string) => call<void>('fs_create_file', { path }),
    createDir: (path: string) => call<void>('fs_create_dir', { path }),
    rename: (from: string, to: string) => call<void>('fs_rename', { from, to }),
    remove: (path: string) => call<void>('fs_delete', { path }),
    graph: (root: string, expanded: string[], refresh = false) =>
        call<DepGraph>('fs_graph', { root, expanded, refresh }),
    impact: (root: string, paths: string[]) => call<string[]>('fs_impact', { root, paths }),
};
