import { invoke } from "@tauri-apps/api/core";
import type { Settings } from "$lib/features/settings/settings-types";

export const CONFIG_COMMANDS = {
    GET_CONFIG: "get_config",
    UPDATE_CONFIG: "update_config",
}

export const SettingsService = {
    async getConfig(): Promise<Settings> {
        return await invoke(CONFIG_COMMANDS.GET_CONFIG);
    },
    async updateConfig(newConfig: Settings): Promise<void> {
        invoke(CONFIG_COMMANDS.UPDATE_CONFIG, { newConfig });
    },
};