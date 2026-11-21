import { invoke } from '@tauri-apps/api/core';

/** Invoca un comando de Tauri y normaliza el error a un `Error` con mensaje legible. */
export async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
    try {
        return await invoke<T>(command, args);
    } catch (error) {
        throw error instanceof Error ? error : new Error(String(error));
    }
}
