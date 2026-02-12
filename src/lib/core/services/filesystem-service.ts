import { invoke } from "@tauri-apps/api/core";
import { TAURI_COMMANDS } from "../constants/tauri-commands";
import type { GenericFile } from "../types/tab";

export const FSService = {
    async readFolder(path: string): Promise<GenericFile[]> {
        return await invoke(TAURI_COMMANDS.READ_FOLDER, { path });
    },
    async readFile(path: string): Promise<string> {
        return await invoke(TAURI_COMMANDS.READ_FILE, { path });
    },
    async saveFile(path: string, content: string): Promise<void> {
        await invoke(TAURI_COMMANDS.SAVE_FILE, { path, content });
    }
};