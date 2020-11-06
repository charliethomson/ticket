<script>
    import Customer from "../Tooltip/Customer.svelte"
    import Device from "../Tooltip/Device.svelte"

    export let workorder
    export let dateExpected
    export let tooltip
</script>

<style>
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

<div class="tooltip">
    <svelte:component this={tooltip} {workorder} {dateExpected} />
</div>
