<script>
    export let customer

    import Button from "./Button.svelte"

    import { workorderExpanded, seeModal } from "../stores"

    function goToCreate() {
        $seeModal = true
    }
    function goToRepairQ() {
        alert("Switching the queue to repairs that need to completed")
    }
    function goToInProgress() {
        alert("Switching the queue to repairs in progress")
    }
    function viewWorkorders() {
        $workorderExpanded = !$workorderExpanded
    }
</script>

<style>
    .nav {
        display: flex;
        justify-content: space-between;
    }
    .buttons {
        margin-left: 30px;
        position: relative;
        top: -20px;
        display: flex;
        cursor: pointer;
    }
    .account {
        font-weight: bold;
        align-self: center;
        background-color: #212121;
        border-radius: 10px;
        padding: 20px;
        margin-top: 20px;
        margin-bottom: 10px;
        margin-right: 50px;
    }
    .title {
        margin-top: 30px;
        width: 650px;
        align-self: center;
        font-size: 33px;
        font-weight: bold;
    }
</style>

<div class="nav">
    {#if !$workorderExpanded}
        <div class="buttons">
            <Button content="Create workorder" handleClick={goToCreate} />
            <Button content="Repair queue" handleClick={goToRepairQ} />
            <Button content="In progress" handleClick={goToInProgress} />
        </div>

        <div class="title">All workorders</div>
    {:else}
        <div class="buttons">
            <Button content="Create workorder" handleClick={goToCreate} />
            <Button content="View workorders" handleClick={viewWorkorders} />
        </div>

        <div class="title">{customer}</div>
    {/if}

    <div class="account">Justin Moore</div>
</div>
