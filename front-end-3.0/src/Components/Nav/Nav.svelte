<script>
    import Button from "../Button.svelte"
    import Form from "../Form/Form.svelte"
    import CollapsedWorkorders from "../Workorder/CollapsedWorkorders.svelte"
    import ExpandedWorkorder from "../Workorder/ExpandedWorkorder.svelte"
    import Tooltip from "../Tooltip/Tooltip.svelte"
    import Customer from "../Tooltip/Customer.svelte"
    import Device from "../Tooltip/Device.svelte"
    import { component } from "../../stores"

    export let workorder

    let tooltip = null
    let tooltipShown = false

    let dateExpected = new Date(workorder.quoted_time * 1000).toDateString()

    function goToForm() {
        $component = Form
    }
    function goToRepairQ() {
        alert("Switching the queue to repairs that need to completed")
    }
    function goToInProgress() {
        alert("Switching the queue to repairs in progress")
    }
    function viewWorkorders() {
        $component = CollapsedWorkorders
    }
    function showDevice() {
        tooltip = Device
        tooltipShown = !tooltipShown
    }
    function showCustomer() {
        tooltip = Customer
        tooltipShown = !tooltipShown
    }
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

<div class="nav">
    {#if $component === ExpandedWorkorder}
        <div class="buttons">
            <Button content="Create Workorder" handleClick={goToForm} />
            <Button
                content="View All Workorders"
                handleClick={viewWorkorders} />
            <Button content="Device" handleClick={showDevice} />
            <Button content="Customer" handleClick={showCustomer} />
            {#if tooltipShown}
                <Tooltip {workorder} {tooltip} {dateExpected} />
            {/if}
        </div>
    {:else if $component === Form}
        <div class="buttons">
            <Button
                content="View All Workorders"
                handleClick={viewWorkorders} />
            <Button content="Repair Queue" handleClick={goToRepairQ} />
            <Button content="In Progress" handleClick={goToInProgress} />
        </div>
    {:else}
        <div class="buttons">
            <Button content="Create Workorder" handleClick={goToForm} />
            <Button content="Repair Queue" handleClick={goToRepairQ} />
            <Button content="In Progress" handleClick={goToInProgress} />
        </div>
    {/if}

    <div class="account">Justin Moore</div>
</div>
