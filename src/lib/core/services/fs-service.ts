import { invoke } from "@tauri-apps/api/core";
import type { GenericFile } from "../types/tab";

const FILE_COMMANDS = {
    READ_FOLDER: "read_folder_recursive",
    READ_FILE: "read_file_content",
    SAVE_FILE: "save_file",
    CREATE_FILE: "create_file",
    CREATE_DIR: "create_dir",
    RENAME_FILE: "rename_file",
    DELETE_FILE: "delete_file",
    SEARCH_CONTENT: "search_in_files"
}

export const FSService = {
    async readFolder(path: string): Promise<GenericFile[]> {
        return await invoke(FILE_COMMANDS.READ_FOLDER, { path });
    },
    async readFile(path: string): Promise<string> {
        return await invoke(FILE_COMMANDS.READ_FILE, { path });
    },
    async saveFile(path: string, content: string): Promise<void> {
        await invoke(FILE_COMMANDS.SAVE_FILE, { path, content });
    },
    async createFile(path: string): Promise<void>  { 
        return await invoke(FILE_COMMANDS.CREATE_FILE, { path }); 
    },
    async createDir(path: string): Promise<void>  { 
        return await invoke(FILE_COMMANDS.CREATE_DIR, { path }); 
    },
    async deleteFile(path: string): Promise<void>  { 
        return await invoke(FILE_COMMANDS.DELETE_FILE, { path }); 
    },
    async renameFile(oldPath: string, newPath: string): Promise<void>  { 
        return await invoke(FILE_COMMANDS.RENAME_FILE, { oldPath, newPath }); 
    },
    async searchContent(root: string, query: string): Promise<string[]> {
        return await invoke(FILE_COMMANDS.SEARCH_CONTENT, { root, query });
    }
};