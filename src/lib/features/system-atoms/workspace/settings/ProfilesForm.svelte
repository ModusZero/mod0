<script lang="ts">
    import { CircleAlert, CircleCheck, Plus, Star, Trash } from '@lucide/svelte';
    import { onMount } from 'svelte';
    import Toggle from '#lib/components/common/Toggle.svelte';
    import { profilePresets, type ProfilePreset } from '#lib/core/constants/profile-presets';
    import { i18n, t } from '#lib/core/i18n/utils.svelte';
    import { settingsStack } from '#lib/core/runes/settings.svelte';
    import { toastStack } from '#lib/core/runes/toast-runes.svelte';
    import { configService } from '#lib/core/services/config-service';
    import type { CliProfile, CommandCheck } from '#lib/core/types/settings';

    let profiles = $state<CliProfile[]>($state.snapshot(settingsStack.config.profiles));
    let defaultId = $state(settingsStack.config.defaultProfileId);
    let dirty = $state(false);
    let presetId = $state(profilePresets[1].id);
    let checks = $state<Record<string, CommandCheck | undefined>>({});

    const preset = $derived(profilePresets.find((item) => item.id === presetId));

    const lines = (text: string) =>
        text
            .split('\n')
            .map((line) => line.trim())
            .filter(Boolean);

    function envToText(env: Record<string, string>): string {
        return Object.entries(env)
            .map(([key, value]) => `${key}=${value}`)
            .join('\n');
    }

    function textToEnv(text: string): Record<string, string> {
        return Object.fromEntries(
            lines(text).map((line) => {
                const at = line.indexOf('=');
                return at === -1 ? [line, ''] : [line.slice(0, at).trim(), line.slice(at + 1)];
            }),
        );
    }

    /** Comprueba que el ejecutable exista y que las variables `$NOMBRE` estén definidas en el sistema. */
    async function check(profile: CliProfile) {
        if (!profile.command.trim()) {
            checks[profile.id] = { found: true, path: null, missingEnv: [] };
            return;
        }
        try {
            checks[profile.id] = await configService.profileCheck(
                profile.command,
                $state.snapshot(profile.env),
            );
        } catch {
            checks[profile.id] = undefined;
        }
    }

    onMount(() => profiles.forEach((profile) => void check(profile)));

    function touch<T>(change: () => T): T {
        dirty = true;
        return change();
    }

    function add(template?: ProfilePreset) {
        const fresh: CliProfile = {
            id: `${template?.id ?? 'perfil'}-${Math.random().toString(36).slice(2, 6)}`,
            name: template?.profile.name ?? t('profiles.newName'),
            command: template?.profile.command ?? '',
            args: [...(template?.profile.args ?? [])],
            env: { ...(template?.profile.env ?? {}) },
            hooks: template?.profile.hooks ?? false,
            resumeArgs: [...(template?.profile.resumeArgs ?? [])],
        };
        touch(() => profiles.push(fresh));
        void check(fresh);
    }

    function remove(id: string) {
        if (profiles.length === 1) return;
        touch(() => (profiles = profiles.filter((profile) => profile.id !== id)));
        if (defaultId === id) defaultId = profiles[0].id;
    }

    /** Sugerencia de instalación según la plantilla con el mismo comando. */
    const installFor = (profile: CliProfile) =>
        profilePresets.find((item) => item.profile.command === profile.command)?.install;

    async function save() {
        try {
            await settingsStack.save({ profiles: $state.snapshot(profiles), defaultProfileId: defaultId });
            dirty = false;
            toastStack.success(t('common.saved'));
        } catch (error) {
            toastStack.error(error);
        }
    }
</script>

<h1 class="mb-1 text-lg font-semibold">{t('settings.profiles')}</h1>
<p class="mb-5 text-muted">{t('settings.profilesHint')}</p>

