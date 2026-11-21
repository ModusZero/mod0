import { call } from '#lib/core/services/ipc';
import type { ChangedFile, ChangeStatus, FilePair, GitInfo } from '#lib/core/types/files';

export const reviewService = {
    gitInfo: (path: string) => call<GitInfo>('git_info', { path }),
    changes: (sessionId: string) => call<ChangedFile[]>('review_changes', { sessionId }),
    file: (sessionId: string, path: string, status: ChangeStatus) =>
        call<FilePair>('review_file', { sessionId, path, status }),
    commit: (sessionId: string, message: string) => call<string>('review_commit', { sessionId, message }),
};
