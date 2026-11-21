<script lang="ts">
    import { GitBranch, Radio } from '@lucide/svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { filesStack } from '#lib/core/runes/files.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import { settingsStack } from '#lib/core/runes/settings.svelte';

    const session = $derived(sessionsStack.focused);
</script>

<footer
    class="flex h-6 shrink-0 items-center justify-between border-t border-border-subtle bg-sidebar px-3 text-meta text-muted"
>
    <div class="flex items-center gap-4">
        {#if session?.branch ?? filesStack.git?.branch}
            <span class="flex items-center gap-1.5">
                <GitBranch size={12} />
                {session?.branch ?? filesStack.git?.branch}
            </span>
        {/if}
        {#if session}
            <span class="truncate">{session.name} · {session.profileName}</span>
        {/if}
    </div>

    <div class="flex items-center gap-4">
        <span
            class="flex items-center gap-1.5"
            title={settingsStack.info?.hookPort ? `127.0.0.1:${settingsStack.info.hookPort}` : ''}
        >
            <Radio size={12} class={settingsStack.info?.hookPort ? 'text-ok' : 'text-faint'} />
            {t('footer.observer')}
        </span>
        <span class="font-mono">v{settingsStack.info?.version ?? ''}</span>
    </div>
</footer>
