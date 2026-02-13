import { invoke } from "@tauri-apps/api/core";

export function createLSPPlugin(languageId: string) {
    // Este pequeño objeto intercepta lo que CodeMirror necesita
    // y lo manda al backend de Rust.
    return languageServer({
        client: {
            // No necesitas WebSockets, usas el Bridge de Tauri
            request: (method, params) => invoke("lsp_request", { language: languageId, method, params }),
            notify: (method, params) => invoke("lsp_notify", { language: languageId, method, params }),
        },
        documentUri: `file://${currentFilePath}`,
        languageId: languageId
    });
}