<script>
    import CollapsedWorkorder from "./Components/CollapsedWorkorder.svelte"
    import ExpandedWorkorder from "./Components/ExpandedWorkorder.svelte"
    import Modal from "./Components/Modal/Modal.svelte"
    import Nav from "./Components/Nav.svelte"

    import { workorderExpanded, seeModal } from "./stores"

    let statusesShown = false

    // const statuses = [
    //     "Awaiting Repair",
    //     "Repair in Progress",
    //     "Repaired",
    //     "Unrepairable",
    //     "Awaiting Callback",
    //     "Awaiting Device",
    //     "Awaiting Parts",
    //     "Quality Inspection",
    //     "Need to order parts",
    // ]
    const statuses = [
        {
            status: "Awaiting Repair",
            color: "red",
        },
        {
            status: "Repair in Progress",
            color: "blue",
        },
        {
            status: "Repaired",
            color: "green",
        },
        {
            status: "Unrepairable",
            color: "green",
        },
        {
            status: "Awaiting Callback",
            color: "yellow",
        },
        {
            status: "Awaiting Device",
            color: "yellow",
        },
        {
            status: "Awaiting Parts",
            color: "yellow",
        },
        {
            status: "Quality Inspection",
            color: "red",
        },
        {
            status: "Need to Order Parts",
            color: "red",
        },
    ]
    let indexShown = 0

    $: console.log(indexShown)

    function showStatuses() {
        statusesShown = !statusesShown
    }
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
        margin-top: 50px;
        display: flex;
        justify-content: space-between;
    }
    .statuses {
        display: flex;
        flex-direction: row;
        cursor: pointer;
    }
    .active-status {
        display: inline;
        margin-left: 70px;
        border-bottom: 2px solid #f44336;
        font-weight: bold;
        margin-right: 20px;
    }
    .status-container {
        display: flex;
    }
    .status {
        visibility: hidden;
        margin-right: 10px;
        user-select: none;
    }
    .shown {
        visibility: visible;
    }
    /* Colors for statuses*/
    .blue {
        border-bottom: 2px solid #2196f3;
    }
    .red {
        border-bottom: 2px solid #f44336;
    }
    .yellow {
        border-bottom: 2px solid #ffeb3b;
    }
    .green {
        border-bottom: 2px solid #388e3c;
    }
    .blue-hover:hover {
        border-bottom: 2px solid #2196f3;
    }
    .red-hover:hover {
        border-bottom: 2px solid #f44336;
    }
    .yellow-hover:hover {
        border-bottom: 2px solid #ffeb3b;
    }
    .green-hover:hover {
        border-bottom: 2px solid #388e3c;
    }
    /* Colors for statuses */
    .delivery {
        color: #388e3c;
    }
    .location {
        display: flex;
        font-weight: bold;
        margin-top: 10px;
        margin-right: 70px;
    }
    .storage {
        margin-left: 5px;
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
            <div class="statuses">
                <div
                    class={'active-status ' + statuses[indexShown].color}
                    on:click={showStatuses}>
                    {statuses[indexShown].status}
                </div>

                <div class="status-container">
                    {#each statuses as { status, color }, i}
                        {#if i != indexShown}
                            <div
                                class={'status ' + color + '-hover' + (statusesShown ? ' shown' : '')}
                                on:click={() => {
                                    indexShown = i
                                    showStatuses()
                                }}>
                                {status}
                            </div>
                        {/if}
                    {/each}
                </div>
            </div>
            <div class="location">
                <div class="delivery">Arrived</div>
                <div class="storage">C7</div>
            </div>
        </div>
        <div class="workorders">
            <ExpandedWorkorder />
        </div>
    {/if}
</main>
{#if $seeModal}
    <Modal />
{/if}
