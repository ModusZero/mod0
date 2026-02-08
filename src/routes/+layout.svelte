<script lang="ts">
    import "../app.css";
    import ActivityBar from "$lib/components/layout/ActivityBar.svelte";
    import Header from "$lib/components/layout/Header.svelte";
    import Sidebar from "$lib/components/layout/Sidebar.svelte";
    import Footer from "$lib/components/layout/Footer.svelte";
    import Terminal from "$lib/components/editor/Terminal.svelte";
    import Settings from "$lib/components/settings/Settings.svelte";
    import Workspace from "$lib/components/common/Workspace.svelte";
</script>

<div class="flex flex-col h-screen w-screen overflow-hidden bg-main text-text transition-colors duration-300">
    <Header/>

    <div class="flex flex-1 overflow-hidden relative">
        <ActivityBar />
        
        <Sidebar/>

        <main class="flex-1 flex flex-col min-w-0 min-h-0">
            <div class="flex-1 min-h-0 overflow-hidden">
                <Workspace />
            </div>

            <Terminal />
        </main>
    </div>

    <Footer/>
</div>

<Settings />