<script>
    import { workorderExpanded, activeWorkorder } from "../stores"

    export let workorder
    export let index
    export let statusList
    export let travelStatusList

    function handleExpand() {
        $workorderExpanded = !$workorderExpanded
        $activeWorkorder = index
    }

    $: console.log()
</script>

<style>
    .container {
        display: flex;
        justify-content: space-between;
        border-bottom: 2px solid #e3e3e3;
        padding: 10px;
        cursor: pointer;
    }
    .container > div {
        width: 200px;
        text-align: center;
    }
</style>

<div class="container" on:click={handleExpand}>
    <div>
        {workorder.customer.first_name + ' ' + workorder.customer.last_name}
    </div>
    <div>{workorder.device.name}</div>
    <div>{workorder.brief}</div>
    <div>{statusList[workorder.status].status}</div>
    <div>{travelStatusList[workorder.travel_status].status}</div>
</div>
