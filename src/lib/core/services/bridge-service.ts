import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface RuntimeCapability {
    id: string;
    capability: 'Lsp' | 'Mcp';
    name: string;
    command: string;
}

export interface ProviderConfig {
    id: string;
    type: 'Ai' | 'Vcs';
    provider: string; // e.g., 'anthropic', 'github'
    token: string;
    settings: Record<string, any>;
}

const BRIDGE_COMMANDS = {
    BOOT_RUNTIME: "boot_runtime",
    REGISTER_PROVIDER: "register_provider",

}

class BridgeService {
    private activeRuntimes = new Set<string>();

    /**
     * AUTO: Para LSPs y MCPs detectados en el código
     */
    async discoverAndBoot(projectPath: string) {
        const suggestions = await invoke<RuntimeCapability[]>('discover_workspace', { projectPath });
        for (const def of suggestions) {
            if (!this.activeRuntimes.has(def.id)) {
                await invoke(BRIDGE_COMMANDS.BOOT_RUNTIME, { def });
                this.activeRuntimes.add(def.id);
            }
        }
    }

    /**
     * MANUAL: Para AI y Git configurados por el usuario
     */
    async registerProvider(config: ProviderConfig) {
        await invoke(BRIDGE_COMMANDS.REGISTER_PROVIDER, { config });
        console.log(`Provider ${config.type} listo: ${config.provider}`);
    }

    async subscribe(runtimeId: string, callback: (payload: any) => void) {
        return await listen(`runtime:${runtimeId}`, (event) => callback(event.payload));
    }
}

export const bridgeService = new BridgeService();