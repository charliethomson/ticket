insert into users (name, phone_number)
values ("Test User", "5555551234");
insert into stores (
        store_name,
        contact_name,
        phone_number,
        email_address,
        address,
        city,
        state,
        zip
    )
values (
        "Test store",
        "Test Contact",
        "5555551234",
        "test@store.com",
        "102 Road Rd",
        "Real City",
        "Real State",
        "12345"
    );
insert into customers (
        customer_name,
        phone_number,
        email_address,
        store_id
    )
values (
        "Test customer",
        "5551235555",
        "test@customer.com",
        1
    );
insert into devices (
        serial_no,
        device_name,
        customer,
        password
    )
values (
        "0123456789",
        "Test device",
        1,
        "password1"
    );
insert into workorders (
        active,
        origin,
        travel_status,
        created,
        quoted,
        workorder_status,
        customer,
        device,
        brief
    )
values (
        1,
        1,
        "Test travel_status",
        UNIX_TIMESTAMP(),
        UNIX_TIMESTAMP(),
        "Test workorder_status",
        1,
        1,
        "Test brief"
    );
insert into notes (
        wo_key,
        contents,
        user,
        posted,
        next_update
    )
values (
        1,
        "Test note content",
        1,
        UNIX_TIMESTAMP(),
        NULL
    );