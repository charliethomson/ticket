<script>
    import CollapsedWorkorders from "./Components/Workorder/CollapsedWorkorders.svelte"
    import ExpandedWorkorder from "./Components/Workorder/ExpandedWorkorder.svelte"
    import Nav from "./Components/Nav/Nav.svelte"
    import Form from "./Components/Form/Form.svelte"
    import Statuses from "./Components/Statuses/Statuses.svelte"
    import Location from "./Components/Statuses/Location.svelte"
    import Alert from "./Components/Alert.svelte"

    import { component, activeWorkorder, alertContent } from "./stores"

    $component = Form

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

    let workorders = {
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
                created: 1603342409,
                quoted_time: 1603342409,
                status: 0,
                travel_status: 0,
                location: null,
                customer: {
                    id: 1,
                    first_name: "Test",
                    last_name: "customer",
                    phone_number: "5551235555",
                    email: "test@customer.com",
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
                        user: {
                            id: 1,
                            first_name: "Charlie",
                            last_name: "Thomson",
                            email: "c.thomson@ubreakifix.com",
                        },
                        contents: "Test note content",
                        created: 1603342410,
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
                created: 1603342409,
                quoted_time: 1603342409,
                status: 0,
                travel_status: 0,
                location: null,
                customer: {
                    id: 1,
                    first_name: "Test",
                    last_name: "customer",
                    phone_number: "5551235555",
                    email: "test@customer.com",
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
                        user: {
                            id: 1,
                            first_name: "Charlie",
                            last_name: "Thomson",
                            email: "c.thomson@ubreakifix.com",
                        },
                        contents: "cocks and balls",
                        created: 1603342410,
                        next_update: null,
                    },
                ],
            },
        ],
    }

    // async function API(url) {
    //     const baseUrl = "http://offsite.repair/api/"
    //     const response = await fetch(baseUrl + url)
    //     const data = await response.json()
    //     return data
    // }

    // let workordersPromise = API("workorders?active=true")
</script>

<style>
    @import url("https://fonts.googleapis.com/css2?family=Montserrat");

    :global(html, body) {
        height: 100%;
        background-color: #121212;
    }
    * {
        color: #e3e3e3;
        font-family: "Montserrat", sans-serif;
    }
    main {
        height: 100%;
        display: flex;
        flex-direction: column;
    }
</style>

<main>
    <Nav workorder={workorders.message[$activeWorkorder]} />

    <svelte:component
        this={$component ? $component : CollapsedWorkorders}
        workorders={workorders.message}
        workorder={workorders.message[$activeWorkorder]}
        {statuses}
        {travelStatuses} />
</main>

{#if $alertContent}
    <Alert content={$alertContent} />
{/if}

<!-- TODO: How do I route to each workorder? Let's say for example we try to access offsite.repair/workorders/123589 to get a specific workorder. With the way the application is built, I could change the url to a certain workorder ID when it is clicked on but that's it.  -->
<!-- TODO: Make login button with a tag that links to /api/auth/login -->
