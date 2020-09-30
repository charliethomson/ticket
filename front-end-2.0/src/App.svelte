<script>
    import CreateStore from "./Components/Create/CreateStore.svelte"
</script>

<style>
    @import url("https://fonts.googleapis.com/css2?family=Fira+Sans");
    :global(html, body) {
        height: 100%;
    }
    main {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100%;
        background-color: #121212;
        color: white;
    }
</style>

<main>
    <CreateStore />
</main>
