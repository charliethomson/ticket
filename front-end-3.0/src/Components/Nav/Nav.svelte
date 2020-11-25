<script>
    import { onMount } from "svelte";
    import Button from "../Helpers/Button.svelte"
    import Tooltip from "../Tooltip/Tooltip.svelte"
    import Customer from "../Tooltip/Customer.svelte"
    import Device from "../Tooltip/Device.svelte"
    import NavLink from "../Helpers/NavLink.svelte"

    let tooltip = null
    let tooltipShown = false
    let url = ""

    function goToRepairQ() {
        alert("Switching the queue to repairs that need to completed")
    }
    function goToInProgress() {
        alert("Switching the queue to repairs in progress")
    }
    function showDevice() {
        tooltip = Device
        tooltipShown = !tooltipShown
    }
    function showCustomer() {
        tooltip = Customer
        tooltipShown = !tooltipShown
    }
    function urlChange() {
        url = window.location.hash.substring(1)
    }

    onMount(urlChange)
</script>

<style>
    .nav {
        display: flex;
        justify-content: space-between;
        align-items: center;
        background-color: #212121;
        height: 80px;
        font-size: 18px;
        margin-bottom: 40px;
    }

    .buttons {
        margin-left: 15px;
        position: relative;
        display: flex;
        cursor: pointer;
        height: 100%;
    }
    .account {
        margin-right: 15px;
        padding: 25px;
        font-weight: bold;
        background-color: #212121;
    }
    .account:hover {
        background-color: #121212;
    }
</style>

<svelte:window on:hashchange={urlChange} />

<div class="nav">
    {#if url.includes('/workorder')}
        <div class="buttons">
            <NavLink href="/create-workorder">Create Workorder</NavLink>
            <NavLink href="/">View All Workorders</NavLink>
            <Button content="Device" handleClick={showDevice} />
            <Button content="Customer" handleClick={showCustomer} />
            {#if tooltipShown}
                <Tooltip {tooltip} />
            {/if}
        </div>
    {:else if url === '/create-workorder'}
        <div class="buttons">
            <NavLink href="/">View All Workorders</NavLink>
            <NavLink href="/repair-queue">Repair Queue</NavLink>
            <Button content="In Progress" handleClick={goToInProgress} />
        </div>
    {:else}
        <div class="buttons">
            <NavLink href="/create-workorder">Create Workorder</NavLink>
            <NavLink href="/repair-queue">Repair Queue</NavLink>
            <NavLink href="/in-progress">In Progress</NavLink>
        </div>
    {/if}

    <div class="account">Justin Moore</div>
</div>
