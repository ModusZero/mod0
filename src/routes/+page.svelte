<script lang="ts">
    import { configStack } from "$lib/runes/config.svelte";
    
    function toggleTheme() {
        const newTheme = configStack.current.theme === "dark" ? "light" : "dark";
        configStack.update({ theme: newTheme });
    }
</script>

<main class="container">
    <h1>Modus Zero <small>v0.1.0</small></h1>
    
    <div class="status-card">
        <p>Tema actual: <strong>{configStack.current.theme}</strong></p>
        <p>Idioma: <strong>{configStack.current.language}</strong></p>
        
        <button onclick={toggleTheme}>
            Cambiar a tema {configStack.current.theme === "dark" ? "Claro" : "Oscuro"}
        </button>
    </div>
</main>

<style>
    :global(body.dark) {
        background-color: #1a1a1a;
        color: #ffffff;
    }
    :global(body.light) {
        background-color: #f6f6f6;
        color: #0f0f0f;
    }

    .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 80vh;
        font-family: sans-serif;
    }

    .status-card {
        padding: 2rem;
        border: 1px solid #444;
        border-radius: 12px;
        background: rgba(255, 255, 255, 0.05);
    }

    button {
        margin-top: 1rem;
        padding: 0.8rem 1.5rem;
        border-radius: 8px;
        cursor: pointer;
        border: 1px solid #396cd8;
        background: #396cd8;
        color: white;
    }
</style>