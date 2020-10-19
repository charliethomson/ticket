<script>
    export let customer

    import Button from "./Button.svelte"

    import { workorderExpanded, seeModal } from "../stores"

    let deviceShown = false
    let customerShown = false
    let contentShown = false

    $: console.log("device shown " + deviceShown)
    $: console.log("customer shown " + customerShown)
    $: console.log("content shown " + contentShown)

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
        if (contentShown === true) {
            contentShown = false
            if (customerShown === true) customerShown = false
            if (deviceShown === true) deviceShown = false
        }
    }
    function showDevice() {
        if (customerShown == true) {
            deviceShown = !deviceShown
            customerShown = !customerShown
        } else {
            contentShown = !contentShown
            deviceShown = !deviceShown
        }
    }
    function showCustomer() {
        if (deviceShown == true) {
            customerShown = !customerShown
            deviceShown = !deviceShown
        } else {
            contentShown = !contentShown
            customerShown = !customerShown
        }
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
    .tooltip {
        position: absolute;
        width: 300px;
        padding: 20px;
        top: 75px;
        left: 375px;
        text-align: center;
        background-color: #212121;
        border-radius: 10px;
        color: #e3e3e3;
        box-shadow: 4px 4px 10px 10px rgba(0, 0, 0, 0.5);
    }
</style>

<div class="nav">
    {#if !$workorderExpanded}
        <div class="buttons">
            <Button content="Create Workorder" handleClick={goToCreate} />
            <Button content="Repair Queue" handleClick={goToRepairQ} />
            <Button content="In Progress" handleClick={goToInProgress} />
        </div>
    {:else}
        <div class="buttons">
            <Button content="Create Workorder" handleClick={goToCreate} />
            <Button content="View Workorders" handleClick={viewWorkorders} />
            <Button content="Device" handleClick={showDevice} />
            <Button content="Customer" handleClick={showCustomer} />
            {#if contentShown}
                <div class="content">
                    {#if deviceShown && !customerShown}
                        <div class="device tooltip">
                            <div class="serial">Serial: C12358ABC34</div>
                            <div class="make">Device: Macbook Pro</div>
                            <div class="password">Password: Cocks12</div>
                            <div class="TAT">Expected by: 10/25/2020</div>
                            <div class="description">
                                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
                            </div>
                        </div>
                    {/if}
                    {#if customerShown && !deviceShown}
                        <div class="customer tooltip">
                            <div class="first">Colton</div>
                            <div class="last">Hobbs</div>
                            <div class="phone">540-371-3349</div>
                            <div class="email">c.hobbs@ubreakifix.com</div>
                        </div>
                    {/if}
                    <!-- <Text label={'First name'} />
            <Text label={'Last name'} />
            <Text label={'Phone #'} />
            <Text label={'Email'} /> -->
                </div>
            {/if}
        </div>
    {/if}

    <div class="account">Justin Moore</div>
</div>
