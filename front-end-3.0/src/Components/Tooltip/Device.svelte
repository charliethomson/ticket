<script>
    export let workorder
    export let dateExpected

    console.log(workorder)
</script>

<div class="device tooltip">
    <div class="serial">{'Serial: ' + workorder.device.serial}</div>
    <div class="make">{'Device: ' + workorder.device.name}</div>
    <div class="password">{'Password: ' + workorder.device.password}</div>
    <div class="TAT">{'Expected by:  ' + dateExpected}</div>
    <div class="description">{workorder.brief}</div>
</div>
