<script>
    import { getWorkorders } from '../../utils'
    import Container from "../Helpers/Container.svelte"
    import WorkorderLink from "../Helpers/WorkorderLink.svelte"

    export let statuses = []
    export let travelStatuses = []
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
    {#await getWorkorders()}
        <div>Loading workorders...</div>
    {:then workorders}
        {#each workorders as workorder}
            <WorkorderLink href={`/workorder/${workorder.workorder_id}`}>
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
    {:catch error}
        <div>Unable to fetch workorders. Error: {error}</div>
    {/await}
</Container>
