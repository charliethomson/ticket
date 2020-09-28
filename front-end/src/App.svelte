<script>
    import Store from "./Components/Store.svelte"
    import Customer from "./Components/Customer.svelte"
    import Device from "./Components/Device.svelte"

    import { storeSubmitted, customerSubmitted } from "./stores"
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
    <!-- {#if $storeSubmitted}
        <Customer />
        <Device />
    {:else}
        
    {/if} -->

    {#if $storeSubmitted && !$customerSubmitted}
        <Customer />
    {:else if $storeSubmitted && $customerSubmitted }
        <Device />
    {:else}
        <Store />
    {/if}
</main>
