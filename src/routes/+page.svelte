<script lang="ts">
    import ActivityBar from '#lib/components/layout/ActivityBar.svelte';
    import AttentionRail from '#lib/components/layout/AttentionRail.svelte';
    import Footer from '#lib/components/layout/Footer.svelte';
    import Header from '#lib/components/layout/Header.svelte';
    import Sidebar from '#lib/components/layout/Sidebar.svelte';
    import Workspace from '#lib/components/layout/Workspace.svelte';
    import NewSessionDialog from '#lib/features/workbench/NewSessionDialog.svelte';
    import Omnibar from '#lib/features/workbench/Omnibar.svelte';
</script>

<div class="flex h-full flex-col">
    <Header />
    <div class="flex min-h-0 flex-1">
        <ActivityBar />
        <Sidebar />
        <Workspace />
        <AttentionRail />
    </div>
    <Footer />
</div>

<Omnibar />
<NewSessionDialog />
