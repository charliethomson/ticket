<script>
    import Container from "../Helpers/Container.svelte"
    import WorkorderLink from "../Helpers/WorkorderLink.svelte"

    export let statuses = []
    export let travelStatuses = []

    // function handleExpand(i) {
    //     $component = ExpandedWorkorder
    //     $activeWorkorder = i
    // }

    let workorders = [
        {
            id: 0,
            active: true,
            origin: 0,
            travel_status: 1,
            created: 1605187812,
            quoted_time: 5,
            status: 1,
            customer: {
                first_name: "Bob",
                last_name: "Black",
                phone_number: "111-111-1111",
                email_address: "bobblack@gmail.com",
            },
            device: {
                serial: "C1231234123",
                name: "Macbook Air",
                customer_id: 1,
                password: "Blahblah",
            },
            brief: "Dropped qweqwe",
        },
        {
            id: 1,
            active: true,
            origin: 0,
            travel_status: 2,
            created: 1605187812,
            quoted_time: 5,
            status: 0,
            customer: {
                first_name: "Bob",
                last_name: "Black",
                phone_number: "111-111-1111",
                email_address: "bobblack@gmail.com",
            },
            device: {
                serial: "C1231234123",
                name: "Macbook Air",
                customer_id: 1,
                password: "Blahblah",
            },
            brief: "Dropped and no worky",
        },
    ]
</script>

<style>
    .titles > div {
        width: 200px;
        text-align: center;
    }
    .titles {
        display: flex;
        justify-content: space-between;
        font-weight: bold;
        font-size: 20px;
        padding: 10px;
        border-bottom: 2px solid white;
    }
    .workorder-label {
        width: 200px;
        text-align: center;
    }
</style>

<Container>
    <div class="titles">
        <div>Customer</div>
        <div>Device</div>
        <div>Description</div>
        <div>Status</div>
        <div>Location</div>
    </div>
    {#each workorders as workorder}
        <WorkorderLink href={'/#/workorder'} id={workorder.id}>
            <div class="workorder-label">
                {workorder.customer.first_name + ' ' + workorder.customer.last_name}
            </div>
            <div class="workorder-label">{workorder.device.name}</div>
            <div class="workorder-label">{workorder.brief}</div>
            <div class="workorder-label">
                {statuses[workorder.status]?.status}
            </div>
            <div class="workorder-label">
                {travelStatuses[workorder.travel_status]?.status}
            </div>
        </WorkorderLink>
    {/each}
</Container>
