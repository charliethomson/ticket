<script>
    import CollapsedWorkorder from "./Components/CollapsedWorkorder.svelte"
    import ExpandedWorkorder from "./Components/ExpandedWorkorder.svelte"
    import Nav from "./Components/Nav.svelte"

    import { workorderExpanded } from "./stores"
</script>

<style>
    @import url("https://fonts.googleapis.com/css2?family=Montserrat");

    :global(html, body) {
        height: 100%;
    }
    * {
        color: white;
        font-family: "Montserrat", sans-serif;
    }
    main {
        display: flex;
        flex-direction: column;
        height: 100%;
        background-color: #121212;
        color: white;
    }
    .workorders {
        height: 100%;
        margin: 50px;
        background-color: #212121;
        border-radius: 10px;
        padding: 40px;
    }
    .titles {
        display: flex;
        justify-content: space-between;
        font-weight: bold;
        font-size: 20px;
        padding: 10px;
        border-bottom: 2px solid white;
    }
</style>

<main>
    <Nav />
    <div class="workorders">
        {#if !$workorderExpanded}
            <div class="titles">
                <div>Customer</div>
                <div>Device</div>
                <div>Description</div>
                <div>Status</div>
                <div>Location</div>
            </div>
            <CollapsedWorkorder
                customer="Colton Hobbs"
                device="Macbook Pro"
                description="No power/Doa"
                status="Awaiting Repair"
                location="Here - C7" />
            <CollapsedWorkorder
                customer="Colton Hobbs"
                device="Macbook Pro"
                description="No power/Doa"
                status="Awaiting Repair"
                location="Here - C7" />
        {:else}
            <ExpandedWorkorder />
        {/if}
    </div>
</main>
