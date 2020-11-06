<script>
    import Container from "../Container.svelte"
    import ExpandedWorkorder from "../Workorder/ExpandedWorkorder.svelte"
    import { activeWorkorder, component } from "../../stores"

    export let workorders
    export let statuses
    export let travelStatuses

    function handleExpand(i) {
        $component = ExpandedWorkorder
        $activeWorkorder = i
    }
</script>

<style>
    .workorder {
        display: flex;
        justify-content: space-between;
        border-bottom: 2px solid #e3e3e3;
        padding: 10px;
        cursor: pointer;
    }
    .workorder > div {
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
    .titles > div {
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
    {#each workorders as workorder, i}
        <div class="workorder" on:click={(e) => handleExpand(i)}>
            <div>
                {workorder.customer.first_name + ' ' + workorder.customer.last_name}
            </div>
            <div>{workorder.device.name}</div>
            <div>{workorder.brief}</div>
            <div>{statuses[workorder.status].status}</div>
            <div>{travelStatuses[workorder.travel_status].status}</div>
        </div>
    {/each}
</Container>
