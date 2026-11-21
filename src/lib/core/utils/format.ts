/** Último segmento de una ruta (Windows o POSIX). */
export function basename(path: string): string {
    const parts = path.split(/[\\/]/).filter(Boolean);
    return parts[parts.length - 1] ?? path;
}

/** Ruta relativa a `root` con separadores `/` (si no cuelga de `root`, devuelve la original). */
export function relativeTo(root: string, path: string): string {
    const normalizedRoot = root.replace(/\\/g, '/').replace(/\/$/, '');
    const normalized = path.replace(/\\/g, '/');
    return normalized.startsWith(`${normalizedRoot}/`)
        ? normalized.slice(normalizedRoot.length + 1)
        : normalized;
}

/** Une una ruta base con una relativa respetando el separador de la base. */
export function joinPath(base: string, relative: string): string {
    const separator = base.includes('\\') && !base.includes('/') ? '\\' : '/';
    return `${base.replace(/[\\/]$/, '')}${separator}${relative.replace(/[\\/]/g, separator)}`;
}

/** Duración corta: `45s`, `3m`, `2h 10m`. */
export function formatDuration(ms: number): string {
    const seconds = Math.max(0, Math.floor(ms / 1000));
    if (seconds < 60) return `${seconds}s`;

    const minutes = Math.floor(seconds / 60);
    if (minutes < 60) return `${minutes}m`;

    const hours = Math.floor(minutes / 60);
    return `${hours}h ${minutes % 60}m`;
}

/** Número compacto: `1.2k`, `140k`, `1.1M`. */
export function formatCompact(value: number): string {
    if (value < 1000) return String(value);
    if (value < 100_000) return `${(value / 1000).toFixed(1).replace(/\.0$/, '')}k`;
    if (value < 1_000_000) return `${Math.round(value / 1000)}k`;
    return `${(value / 1_000_000).toFixed(1).replace(/\.0$/, '')}M`;
}

/** Puntaje de coincidencia difusa (subsecuencia). `null` si no coincide. */
export function fuzzyScore(query: string, text: string): number | null {
    const needle = query.toLowerCase();
    const haystack = text.toLowerCase();
    if (!needle) return 0;

    let score = 0;
    let cursor = 0;
    let streak = 0;

    for (const char of needle) {
        const found = haystack.indexOf(char, cursor);
        if (found === -1) return null;

        streak = found === cursor ? streak + 1 : 0;
        score += 1 + streak * 2 - (found - cursor) * 0.1;
        // Premia coincidir al inicio de una palabra o del nombre de archivo.
        if (found === 0 || /[\\/_\-. ]/.test(haystack[found - 1])) score += 3;
        cursor = found + 1;
    }
    return score - haystack.length * 0.01;
}

/** Une clases condicionales ignorando valores falsos. */
export function cn(...parts: Array<string | false | null | undefined>): string {
    return parts.filter(Boolean).join(' ');
}
