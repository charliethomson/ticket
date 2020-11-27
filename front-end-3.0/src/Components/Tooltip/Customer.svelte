<script>
    // TODO: get data from API
    let customer = {
        first_name: "asd",
        last_name: "asd",
        phone_number: "555-555-1234",
        email_address: "a@a.a",
    }

    // OnMount sumbmit a request to the service that grabs the customer @ the activeWOid
</script>

<div class="customer tooltip">
    <div class="name">
        {'Name : ' + customer.first_name + ' ' + customer.last_name}
    </div>

    <div class="phone">{'Phone #: ' + customer.phone_number}</div>
    <div class="email">{'Email: ' + customer.email}</div>
</div>
