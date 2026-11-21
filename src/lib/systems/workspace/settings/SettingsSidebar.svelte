<script lang="ts">
    import { Bell, Info, Palette, Terminal } from '@lucide/svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { settingsStack } from '#lib/core/runes/settings.svelte';

    const sections = [
        { id: 'profiles', label: 'settings.profiles', icon: Terminal },
        { id: 'notifications', label: 'settings.notifications', icon: Bell },
        { id: 'appearance', label: 'settings.appearance', icon: Palette },
        { id: 'about', label: 'settings.about', icon: Info },
    ] as const;
</script>

<div class="flex h-full flex-col">
    <div class="panel-title">{t('activity.settings')}</div>
    <nav class="px-1.5">
        {#each sections as section (section.id)}
            <button
                class="flex w-full items-center gap-2.5 rounded-md px-2.5 py-1.5 text-left transition-colors {settingsStack.section ===
                section.id
                    ? 'bg-active'
                    : 'text-muted hover:bg-hover hover:text-text'}"
                onclick={() => (settingsStack.section = section.id)}
            >
                <section.icon size={14} />
                {t(section.label)}
            </button>
        {/each}
    </nav>
</div>
