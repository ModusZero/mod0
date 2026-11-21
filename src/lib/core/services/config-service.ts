import { call } from '#lib/core/services/ipc';
import type { AppConfig, AppInfo, CommandCheck } from '#lib/core/types/settings';

export const configService = {
    get: () => call<AppConfig>('config_get'),
    update: (config: AppConfig) => call<AppConfig>('config_update', { config }),
    setProject: (path: string) => call<AppConfig>('config_set_project', { path }),
    appInfo: () => call<AppInfo>('app_info'),
    notifyTest: () => call<void>('notify_test'),
    profileCheck: (command: string, env: Record<string, string>) =>
        call<CommandCheck>('profile_check', { command, env }),
};
