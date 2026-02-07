<script lang="ts">
    import { configStack } from "$lib/runes/config.svelte";
    
    function toggleTheme() {
        const newTheme = configStack.current.theme === "dark" ? "light" : "dark";
        configStack.update({ theme: newTheme });
    }
</script>

<main class="flex h-[80vh] flex-col items-center justify-center font-sans">
    <h1 class="text-4xl font-bold mb-4">
        Mod0 <small class="text-sm opacity-50">v0.1.0</small>
    </h1>
    
    <div class="p-8 border border-white/10 rounded-xl bg-white/5 backdrop-blur-sm">
        <p class="mb-2">Tema: <span class="text-accent font-mono font-bold uppercase">{configStack.current.theme}</span></p>
        <p class="mb-6 opacity-80 text-sm italic">Persistencia activa en Rust</p>
        
        <button 
            onclick={toggleTheme}
            class="w-full py-2 px-6 rounded-lg bg-accent text-white font-semibold hover:opacity-90 active:scale-95 transition-all shadow-lg shadow-accent/20"
        >
            Cambiar a tema {configStack.current.theme === "dark" ? "Claro" : "Oscuro"}
        </button>
    </div>
</main>