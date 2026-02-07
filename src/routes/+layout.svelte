<script lang="ts">
    import "../app.css";
    import ActivityBar from "$lib/components/layout/ActivityBar.svelte";
    import Header from "$lib/components/layout/Header.svelte";
    import Sidebar from "$lib/components/layout/Sidebar.svelte";
    import Footer from "$lib/components/layout/Footer.svelte";

    let { children } = $props();
</script>

<div class="flex flex-col h-screen w-screen overflow-hidden bg-main text-text transition-colors duration-300">
    <Header/>

    <div class="flex flex-1 overflow-hidden relative">
        <ActivityBar />
        
        <Sidebar/>

        <main class="flex-1 flex flex-col relative overflow-hidden bg-main shadow-inner">
            {@render children()}
        </main>
    </div>

    <Footer/>
</div>