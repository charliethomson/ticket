insert into users (google_id, first_name, last_name, email_address)
values (
        113960148803902249233,
        "Charlie",
        "Thomson",
        "c.thomson@ubreakifix.com"
    );
insert into stores (
        contact_name,
        phone_number,
        email_address,
        address,
        city,
        state,
        zip
    )
values (
        "Test Contact",
        "5555551234",
        "test@store.com",
        "102 Road Rd",
        "Real City",
        "ST",
        "12345"
    );
insert into customers (
        first_name,
        last_name,
        phone_number,
        email_address
    )
values (
        "Test",
        "customer",
        "5551235555",
        "test@customer.com"
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
        created,
        quoted,
        workorder_status,
        travel_status,
        customer,
        device,
        brief
    )
values (
        1,
        1,
        UNIX_TIMESTAMP(),
        UNIX_TIMESTAMP(),
        0,
        0,
        1,
        1,
        "Test brief"
    );
insert into workorders (
        active,
        origin,
        created,
        quoted,
        workorder_status,
        travel_status,
        customer,
        device,
        brief
    )
values (
        0,
        1,
        UNIX_TIMESTAMP(),
        UNIX_TIMESTAMP(),
        0,
        0,
        1,
        1,
        "Inactive Workorder"
    );
insert into workorders (
        active,
        origin,
        created,
        quoted,
        workorder_status,
        travel_status,
        customer,
        device,
        brief
    )
values (
        1,
        1,
        UNIX_TIMESTAMP(),
        UNIX_TIMESTAMP(),
        6,
        0,
        1,
        1,
        "Status 6 ? Lol"
    );
insert into notes (
        workorder_id,
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