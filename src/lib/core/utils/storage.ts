/** Lectura/escritura segura de preferencias de interfaz en `localStorage`. */
export function loadLocal<T>(key: string, fallback: T): T {
    try {
        const raw = localStorage.getItem(`mod0.${key}`);
        return raw === null ? fallback : (JSON.parse(raw) as T);
    } catch {
        return fallback;
    }
}

export function saveLocal(key: string, value: unknown): void {
    try {
        localStorage.setItem(`mod0.${key}`, JSON.stringify(value));
    } catch {
        // Sin almacenamiento disponible: la preferencia simplemente no se recuerda.
    }
}
