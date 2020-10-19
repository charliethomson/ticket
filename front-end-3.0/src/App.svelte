<script>
    import CollapsedWorkorder from "./Components/CollapsedWorkorder.svelte"
    import ExpandedWorkorder from "./Components/ExpandedWorkorder.svelte"
    import Modal from "./Components/Modal/Modal.svelte"
    import Nav from "./Components/Nav.svelte"
    import Statuses from "./Components/Statuses.svelte"
    import Location from "./Components/Location.svelte"

    import { workorderExpanded, seeModal } from "./stores"
</script>

<style>
    @import url("https://fonts.googleapis.com/css2?family=Montserrat");

    :global(html, body) {
        height: 100%;
    }
    * {
        color: #e3e3e3;
        font-family: "Montserrat", sans-serif;
    }
    main {
        display: flex;
        flex-direction: column;
        height: 100%;
        background-color: #121212;
    }

    .container {
        display: flex;
        justify-content: space-between;
        align-items: flex-end;
    }

    .workorders {
        height: 100%;
        margin: 50px;
        margin-top: 10px;
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
    <Nav customer="Colton Hobbs" />

    {#if !$workorderExpanded}
        <div class="workorders">
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
        </div>
    {:else}
        <div class="container">
            <Statuses />
            <Location />
        </div>
        <div class="workorders">
            <ExpandedWorkorder />
        </div>
    {/if}
</main>
{#if $seeModal}
    <Modal />
{/if}