<section class="mb-6 rounded-xl border border-border-subtle bg-sidebar p-4">
    <h2 class="label">{t('profiles.fromTemplate')}</h2>
    <div class="flex gap-2">
        <select class="input flex-1" bind:value={presetId}>
            <optgroup label={t('profiles.groupClaude')}>
                {#each profilePresets.filter((item) => item.group === 'claude') as item (item.id)}
                    <option value={item.id}>{item.profile.name}</option>
                {/each}
            </optgroup>
            <optgroup label={t('profiles.groupOther')}>
                {#each profilePresets.filter((item) => item.group === 'other') as item (item.id)}
                    <option value={item.id}>{item.profile.name}</option>
                {/each}
            </optgroup>
        </select>
        <button class="btn btn-primary" onclick={() => preset && add(preset)}
            ><Plus size={13} />{t('profiles.add')}</button
        >
    </div>
    {#if preset}
        <p class="mt-2 text-meta text-muted">{preset.note[i18n.language]}</p>
        {#if preset.install}
            <p class="mt-1.5 text-meta text-faint">
                {t('profiles.install')}:
                <code class="rounded bg-main px-1.5 py-0.5 font-mono">{preset.install}</code>
            </p>
        {/if}
    {/if}
    <p class="mt-2 text-meta text-faint">{t('profiles.keysTip')}</p>
</section>

<div class="space-y-4">
    {#each profiles as profile (profile.id)}
        {@const state = checks[profile.id]}
        <section class="rounded-xl border border-border-subtle p-4">
            <header class="mb-3 flex items-center gap-2">
                <input
                    class="input max-w-60 font-medium"
                    bind:value={profile.name}
                    oninput={() => (dirty = true)}
                />
                <button
                    class="btn {defaultId === profile.id ? 'text-st-waiting' : 'btn-ghost'}"
                    onclick={() => touch(() => (defaultId = profile.id))}
                    title={t('profiles.default')}
                >
                    <Star size={13} fill={defaultId === profile.id ? 'currentColor' : 'none'} />
                    {defaultId === profile.id ? t('profiles.default') : t('profiles.makeDefault')}
                </button>
                <button
                    class="icon-btn ml-auto hover:text-danger"
                    disabled={profiles.length === 1}
                    onclick={() => remove(profile.id)}
                    aria-label={t('common.delete')}><Trash size={14} /></button
                >
            </header>

            <div class="grid grid-cols-2 gap-3">
                <div>
                    <label class="label" for="pf-cmd-{profile.id}">{t('profiles.command')}</label>
                    <input
                        id="pf-cmd-{profile.id}"
                        class="input font-mono"
                        placeholder="claude"
                        bind:value={profile.command}
                        oninput={() => (dirty = true)}
                        onchange={() => check(profile)}
                    />
                </div>
                <div>
                    <label class="label" for="pf-args-{profile.id}">{t('profiles.args')}</label>
                    <textarea
                        id="pf-args-{profile.id}"
                        class="input min-h-8 font-mono"
                        rows="1"
                        value={profile.args.join('\n')}
                        oninput={(event) => touch(() => (profile.args = lines(event.currentTarget.value)))}
                    ></textarea>
                </div>
                <div>
                    <label class="label" for="pf-env-{profile.id}">{t('profiles.env')}</label>
                    <textarea
                        id="pf-env-{profile.id}"
                        class="input font-mono"
                        rows="3"
                        placeholder="ANTHROPIC_AUTH_TOKEN=$OPENROUTER_API_KEY"
                        value={envToText(profile.env)}
                        oninput={(event) => touch(() => (profile.env = textToEnv(event.currentTarget.value)))}
                        onchange={() => check(profile)}></textarea>
                </div>
                <div>
                    <label class="label" for="pf-resume-{profile.id}">{t('profiles.resume')}</label>
                    <textarea
                        id="pf-resume-{profile.id}"
                        class="input font-mono"
                        rows="3"
                        placeholder={'--resume\n{session}'}
                        value={profile.resumeArgs.join('\n')}
                        oninput={(event) =>
                            touch(() => (profile.resumeArgs = lines(event.currentTarget.value)))}></textarea>
                </div>
            </div>

            {#if state}
                <div class="mt-3 space-y-1 text-meta">
                    {#if profile.command.trim()}
                        {#if state.found}
                            <p class="flex items-center gap-1.5 text-ok">
                                <CircleCheck size={13} />
                                {t('profiles.found')}
                                {#if state.path}<span class="truncate font-mono text-faint">{state.path}</span
                                    >{/if}
                            </p>
                        {:else}
                            <p class="flex items-center gap-1.5 text-st-waiting">
                                <CircleAlert size={13} />
                                {t('profiles.notFound', { command: profile.command })}
                                {#if installFor(profile)}
                                    <code class="rounded bg-main px-1.5 py-0.5 font-mono text-muted"
                                        >{installFor(profile)}</code
                                    >
                                {/if}
                            </p>
                        {/if}
                    {/if}
                    {#if state.missingEnv.length > 0}
                        <p class="flex items-center gap-1.5 text-st-waiting">
                            <CircleAlert size={13} />
                            {t('profiles.missingEnv')}
                            <span class="font-mono">{state.missingEnv.join(', ')}</span>
                        </p>
                    {/if}
                </div>
            {/if}

            <Toggle
                checked={profile.hooks}
                label={t('profiles.hooks')}
                hint={t('profiles.hooksHint')}
                onchange={(hooks) => touch(() => (profile.hooks = hooks))}
            />
        </section>
    {/each}
</div>

<div class="mt-4 flex items-center gap-2">
    <button class="btn" onclick={() => add()}><Plus size={13} />{t('profiles.addEmpty')}</button>
    <button class="btn btn-primary ml-auto" onclick={save} disabled={!dirty}>{t('common.save')}</button>
</div>
<p class="mt-3 text-meta text-faint">{t('profiles.secretsHint')}</p>
