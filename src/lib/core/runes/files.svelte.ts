import { configService } from '#lib/core/services/config-service';
import { eventService } from '#lib/core/services/event-service';
import { fsService } from '#lib/core/services/fs-service';
import { reviewService } from '#lib/core/services/review-service';
import type { DepGraph, DirEntryInfo, GitInfo } from '#lib/core/types/files';
import { settingsStack } from '#lib/core/runes/settings.svelte';

/** Proyecto abierto, árbol de archivos (carga perezosa) y grafo de dependencias. */
class FilesStack {
    projectPath = $state<string | null>(null);
    /** Raíz que se está explorando: el proyecto o el worktree de una sesión. */
    rootPath = $state<string | null>(null);
    git = $state<GitInfo | null>(null);

    children = $state<Record<string, DirEntryInfo[]>>({});
    expanded = $state<Record<string, boolean>>({});
    index = $state<string[]>([]);

    graph = $state<DepGraph | null>(null);
    /** Carpetas abiertas en el grafo (id = ruta relativa). Vacío = solo el primer nivel. */
    graphExpanded = $state<string[]>([]);
    graphLoading = $state(false);
    graphError = $state<string | null>(null);
    /** Archivos (rutas relativas) que dependen de los cambios de la sesión enfocada. */
    impact = $state<string[]>([]);
    impactOn = $state(false);
    private graphInitialized = false;

    private refreshTimer: ReturnType<typeof setTimeout> | undefined;

    async init(): Promise<() => void> {
        const last = settingsStack.config.lastProjectPath;
        if (last) await this.openProject(last).catch(() => undefined);

        return eventService.onFsChanged((cwd) => this.onFsChanged(cwd));
    }

    async openProject(path: string) {
        await configService.setProject(path).then((config) => (settingsStack.config = config));
        this.projectPath = path;
        await this.setRoot(path);
    }

    async setRoot(path: string) {
        this.rootPath = path;
        this.children = {};
        this.expanded = { [path]: true };
        this.index = [];
        this.graph = null;
        this.graphError = null;
        this.graphExpanded = [];
        this.graphInitialized = false;
        this.impact = [];

        this.git = await reviewService.gitInfo(path).catch(() => null);
        await this.loadDir(path);
    }

    async loadDir(path: string) {
        this.children[path] = await fsService.listDir(path);
    }

    async toggle(path: string) {
        const open = !this.expanded[path];
        this.expanded[path] = open;
        if (open && !this.children[path]) await this.loadDir(path).catch(() => (this.expanded[path] = false));
    }

    /** Recarga todas las carpetas expandidas (tras actividad de las sesiones). */
    async refresh() {
        const open = Object.keys(this.expanded).filter((path) => this.expanded[path]);
        await Promise.all(open.map((path) => this.loadDir(path).catch(() => delete this.children[path])));
        this.index = [];
    }

    async ensureIndex(): Promise<string[]> {
        if (this.index.length === 0 && this.rootPath) this.index = await fsService.listFiles(this.rootPath);
        return this.index;
    }

    async loadGraph(refresh = false) {
        if (!this.rootPath) return;
        this.graphLoading = true;
        this.graphError = null;
        try {
            this.graph = await fsService.graph(this.rootPath, $state.snapshot(this.graphExpanded), refresh);

            // Primera carga de una raíz: abre los dos primeros niveles para que el mapa sea útil de entrada.
            if (!this.graphInitialized) {
                this.graphInitialized = true;
                if (this.graph.dirs.length <= 400) {
                    this.graphExpanded = this.dirsBelow(2);
                    this.graph = await fsService.graph(
                        this.rootPath,
                        $state.snapshot(this.graphExpanded),
                        false,
                    );
                }
            }
        } catch (error) {
            this.graphError = error instanceof Error ? error.message : String(error);
        } finally {
            this.graphLoading = false;
        }
    }

    /** Carpetas con profundidad menor a `level` (1 = solo las de primer nivel). */
    dirsBelow(level: number): string[] {
        return (this.graph?.dirs ?? []).filter((dir) => dir.split('/').length < level);
    }

    async setGraphLevel(level: number) {
        this.graphExpanded = this.dirsBelow(level);
        await this.loadGraph();
    }

    async toggleExpand(id: string) {
        const open = this.graphExpanded.includes(id);
        this.graphExpanded = open
            ? this.graphExpanded.filter((dir) => dir !== id && !dir.startsWith(`${id}/`))
            : [...this.graphExpanded, id];
        await this.loadGraph();
    }

    /** Calcula qué archivos se ven afectados por los cambios dados (dependientes directos e indirectos). */
    async loadImpact(changed: string[]) {
        if (!this.rootPath || changed.length === 0) {
            this.impact = [];
            return;
        }
        this.impact = await fsService.impact(this.rootPath, changed).catch(() => []);
    }

    /** Las sesiones avisan cuando hay cambios en disco; se agrupan en un solo refresco. */
    private onFsChanged(cwd: string) {
        const root = this.rootPath?.replace(/\\/g, '/');
        if (!root || !cwd.replace(/\\/g, '/').startsWith(root)) return;

        clearTimeout(this.refreshTimer);
        this.refreshTimer = setTimeout(() => void this.refresh(), 600);
    }
}

export const filesStack = new FilesStack();
