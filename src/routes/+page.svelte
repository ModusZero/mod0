<script lang="ts">
    import { configStack } from "$lib/runes/config.svelte";
    
    function toggleTheme() {
        const newTheme = configStack.current.theme === "dark" ? "light" : "dark";
        configStack.update({ theme: newTheme });
    }
</script>

<main class="flex h-[80vh] flex-col items-center justify-center">
    <div class="p-10 border border-border-subtle rounded-2xl bg-main shadow-2xl transition-all duration-500">
        <h1 class="text-3xl font-black mb-2 text-text">MOD0 SYSTEM</h1>
        
        <p class="mb-8 text-text/40 font-mono text-sm uppercase tracking-tighter">
            Theme engine v1.0.4
        </p>
        
        <button 
            onclick={toggleTheme}
            class="w-full py-4 px-8 rounded-xl bg-accent text-main font-bold hover:scale-[1.02] active:scale-[0.98] transition-all shadow-lg shadow-accent/20"
        >
            PASAR A MODO {configStack.current.theme === "dark" ? "CLARO" : "OSCURO"}
        </button>
    </div>
</main>