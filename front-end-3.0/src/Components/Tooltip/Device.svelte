<script>
    // TODO: get data from API
    let device = {
        serial: "123",
        name: "123",
        password: "123",
        customer_id: null,
    }

    // OnMount sumbmit a request to the service that grabs the device @ the activeWOid
</script>

<div class="device tooltip">
    <div class="serial">{'Serial: ' + device.serial}</div>
    <div class="make">{'Device: ' + device.name}</div>
    <div class="password">{'Password: ' + device.password}</div>
</div>
