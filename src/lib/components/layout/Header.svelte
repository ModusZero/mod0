<script lang="ts">
    import {
        Bell,
        ChevronDown,
        Eye,
        FolderOpen,
        Loader,
        Minus,
        PanelRight,
        Search,
        Square,
        SquareTerminal,
        X,
    } from '@lucide/svelte';
    import { platform } from '@tauri-apps/plugin-os';
    import logo from '#lib/assets/logo-unified.svg';
    import { jumpToAttention, openFolder } from '#lib/core/registry/actions';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { filesStack } from '#lib/core/runes/files.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import { settingsStack } from '#lib/core/runes/settings.svelte';
    import { toggleUIStack } from '#lib/core/runes/toggle-ui.svelte';
    import { windowService } from '#lib/core/services/window-service';
    import { basename } from '#lib/core/utils/format';

    let menuOpen = $state(false);

    const os = (() => {
        try {
            return platform();
        } catch {
            return 'windows';
        }
    })();

    const counts = $derived(sessionsStack.counts);
    const recents = $derived(
        settingsStack.config.recentProjects.filter((path) => path !== filesStack.projectPath),
    );

    async function choose(path: string) {
        menuOpen = false;
        await filesStack.openProject(path);
    }
</script>

<svelte:window onpointerdown={() => (menuOpen = false)} />

<header
    class="relative flex h-10 shrink-0 items-center border-b border-border-subtle bg-sidebar select-none"
    data-tauri-drag-region
>
    <!-- Izquierda: marca y proyecto -->
    <div class="flex items-center gap-2 pl-3 {os === 'macos' ? 'ml-16' : ''}" data-tauri-drag-region>
        <img src={logo} alt="MOD0" class="size-5" draggable="false" />

        <div class="relative" onpointerdown={(event) => event.stopPropagation()} role="presentation">
            <button class="btn btn-ghost gap-1.5 px-2" onclick={() => (menuOpen = !menuOpen)}>
                <span class="max-w-48 truncate font-medium text-text">
                    {filesStack.projectPath ? basename(filesStack.projectPath) : t('project.none')}
                </span>
                <ChevronDown size={13} />
            </button>

            {#if menuOpen}
                <div
                    class="absolute top-8 left-0 z-40 w-72 rounded-lg border border-border-subtle bg-raised p-1 shadow-xl"
                >
                    <button
                        class="row-hover flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left"
                        onclick={() => {
                            menuOpen = false;
                            void openFolder();
                        }}
                    >
                        <FolderOpen size={14} class="text-muted" />
                        {t('project.open')}
                    </button>

                    {#if recents.length > 0}
                        <div class="px-2.5 pt-2 pb-1 text-meta tracking-wide text-faint uppercase">
                            {t('project.recent')}
                        </div>
                        {#each recents as path (path)}
                            <button
                                class="row-hover flex w-full flex-col rounded-md px-2.5 py-1.5 text-left"
                                onclick={() => choose(path)}
                            >
                                <span>{basename(path)}</span>
                                <span class="truncate text-meta text-faint">{path}</span>
                            </button>
                        {/each}
                    {/if}
                </div>
            {/if}
        </div>
    </div>

    <!-- Centro: paleta -->
    <div class="pointer-events-none absolute inset-x-0 flex justify-center">
        <button
            class="pointer-events-auto flex h-7 w-[26rem] max-w-[34vw] items-center gap-2 rounded-md border border-border-subtle bg-main px-2.5 text-muted transition-colors hover:border-accent/40 hover:text-text"
            onclick={() => toggleUIStack.openPalette()}
        >
            <Search size={13} />
            <span class="flex-1 truncate text-left">{t('palette.trigger')}</span>
            <span class="kbd">Ctrl K</span>
        </button>
    </div>

    <!-- Derecha: estado global y controles -->
    <div class="ml-auto flex items-center gap-1 pr-1" data-tauri-drag-region>
        <div class="mr-2 flex items-center gap-1">
            {#if counts.working > 0}
                <span
                    class="flex items-center gap-1.5 rounded-md px-2 py-1 text-st-working"
                    title={t('status.working')}
                >
                    <Loader size={13} class="anim-spin" />
                    <span class="font-mono">{counts.working}</span>
                </span>
            {/if}
            {#if counts.waiting > 0}
                <button
                    class="flex items-center gap-1.5 rounded-md bg-st-waiting/12 px-2 py-1 text-st-waiting"
                    title={t('status.waiting')}
                    onclick={jumpToAttention}
                >
                    <Bell size={13} class="anim-pulse" />
                    <span class="font-mono">{counts.waiting}</span>
                </button>
            {/if}
            {#if counts.review > 0}
                <button
                    class="flex items-center gap-1.5 rounded-md bg-st-review/12 px-2 py-1 text-st-review"
                    title={t('status.review')}
                    onclick={jumpToAttention}
                >
                    <Eye size={13} />
                    <span class="font-mono">{counts.review}</span>
                </button>
            {/if}
        </div>

        <button class="icon-btn" title={t('header.terminal')} onclick={toggleUIStack.toggleTerminal}>
            <SquareTerminal size={16} class={toggleUIStack.terminalOpen ? 'text-accent' : ''} />
        </button>
        <button class="icon-btn" title={t('header.rail')} onclick={toggleUIStack.toggleRail}>
            <PanelRight size={16} class={toggleUIStack.railOpen ? 'text-accent' : ''} />
        </button>

        {#if os !== 'macos'}
            <div class="ml-1 flex">
                <button
                    class="flex h-10 w-11 items-center justify-center text-muted hover:bg-hover"
                    onclick={windowService.minimize}
                    aria-label="Minimizar"
                >
                    <Minus size={15} />
                </button>
                <button
                    class="flex h-10 w-11 items-center justify-center text-muted hover:bg-hover"
                    onclick={windowService.toggleMaximize}
                    aria-label="Maximizar"
                >
                    <Square size={12} />
                </button>
                <button
                    class="flex h-10 w-11 items-center justify-center text-muted hover:bg-danger hover:text-white"
                    onclick={windowService.close}
                    aria-label="Cerrar"
                >
                    <X size={16} />
                </button>
            </div>
        {/if}
    </div>
</header>
