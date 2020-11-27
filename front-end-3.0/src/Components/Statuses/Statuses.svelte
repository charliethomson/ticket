<script>
    import { onMount } from 'svelte'
    import { getWorkorder } from '../../utils'
    export let statuses = []
    export let id = 0
    let statusesShown = false
    let workorder = {}

    function showStatuses() {
        statusesShown = !statusesShown
    }

    onMount(async () => {
        workorder = await getWorkorder(id)
    })
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
    {#if statuses[workorder?.status]}
        <div
            class={'active-status ' + statuses[workorder?.status]?.color}
            on:click={showStatuses}>
            {statuses[workorder?.status]?.status}
        </div>
    {/if}

    {#if statusesShown}
        <div class="status-container">
            {#each statuses as { status, color }, i}
                {#if i != workorder.status}
                    <div
                        class={'status ' + color + '-hover' + (statusesShown ? ' shown' : '')}
                        on:click={() => {
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
