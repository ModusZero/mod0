<script lang="ts">
    import { FolderOpen } from '@lucide/svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { settingsStack } from '#lib/core/runes/settings.svelte';
    import { windowService } from '#lib/core/services/window-service';

    const info = $derived(settingsStack.info);

    const shortcuts: Array<[string, string]> = [
        ['Ctrl+K / Ctrl+Shift+P', 'palette.commands'],
        ['Ctrl+P / Ctrl+Shift+O', 'palette.files'],
        ['Ctrl+Shift+N', 'session.new'],
        ['Alt+1…9', 'shortcut.jump'],
        ['Ctrl+Shift+A', 'rail.nextAttention'],
        ['Ctrl+1…6', 'shortcut.views'],
        ['Ctrl+`', 'header.terminal'],
        ['Ctrl+Shift+B / R', 'shortcut.panels'],
        ['Ctrl + / − / 0', 'shortcut.zoom'],
    ];
</script>

<h1 class="mb-1 text-lg font-semibold">
    MOD0 <span class="font-mono text-meta font-normal text-faint">v{info?.version}</span>
</h1>
<p class="mb-6 text-muted">{t('about.tagline')}</p>

<section class="mb-6 space-y-2">
    <h2 class="label">{t('about.runtime')}</h2>
    <dl class="grid grid-cols-[9rem_1fr] gap-y-1.5 text-ui">
        <dt class="text-muted">{t('about.observer')}</dt>
        <dd class="font-mono">{info?.hookPort ? `127.0.0.1:${info.hookPort}` : t('about.inactive')}</dd>
        <dt class="text-muted">{t('about.shell')}</dt>
        <dd class="font-mono">{info?.shell}</dd>
        <dt class="text-muted">{t('about.data')}</dt>
        <dd class="flex items-center gap-2 font-mono break-all">
            {info?.dataDir}
            {#if info}<button
                    class="icon-btn size-6"
                    onclick={() => windowService.openPath(info.dataDir)}
                    aria-label={t('about.open')}><FolderOpen size={13} /></button
                >{/if}
        </dd>
        <dt class="text-muted">{t('about.config')}</dt>
        <dd class="font-mono break-all">{info?.configDir}</dd>
    </dl>
</section>

<section class="mb-6">
    <h2 class="label">{t('about.how')}</h2>
    <p class="text-muted">{t('about.howText')}</p>
</section>

<section>
    <h2 class="label">{t('about.shortcuts')}</h2>
    <dl class="grid grid-cols-[13rem_1fr] gap-y-1">
        {#each shortcuts as [keys, label] (keys)}
            <dt><span class="kbd">{keys}</span></dt>
            <dd class="text-muted">{t(label)}</dd>
        {/each}
    </dl>
</section>
