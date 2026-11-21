<script lang="ts">
    import Segmented from '#lib/components/common/Segmented.svelte';
    import Toggle from '#lib/components/common/Toggle.svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { settingsStack } from '#lib/core/runes/settings.svelte';
    import type { AccentID } from '#lib/core/types/settings';

    const accents: Array<{ id: AccentID; color: string }> = [
        { id: 'blue', color: 'oklch(0.72 0.12 255)' },
        { id: 'teal', color: 'oklch(0.72 0.12 195)' },
        { id: 'violet', color: 'oklch(0.72 0.12 295)' },
        { id: 'amber', color: 'oklch(0.72 0.12 75)' },
        { id: 'rose', color: 'oklch(0.72 0.12 12)' },
    ];

    const config = $derived(settingsStack.config);
</script>

<h1 class="mb-1 text-lg font-semibold">{t('settings.appearance')}</h1>
<p class="mb-6 text-muted">{t('settings.appearanceHint')}</p>

<div class="space-y-6">
    <section>
        <span class="label">{t('settings.theme')}</span>
        <Segmented
            options={[
                { value: 'dark', label: t('theme.dark') },
                { value: 'light', label: t('theme.light') },
                { value: 'system', label: t('theme.system') },
            ]}
            value={config.theme}
            onchange={(theme) => settingsStack.save({ theme })}
        />
    </section>

    <section>
        <span class="label">{t('settings.accent')}</span>
        <div class="flex gap-2">
            {#each accents as accent (accent.id)}
                <button
                    class="size-7 rounded-full ring-offset-2 ring-offset-main transition-shadow {config.accent ===
                    accent.id
                        ? 'ring-2 ring-text'
                        : 'hover:ring-2 hover:ring-border-subtle'}"
                    style="background:{accent.color}"
                    aria-label={accent.id}
                    onclick={() => settingsStack.save({ accent: accent.id })}
                ></button>
            {/each}
        </div>
    </section>

    <section>
        <span class="label">{t('settings.language')}</span>
        <Segmented
            options={[
                { value: 'es', label: 'Español' },
                { value: 'en', label: 'English' },
            ]}
            value={config.language}
            onchange={(language) => settingsStack.save({ language })}
        />
    </section>

    <section class="grid grid-cols-2 gap-6">
        <div>
            <label class="label" for="ap-scale"
                >{t('settings.scale')} · {Math.round(config.uiScale * 100)}%</label
            >
            <input
                id="ap-scale"
                type="range"
                min="0.8"
                max="1.4"
                step="0.05"
                value={config.uiScale}
                class="w-full accent-[var(--accent)]"
                onchange={(event) => settingsStack.save({ uiScale: Number(event.currentTarget.value) })}
            />
        </div>
        <div>
            <label class="label" for="ap-font"
                >{t('settings.terminalFont')} · {config.terminalFontSize}px</label
            >
            <input
                id="ap-font"
                type="range"
                min="10"
                max="20"
                step="1"
                value={config.terminalFontSize}
                class="w-full accent-[var(--accent)]"
                onchange={(event) =>
                    settingsStack.save({ terminalFontSize: Number(event.currentTarget.value) })}
            />
        </div>
    </section>

    <Toggle
        checked={config.reduceMotion}
        label={t('settings.reduceMotion')}
        hint={t('settings.reduceMotionHint')}
        onchange={(reduceMotion) => settingsStack.save({ reduceMotion })}
    />

    <Toggle
        checked={config.autoSave}
        label={t('settings.autoSave')}
        hint={t('settings.autoSaveHint')}
        onchange={(autoSave) => settingsStack.save({ autoSave })}
    />
</div>
