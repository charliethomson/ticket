<script>
    export let workorder
    export let statusList

    let statusesShown = false
    let indexShown = workorder.status

    // const statuses = [
    //     {
    //         status: "Awaiting Repair",
    //         color: "red",
    //     },
    //     {
    //         status: "Quality Inspection",
    //         color: "red",
    //     },
    //     {
    //         status: "Need to Order Parts",
    //         color: "red",
    //     },
    //     {
    //         status: "Awaiting Callback",
    //         color: "yellow",
    //     },
    //     {
    //         status: "Awaiting Device",
    //         color: "yellow",
    //     },
    //     {
    //         status: "Awaiting Parts",
    //         color: "yellow",
    //     },
    //     {
    //         status: "Repair in Progress",
    //         color: "blue",
    //     },
    //     {
    //         status: "Repaired",
    //         color: "green",
    //     },
    //     {
    //         status: "Unrepairable",
    //         color: "green",
    //     },
    // ]

    function showStatuses() {
        statusesShown = !statusesShown
    }
</script>

<style>
    .statuses {
        display: flex;
        flex-direction: row;
        cursor: pointer;
    }
    .active-status {
        display: inline;
        margin-left: 70px;
        border-bottom: 2px solid #f44336;
        font-weight: bold;
        margin-right: 20px;
    }
    .status-container {
        display: flex;
    }
    .status {
        visibility: hidden;
        margin-right: 10px;
        user-select: none;
    }
    .shown {
        visibility: visible;
    }
    /* Colors for statuses*/
    .blue {
        border-bottom: 2px solid #2196f3;
    }
    .red {
        border-bottom: 2px solid #f44336;
    }
    .yellow {
        border-bottom: 2px solid #ffeb3b;
    }
    .green {
        border-bottom: 2px solid #388e3c;
    }
    .blue-hover:hover {
        border-bottom: 2px solid #2196f3;
    }
    .red-hover:hover {
        border-bottom: 2px solid #f44336;
    }
    .yellow-hover:hover {
        border-bottom: 2px solid #ffeb3b;
    }
    .green-hover:hover {
        border-bottom: 2px solid #388e3c;
    }
    /* Colors for statuses */
</style>

<div class="statuses">
    <div
        class={'active-status ' + statusList[indexShown].color}
        on:click={showStatuses}>
        {statusList[indexShown].status}
    </div>

    {#if statusesShown}
        <div class="status-container">
            {#each statusList as { status, color }, i}
                {#if i != indexShown}
                    <div
                        class={'status ' + color + '-hover' + (statusesShown ? ' shown' : '')}
                        on:click={() => {
                            indexShown = i
                            workorder.status = i
                            showStatuses()
                        }}>
                        {status}
                    </div>
                {/if}
            {/each}
        </div>
    {/if}
</div>
