<script>
    import { onMount } from 'svelte'
    import { getWorkorder } from '../../utils'
    export let travelStatuses = []
    export let id = 0
    let workorder = {}

    let statusesActive = false
    let newLocation = "C7"

    function handleClick() {
        statusesActive = !statusesActive
    }

    function handleNewLocation() {
        currentLocation = newLocation
    }

    onMount(async () => {
        workorder = await getWorkorder(id)
    })
</script>

<style>
    .inactive-statuses {
        display: flex;
        font-weight: normal;
        cursor: pointer;
    }
    .green:hover {
        color: #388e3c;
    }

    .yellow:hover {
        color: #ffeb3b;
    }
    .status {
        margin-right: 10px;
    }
    .active-status {
        margin-right: 10px;
        cursor: pointer;
    }
    .active-status.green {
        color: #388e3c;
    }
    .active-status.yellow {
        color: #ffeb3b;
    }
    .location {
        font-size: 18px;
        display: flex;
        font-weight: bold;
        margin-top: 10px;
        margin-right: 50px;
    }
    .storage {
        width: 40px;
        font-size: 16px;
        margin-left: -5px;
        font-family: "Montserrat", sans-serif;
        border: none;
        background: transparent;
        color: #e3e3e3;
        opacity: 1;
        font-weight: bold;
        text-align: center;
    }

    .storage:active {
        border: 2px solid #e3e3e3;
    }
</style>

<div class="location">
    {#if travelStatuses[workorder.travel_status]}
        <div class="inactive-statuses">
            {#if statusesActive}
                {#each travelStatuses as { status, color }, i}
                    <div
                        class={'status ' + color}
                        on:click={() => {
                            workorder.travel_status = i
                            statusesActive = !statusesActive
                        }}>
                        {status}
                    </div>
                {/each}
            {/if}
        </div>
        <div
            class={'active-status ' + travelStatuses[workorder.travel_status]?.color}
            on:click={handleClick}>
            {travelStatuses[workorder.travel_status]?.status}
        </div>
        <input
            type="text"
            class="storage"
            size="1"
            bind:value={newLocation}
            on:input={handleNewLocation} />
    {/if}
</div>
