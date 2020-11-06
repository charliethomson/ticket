<script>
    export let workorder
</script>

<div class="customer tooltip">
    <div class="name">
        {'Name : ' + workorder.customer.first_name + ' ' + workorder.customer.last_name}
    </div>

    <div class="phone">{'Phone #: ' + workorder.customer.phone_number}</div>
    <div class="email">{'Email: ' + workorder.customer.email}</div>
</div>
