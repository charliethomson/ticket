<script>
    import Button from "./Button.svelte"

    function goToCreate() {
        alert("Creating something")
    }
    function goToRepairQ() {
        alert("Switching the queue to repairs that need to completed")
    }
    function goToInProgress() {
        alert("Switching the queue to repairs in progress")
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
    }
    .account {
        font-weight: bold;
        align-self: center;
        background-color: #212121;
        border-radius: 10px;
        padding: 20px;
        margin-top: 30px;
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
    <div class="buttons">
        <Button content="Create workorder" handleClick={goToCreate} />
        <Button content="Repair queue" handleClick={goToRepairQ} />
        <Button content="In progress" handleClick={goToInProgress} />
    </div>

    <div class="title">All workorders</div>

    <div class="account">Justin Moore</div>
</div>
