<script lang="ts">
    import { Rocket } from '@lucide/svelte';
    import Dialog from '#lib/components/common/Dialog.svelte';
    import Segmented from '#lib/components/common/Segmented.svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { openFolder } from '#lib/core/registry/actions';
    import { filesStack } from '#lib/core/runes/files.svelte';
    import { kanbanStack } from '#lib/core/runes/kanban.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import { settingsStack } from '#lib/core/runes/settings.svelte';
    import { toastStack } from '#lib/core/runes/toast-runes.svelte';
    import { toggleUIStack } from '#lib/core/runes/toggle-ui.svelte';

    let name = $state('');
    let profileId = $state('');
    let isolation = $state<'folder' | 'worktree'>('folder');
    let branch = $state('');
    let prompt = $state('');
    let busy = $state(false);

    const profiles = $derived(settingsStack.config.profiles);
    const canWorktree = $derived(filesStack.git?.isRepo ?? false);
    const projectPath = $derived(filesStack.projectPath);

    // Cada vez que se abre, se parte de los valores por defecto (o del preset de una tarea).
    $effect(() => {
        if (!toggleUIStack.newSessionOpen) return;
        const preset = toggleUIStack.newSessionPreset;
        name = preset.name ?? '';
        prompt = preset.prompt ?? '';
        branch = '';
        isolation = 'folder';
        profileId = preset.profileId ?? settingsStack.config.defaultProfileId;
    });

    function close() {
        toggleUIStack.newSessionOpen = false;
    }

    async function submit() {
        if (!projectPath || busy) return;
        busy = true;

        try {
            const session = await sessionsStack.create({
                name: name.trim(),
                profileId,
                projectPath,
                worktree: isolation === 'worktree',
                branch: branch.trim() || undefined,
                prompt: prompt.trim() || undefined,
            });

            const taskId = toggleUIStack.newSessionPreset.taskId;
            if (taskId) kanbanStack.update(taskId, { sessionId: session.id, column: 'doing' });

            toggleUIStack.terminalOpen = true;
            close();
        } catch (error) {
            toastStack.error(error);
        } finally {
            busy = false;
        }
    }
</script>

<Dialog open={toggleUIStack.newSessionOpen} title={t('session.new')} onclose={close}>
    {#if !projectPath}
        <div class="flex flex-col items-center gap-3 py-4 text-center">
            <p class="text-muted">{t('session.needProject')}</p>
            <button class="btn btn-primary" onclick={openFolder}>{t('project.open')}</button>
        </div>
    {:else}
        <form
            class="space-y-4"
            onsubmit={(event) => {
                event.preventDefault();
                void submit();
            }}
        >
            <div>
                <label class="label" for="ns-name">{t('session.name')}</label>
                <input
                    id="ns-name"
                    class="input"
                    bind:value={name}
                    placeholder={t('session.namePlaceholder')}
                />
            </div>

            <div>
                <label class="label" for="ns-profile">{t('session.profile')}</label>
                <select id="ns-profile" class="input" bind:value={profileId}>
                    {#each profiles as profile (profile.id)}
                        <option value={profile.id}>{profile.name}</option>
                    {/each}
                </select>
            </div>

            <div>
                <span class="label">{t('session.isolation')}</span>
                <Segmented
                    options={[
                        { value: 'folder', label: t('session.sameFolder') },
                        ...(canWorktree
                            ? [{ value: 'worktree' as const, label: t('session.worktree') }]
                            : []),
                    ]}
                    value={isolation}
                    onchange={(value) => (isolation = value)}
                />
                <p class="mt-1.5 text-meta text-muted">
                    {isolation === 'worktree' ? t('session.worktreeHint') : t('session.sameFolderHint')}
                </p>
                {#if isolation === 'worktree'}
                    <input
                        class="input mt-2 font-mono"
                        bind:value={branch}
                        placeholder={t('session.branchPlaceholder')}
                    />
                {/if}
            </div>

            <div>
                <label class="label" for="ns-prompt">{t('session.prompt')}</label>
                <textarea
                    id="ns-prompt"
                    class="input"
                    rows="3"
                    bind:value={prompt}
                    placeholder={t('session.promptPlaceholder')}></textarea>
            </div>

            <div class="flex items-center justify-between pt-1">
                <span class="max-w-[16rem] truncate text-meta text-faint" title={projectPath}
                    >{projectPath}</span
                >
                <div class="flex gap-2">
                    <button type="button" class="btn" onclick={close}>{t('common.cancel')}</button>
                    <button type="submit" class="btn btn-primary" disabled={busy || profiles.length === 0}>
                        <Rocket size={13} />
                        {busy ? t('common.starting') : t('session.start')}
                    </button>
                </div>
            </div>
        </form>
    {/if}
</Dialog>
