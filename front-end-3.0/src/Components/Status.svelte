<script>
    export const statuses = [
        {
            status: "Awaiting Repair",
            color: "red",
        },
        {
            status: "Repair in Progress",
            color: "blue",
        },
        {
            status: "Repaired",
            color: "green",
        },
        {
            status: "Unrepairable",
            color: "green",
        },
        {
            status: "Awaiting Callback",
            color: "yellow",
        },
        {
            status: "Awaiting Device",
            color: "yellow",
        },
        {
            status: "Awaiting Parts",
            color: "yellow",
        },
        {
            status: "Quality Inspection",
            color: "red",
        },
        {
            status: "Need to Order Parts",
            color: "red",
        },
    ];

    export const travelStatuses = [
        {
            status: "Awaiting Shipment",
            color: "red",
        },
        {
            status: "En Route",
            color: "yellow",
        },
        {
            status: "Arrived",
            color: "green",
        },
    ];
    let statusesShown = false;
    let indexShown = 0;
    function showStatuses() {
        statusesShown = !statusesShown;
    }
</script>

<style>
    .container {
        margin-top: 50px;
        display: flex;
        justify-content: space-between;
    }
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
    .delivery {
        color: #388e3c;
    }
    .location {
        display: flex;
        font-weight: bold;
        margin-top: 10px;
        margin-right: 70px;
    }
    .storage {
        margin-left: 5px;
    }
</style>

<div class="container">
    <div class="statuses">
        <div
            class={'active-status ' + statuses[indexShown].color}
            on:click={showStatuses}>
            {statuses[indexShown].status}
        </div>

        <div class="status-container">
            {#each statuses as { status, color }, i}
                {#if i != indexShown}
                    <div
                        class={'status ' + color + '-hover' + (statusesShown ? ' shown' : '')}
                        on:click={() => {
                            indexShown = i;
                            showStatuses();
                        }}>
                        {status}
                    </div>
                {/if}
            {/each}
        </div>
    </div>
    <div class="location">
        <div class="delivery">Arrived</div>
        <div class="storage">C7</div>
    </div>
</div>
