<script>
    import { workorderExpanded } from "../stores"

    export let customer
    export let device
    export let description
    export let status
    export let location

    function handleClick() {
        $workorderExpanded = !$workorderExpanded
        alert("Expanding workorder to take up entire div")
    }
</script>

<style>
    .container {
        display: flex;
        justify-content: space-between;
        border-bottom: 2px solid #e3e3e3;
        padding: 10px;
        cursor: pointer;
    }
</style>

<div class="container" on:click={handleClick}>
    <div>{customer}</div>
    <div>{device}</div>
    <div>{description}</div>
    <div>{status}</div>
    <div>{location}</div>
</div>
