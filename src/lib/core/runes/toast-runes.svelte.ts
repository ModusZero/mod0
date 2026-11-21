export type ToastKind = 'info' | 'success' | 'error';

export interface Toast {
    id: number;
    kind: ToastKind;
    message: string;
}

/** Avisos efímeros en pantalla. */
class ToastStack {
    toasts = $state<Toast[]>([]);
    private nextId = 1;

    push(kind: ToastKind, message: string, ttl = 4500) {
        const id = this.nextId++;
        this.toasts.push({ id, kind, message });
        setTimeout(() => this.dismiss(id), kind === 'error' ? ttl * 1.6 : ttl);
    }

    info = (message: string) => this.push('info', message);
    success = (message: string) => this.push('success', message);
    error = (error: unknown) => this.push('error', error instanceof Error ? error.message : String(error));

    dismiss(id: number) {
        this.toasts = this.toasts.filter((toast) => toast.id !== id);
    }
}

export const toastStack = new ToastStack();
