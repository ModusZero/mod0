<script lang="ts">
    import '../app.css';
    import { onMount, type Snippet } from 'svelte';
    import ToastHost from '#lib/components/common/ToastHost.svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { filesStack } from '#lib/core/runes/files.svelte';
    import { graphStack } from '#lib/core/runes/graph.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import { settingsStack } from '#lib/core/runes/settings.svelte';
    import { handleHotkey } from '#lib/features/workbench/hotkeys';

    let { children }: { children: Snippet } = $props();

    let ready = $state(false);
    let failure = $state<string | null>(null);

    onMount(() => {
        const cleanups: Array<() => void> = [];

        // Captura (fase de captura) para que los atajos funcionen aunque el foco esté en la terminal.
        window.addEventListener('keydown', handleHotkey, true);
        cleanups.push(() => window.removeEventListener('keydown', handleHotkey, true));

        // Al volver a la ventana se refresca el árbol (no hay watchers de archivos).
        const onFocus = () => void filesStack.refresh();
        window.addEventListener('focus', onFocus);
        cleanups.push(() => window.removeEventListener('focus', onFocus));

        // Sin menú contextual del navegador: la app es de escritorio.
        const blockMenu = (event: MouseEvent) => event.preventDefault();
        window.addEventListener('contextmenu', blockMenu);
        cleanups.push(() => window.removeEventListener('contextmenu', blockMenu));

        (async () => {
            try {
                await settingsStack.load();
                cleanups.push(await sessionsStack.init(), await graphStack.init());
                cleanups.push(await filesStack.init());
                ready = true;
            } catch (error) {
                failure = error instanceof Error ? error.message : String(error);
            }
        })();

        return () => cleanups.forEach((stop) => stop());
    });
</script>

{#if failure}
    <div class="flex h-full flex-col items-center justify-center gap-2 p-8 text-center">
        <p class="font-medium text-danger">{t('boot.failed')}</p>
        <p class="max-w-lg font-mono text-meta text-muted">{failure}</p>
        <p class="text-meta text-faint">{t('boot.hint')}</p>
    </div>
{:else if ready}
    {@render children()}
{/if}

<ToastHost />
