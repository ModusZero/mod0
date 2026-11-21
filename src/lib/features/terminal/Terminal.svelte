<script lang="ts">
    import { FitAddon } from '@xterm/addon-fit';
    import { WebglAddon } from '@xterm/addon-webgl';
    import { Terminal } from '@xterm/xterm';
    import '@xterm/xterm/css/xterm.css';
    import { onDestroy, onMount } from 'svelte';
    import { settingsStack } from '#lib/core/runes/settings.svelte';
    import { terminalService } from '#lib/core/services/terminal-service';
    import type { Session } from '#lib/core/types/session';
    import { isAppShortcut } from '#lib/features/workbench/hotkeys';
    import { terminalTheme } from './terminal-theme';

    interface Props {
        session: Session;
        visible: boolean;
    }

    let { session, visible }: Props = $props();

    let host: HTMLDivElement;
    let term = $state.raw<Terminal | null>(null);
    let fit: FitAddon | null = null;
    let webgl: WebglAddon | null = null;
    let observer: ResizeObserver | null = null;
    let attached = false;
    let frame = 0;

    const alive = $derived(session.status !== 'exited');

    async function attach() {
        if (!term || attached) return;
        attached = true;

        try {
            term.reset();
            const history = await terminalService.attach(session.id, (bytes) => term?.write(bytes));
            term.write(history);
            fitNow();
        } catch {
            attached = false;
        }
    }

    function fitNow() {
        cancelAnimationFrame(frame);
        frame = requestAnimationFrame(() => {
            if (!host || host.clientWidth === 0) return;
            fit?.fit();
        });
    }

    function useWebgl(enabled: boolean) {
        if (!term) return;
        if (!enabled) {
            webgl?.dispose();
            webgl = null;
            return;
        }
        if (webgl) return;

        // WebGL acelera mucho el pintado, pero solo se activa en la terminal visible.
        try {
            webgl = new WebglAddon();
            webgl.onContextLoss(() => {
                webgl?.dispose();
                webgl = null;
            });
            term.loadAddon(webgl);
        } catch {
            webgl = null;
        }
    }

    onMount(() => {
        const mono = getComputedStyle(document.documentElement).getPropertyValue('--font-mono').trim();
        const instance = new Terminal({
            fontFamily: mono || 'Consolas, monospace',
            fontSize: settingsStack.config.terminalFontSize,
            lineHeight: 1.2,
            cursorBlink: false,
            cursorStyle: 'bar',
            scrollback: 5000,
            macOptionIsMeta: true,
            theme: terminalTheme(settingsStack.dark),
        });

        fit = new FitAddon();
        instance.loadAddon(fit);
        instance.open(host);

        // Deja pasar los atajos de la aplicación y copia con Ctrl+C cuando hay selección.
        instance.attachCustomKeyEventHandler((event) => {
            if (isAppShortcut(event)) return false;
            if (event.type === 'keydown' && event.ctrlKey && event.key === 'c' && instance.hasSelection()) {
                void navigator.clipboard.writeText(instance.getSelection());
                instance.clearSelection();
                return false;
            }
            return true;
        });

        instance.onData((data) => void terminalService.write(session.id, data).catch(() => undefined));
        instance.onResize(({ rows, cols }) => {
            if (attached) void terminalService.resize(session.id, rows, cols).catch(() => undefined);
        });

        observer = new ResizeObserver(fitNow);
        observer.observe(host);
        term = instance;
    });

    onDestroy(() => {
        cancelAnimationFrame(frame);
        observer?.disconnect();
        webgl?.dispose();
        term?.dispose();
        if (attached) void terminalService.detach(session.id).catch(() => undefined);
    });

    // Conectar al PTY mientras la sesión viva; avisar cuando termina.
    $effect(() => {
        if (!term) return;
        if (alive) void attach();
        else if (attached) {
            attached = false;
            term.write('\r\n\x1b[2m[proceso terminado]\x1b[0m\r\n');
        }
    });

    $effect(() => {
        if (!term) return;
        useWebgl(visible);
        if (visible) {
            fitNow();
            term.focus();
        }
    });

    $effect(() => {
        if (!term) return;
        term.options.theme = terminalTheme(settingsStack.dark);
        term.options.fontSize = settingsStack.config.terminalFontSize;
        fitNow();
    });
</script>

<div bind:this={host} class="h-full w-full bg-main" class:hidden={!visible}></div>
