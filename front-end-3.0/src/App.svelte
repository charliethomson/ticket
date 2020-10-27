<script>
    import CollapsedWorkorder from "./Components/CollapsedWorkorder.svelte"
    import ExpandedWorkorder from "./Components/ExpandedWorkorder.svelte"
    import Modal from "./Components/Modal/Modal.svelte"
    import Nav from "./Components/Nav.svelte"
    import Statuses from "./Components/Statuses.svelte"
    import Location from "./Components/Location.svelte"

    import { workorderExpanded, seeModal, activeWorkorder } from "./stores"

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
    ]

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
    ]

    let workorder = {
        ok: true,
        message: [
            {
                workorder_id: 1,
                active: true,
                origin: {
                    id: 1,
                    name: "Test store",
                    contact_name: "Test Contact",
                    phone_number: "5555551234",
                    email: "test@store.com",
                    address: "102 Road Rd",
                    city: "Real City",
                    state: "Real State",
                    zip: "12345",
                },
                created: 1603212828,
                quoted_time: 1603212828,
                status: 0,
                travel_status: 1,
                location: null,
                customer: {
                    id: 1,
                    first_name: "Bob",
                    last_name: "Sagett",
                    phone_number: "5551235555",
                    email: "test@customer.com",
                    store_id: 1,
                },
                device: {
                    id: 1,
                    serial: "0123456789",
                    name: "Test device",
                    customer_id: 1,
                    password: "password1",
                },
                brief: "Test brief",
                notes: [
                    {
                        contents: "Test note content",
                        user: 1,
                        created: 1603212828,
                        next_update: null,
                    },
                ],
            },
            {
                workorder_id: 2,
                active: true,
                origin: {
                    id: 1,
                    name: "Test store",
                    contact_name: "Test Contact",
                    phone_number: "5555551234",
                    email: "test@store.com",
                    address: "102 Road Rd",
                    city: "Real City",
                    state: "Real State",
                    zip: "12345",
                },
                created: 1603212828,
                quoted_time: 1603212828,
                status: 0,
                travel_status: 1,
                location: null,
                customer: {
                    id: 1,
                    first_name: "Jeff",
                    last_name: "Sagett",
                    phone_number: "5551235555",
                    email: "test@customer.com",
                    store_id: 1,
                },
                device: {
                    id: 1,
                    serial: "0123456789",
                    name: "Test device",
                    customer_id: 1,
                    password: "password1",
                },
                brief: "Test brief",
                notes: [
                    {
                        contents: "Dicks",
                        user: 1,
                        created: 1603212828,
                        next_update: null,
                    },
                ],
            },
        ],
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
    <Nav workorder={workorder.message[$activeWorkorder]} />
    {#if !$workorderExpanded}
        <div class="workorders">
            <div class="titles">
                <div>Customer</div>
                <div>Device</div>
                <div>Description</div>
                <div>Status</div>
                <div>Location</div>
            </div>
            {#each workorder.message as workorder, i}
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
                workorder={workorder.message[$activeWorkorder]}
                statusList={statuses} />
            <Location
                travelStatusList={travelStatuses}
                workorder={workorder.message[$activeWorkorder]} />
        </div>
        <div class="workorders">
            <ExpandedWorkorder
                workorder={workorder.message[$activeWorkorder]} />
        </div>
    {/if}
</main>
{#if $seeModal}
    <Modal />
{/if}

<!-- TODO: How do I route to each workorder? Let's say for example we try to access offsite.repair/workorders/123589 to get a specific workorder. With the way the application is built, I could change the url to a certain workorder ID when it is clicked on but that's it.  -->
<!-- TODO: Make login button with a tag that links to /api/auth/login -->
