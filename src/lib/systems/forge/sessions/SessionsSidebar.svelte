<script lang="ts">
    import { FolderOpen, GitBranch, Plus } from '@lucide/svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { openFolder } from '#lib/core/registry/actions';
    import { filesStack } from '#lib/core/runes/files.svelte';
    import { settingsStack } from '#lib/core/runes/settings.svelte';
    import { toggleUIStack } from '#lib/core/runes/toggle-ui.svelte';
    import { basename } from '#lib/core/utils/format';
</script>

<div class="flex h-full flex-col overflow-y-auto">
    <div class="panel-title">{t('project.title')}</div>
    <div class="px-3 pb-3">
        {#if filesStack.projectPath}
            <p class="truncate font-medium">{basename(filesStack.projectPath)}</p>
            <p class="truncate text-meta text-faint" title={filesStack.projectPath}>
                {filesStack.projectPath}
            </p>
            {#if filesStack.git?.branch}
                <p class="mt-1 flex items-center gap-1.5 text-meta text-muted">
                    <GitBranch size={12} />
                    {filesStack.git.branch}
                </p>
            {/if}
        {:else}
            <p class="text-muted">{t('project.none')}</p>
        {/if}
        <button class="btn mt-2 w-full" onclick={openFolder}>
            <FolderOpen size={13} />
            {t('project.open')}
        </button>
    </div>

    <div class="panel-title">{t('sidebar.launch')}</div>
    <div class="px-1.5 pb-3">
        {#each settingsStack.config.profiles as profile (profile.id)}
            <button
                class="row-hover flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left"
                onclick={() => toggleUIStack.openNewSession({ profileId: profile.id })}
            >
                <Plus size={13} class="text-faint" />
                <span class="flex-1 truncate">{profile.name}</span>
                {#if profile.hooks}<span class="rounded bg-accent-soft px-1 text-[0.625rem] text-accent"
                        >hooks</span
                    >{/if}
            </button>
        {/each}
    </div>

    {#if settingsStack.config.recentProjects.length > 1}
        <div class="panel-title">{t('project.recent')}</div>
        <div class="px-1.5 pb-3">
            {#each settingsStack.config.recentProjects.filter((path) => path !== filesStack.projectPath) as path (path)}
                <button
                    class="row-hover flex w-full flex-col rounded-md px-2 py-1.5 text-left"
                    onclick={() => filesStack.openProject(path)}
                >
                    <span class="truncate">{basename(path)}</span>
                    <span class="truncate text-meta text-faint">{path}</span>
                </button>
            {/each}
        </div>
    {/if}
</div>
