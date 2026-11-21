<script lang="ts">
    import { Send } from '@lucide/svelte';
    import Toggle from '#lib/components/common/Toggle.svelte';
    import { webhookPresets } from '#lib/core/constants/webhook-presets';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { settingsStack } from '#lib/core/runes/settings.svelte';
    import { toastStack } from '#lib/core/runes/toast-runes.svelte';
    import { configService } from '#lib/core/services/config-service';
    import type { NotificationConfig } from '#lib/core/types/settings';

    let draft = $state<NotificationConfig>($state.snapshot(settingsStack.config.notifications));
    let dirty = $state(false);

    function patch(change: Partial<NotificationConfig>) {
        draft = { ...draft, ...change };
        dirty = true;
    }

    function patchWebhook(change: Partial<NotificationConfig['webhook']>) {
        patch({ webhook: { ...draft.webhook, ...change } });
    }

    function applyPreset(id: string) {
        const preset = webhookPresets.find((item) => item.id === id);
        if (preset) patchWebhook({ ...preset.webhook, enabled: true });
    }

    async function save() {
        try {
            await settingsStack.save({ notifications: $state.snapshot(draft) });
            dirty = false;
            toastStack.success(t('common.saved'));
        } catch (error) {
            toastStack.error(error);
        }
    }

    async function test() {
        try {
            if (dirty) await save();
            await configService.notifyTest();
        } catch (error) {
            toastStack.error(error);
        }
    }
</script>

<h1 class="mb-1 text-lg font-semibold">{t('settings.notifications')}</h1>
<p class="mb-6 text-muted">{t('settings.notificationsHint')}</p>

<section class="mb-6 divide-y divide-border-subtle">
    <Toggle
        checked={draft.onNeedsYou}
        label={t('notify.needsYou')}
        hint={t('notify.needsYouHint')}
        onchange={(onNeedsYou) => patch({ onNeedsYou })}
    />
    <Toggle
        checked={draft.onSessionDone}
        label={t('notify.sessionDone')}
        hint={t('notify.sessionDoneHint')}
        onchange={(onSessionDone) => patch({ onSessionDone })}
    />
    <Toggle
        checked={draft.onAllIdle}
        label={t('notify.allIdle')}
        hint={t('notify.allIdleHint')}
        onchange={(onAllIdle) => patch({ onAllIdle })}
    />
</section>

<section class="mb-6 divide-y divide-border-subtle">
    <Toggle checked={draft.desktop} label={t('notify.desktop')} onchange={(desktop) => patch({ desktop })} />
    <Toggle
        checked={draft.onlyWhenUnfocused}
        label={t('notify.onlyUnfocused')}
        hint={t('notify.onlyUnfocusedHint')}
        onchange={(onlyWhenUnfocused) => patch({ onlyWhenUnfocused })}
    />
</section>

<section class="mb-6 rounded-xl border border-border-subtle p-4">
    <Toggle
        checked={draft.webhook.enabled}
        label={t('notify.webhook')}
        hint={t('notify.webhookHint')}
        onchange={(enabled) => patchWebhook({ enabled })}
    />

    {#if draft.webhook.enabled}
        <div class="mt-3 space-y-3">
            <div>
                <span class="label">{t('notify.presets')}</span>
                <div class="flex flex-wrap gap-1.5">
                    {#each webhookPresets as preset (preset.id)}
                        <button class="btn" onclick={() => applyPreset(preset.id)}>{preset.label}</button>
                    {/each}
                </div>
            </div>

            <div class="grid grid-cols-[6rem_1fr] gap-3">
                <div>
                    <label class="label" for="wh-method">{t('notify.method')}</label>
                    <select
                        id="wh-method"
                        class="input"
                        value={draft.webhook.method}
                        onchange={(event) =>
                            patchWebhook({ method: event.currentTarget.value as 'GET' | 'POST' })}
                    >
                        <option>POST</option><option>GET</option>
                    </select>
                </div>
                <div>
                    <label class="label" for="wh-url">URL</label>
                    <input
                        id="wh-url"
                        class="input font-mono"
                        value={draft.webhook.url}
                        oninput={(event) => patchWebhook({ url: event.currentTarget.value })}
                    />
                </div>
            </div>

            {#if draft.webhook.method === 'POST'}
                <div>
                    <label class="label" for="wh-body">{t('notify.body')}</label>
                    <textarea
                        id="wh-body"
                        class="input font-mono"
                        rows="3"
                        value={draft.webhook.body}
                        oninput={(event) => patchWebhook({ body: event.currentTarget.value })}></textarea>
                </div>
                <div>
                    <label class="label" for="wh-type">Content-Type</label>
                    <input
                        id="wh-type"
                        class="input font-mono"
                        value={draft.webhook.contentType}
                        oninput={(event) => patchWebhook({ contentType: event.currentTarget.value })}
                    />
                </div>
            {/if}

            <p class="text-meta text-muted">
                {t('notify.placeholders')}:
                <code class="font-mono">{'{title} {message} {session} {status}'}</code>
                · <code class="font-mono">{'{message_url}'}</code> ·
                <code class="font-mono">{'{message_json}'}</code>
            </p>
            <p class="text-meta text-faint">{t('notify.secretsWarning')}</p>
        </div>
    {/if}
</section>

<div class="flex gap-2">
    <button class="btn btn-primary" onclick={save} disabled={!dirty}>{t('common.save')}</button>
    <button class="btn" onclick={test}>
        <Send size={13} />
        {t('notify.test')}
    </button>
</div>
