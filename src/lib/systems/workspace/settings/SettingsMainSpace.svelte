<script lang="ts">
    import { settingsStack } from '#lib/core/runes/settings.svelte';
    import AboutPanel from '#lib/features/system-atoms/workspace/settings/AboutPanel.svelte';
    import AppearanceForm from '#lib/features/system-atoms/workspace/settings/AppearanceForm.svelte';
    import NotificationsForm from '#lib/features/system-atoms/workspace/settings/NotificationsForm.svelte';
    import ProfilesForm from '#lib/features/system-atoms/workspace/settings/ProfilesForm.svelte';
</script>

<div class="h-full overflow-y-auto">
    <div class="mx-auto max-w-3xl p-6">
        {#if settingsStack.section === 'profiles'}
            <ProfilesForm />
        {:else if settingsStack.section === 'notifications'}
            <NotificationsForm />
        {:else if settingsStack.section === 'appearance'}
            <AppearanceForm />
        {:else}
            <AboutPanel />
        {/if}
    </div>
</div>
