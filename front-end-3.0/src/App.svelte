<script>
    import CollapsedWorkorder from "./Components/CollapsedWorkorder.svelte";
    import ExpandedWorkorder from "./Components/ExpandedWorkorder.svelte";
    import Modal from "./Components/Modal/Modal.svelte";
    import Nav from "./Components/Nav.svelte";
    import Statuses from "./Components/Statuses.svelte";
    import Location from "./Components/Location.svelte";
    import Alert from "./Components/Alert.svelte";

    import {
        workorderExpanded,
        seeModal,
        activeWorkorder,
        isNoteValid,
        isFormValid,
    } from "./stores";

    const travelStatuses = [
        {
            status: "Arrived",
            color: "green",
        },
        {
            status: "En-route",
            color: "yellow",
        },
        {
            status: "OG store",
            color: "yellow",
        },
    ];

    const statuses = [
        {
            status: "Awaiting Repair",
            color: "red",
        },
        {
            status: "Quality Inspection",
            color: "red",
        },
        {
            status: "Need to Order Parts",
            color: "red",
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
    ];

    async function API(url) {
        const baseUrl = "http://offsite.repair/api/";
        const response = await fetch(baseUrl + url);
        const data = await response.json();
        return data;
    }

    let workordersPromise = API("workorders?active=true");
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
    .titles > div {
        width: 200px;
        text-align: center;
    }
</style>

<main>
    {#await workordersPromise}
        <div class="waiting">We are waiting on workorders to load...</div>
    {:then workorders}
        <Nav workorder={workorders.message[$activeWorkorder]} />
        {#if !$workorderExpanded}
            <div class="workorders">
                <div class="titles">
                    <div>Customer</div>
                    <div>Device</div>
                    <div>Description</div>
                    <div>Status</div>
                    <div>Location</div>
                </div>
                {#each workorders.message as workorder, i}
                    <CollapsedWorkorder
                        {workorder}
                        index={i}
                        statusList={statuses}
                        travelStatusList={travelStatuses} />
                {/each}
            </div>
        {:else}
            <div class="container">
                <Statuses
                    workorder={workorders.message[$activeWorkorder]}
                    statusList={statuses} />
                <Location
                    travelStatusList={travelStatuses}
                    workorder={workorders.message[$activeWorkorder]} />
            </div>
            <div class="workorders">
                <ExpandedWorkorder
                    workorder={workorders.message[$activeWorkorder]} />
            </div>
        {/if}
    {:catch error}
        <div class="error">We weren't able to get the workorders</div>
    {/await}
</main>
{#if $seeModal}
    <Modal />
{/if}

{#if !$isFormValid}
    <Alert content="Please check your form input!" />
{/if}

{#if !$isNoteValid}
    <Alert content="Please enter valid notes!" />
{/if}

<!-- TODO: How do I route to each workorder? Let's say for example we try to access offsite.repair/workorders/123589 to get a specific workorder. With the way the application is built, I could change the url to a certain workorder ID when it is clicked on but that's it.  -->
<!-- TODO: Make login button with a tag that links to /api/auth/login -->
