<script>
    import CollapsedWorkorder from "./Components/CollapsedWorkorder.svelte";
    import ExpandedWorkorder from "./Components/ExpandedWorkorder.svelte";
    import Modal from "./Components/Modal/Modal.svelte";
    import Nav from "./Components/Nav.svelte";
    import Status from "./Components/Status.svelte";

    import { workorderExpanded, seeModal } from "./stores";

    const collapsedWorkorders = fetch(
        "http://localhost:8080/api/workorders?active=true"
    )
        .then(async (response) => await response.json())
        .then((json) => {
            console.log(json);
            if (json.ok) {
                return json.message.map((workorder) => ({
                    workorderId: workorder.workorder_id,
                    customer: workorder.customer.name,
                    device: workorder.device.name,
                    description: workorder.brief,
                    status: Status.statuses[workorder.status].status,
                    travelStatus:
                        Status.travelStatuses[workorder.travel_status].status,
                    location: workorder.location,
                }));
            } else {
                return null;
            }
        })
        .catch((e) => console.error(`Error in fetch ${e}`));
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

    {#if $workorderExpanded === null}
        <div class="workorders">
            <div class="titles">
                <div>Customer</div>
                <div>Device</div>
                <div>Description</div>
                <div>Status</div>
                <div>Location</div>
            </div>
            {#await collapsedWorkorders}
                <p>Loading</p>
            {:then workorders}
                {#each workorders as workorder}
                    <CollapsedWorkorder
                        workorderId={workorder.workorderId}
                        customer={workorder.customer}
                        device={workorder.device}
                        description={workorder.description}
                        status={workorder.status}
                        travelStatus={workorder.travelStatus}
                        location={workorder.location} />
                {/each}
            {/await}
        </div>
    {:else}
        <Status />
        <div class="workorders">
            <ExpandedWorkorder />
        </div>
    {/if}
</main>
{#if $seeModal}
    <Modal />
{/if}
