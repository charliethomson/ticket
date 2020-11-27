table! {
    customers (id) {
        id -> Bigint,
        first_name -> Varchar,
        last_name -> Varchar,
        phone_number -> Varchar,
        email_address -> Varchar,
    }
}

table! {
    devices (id) {
        id -> Bigint,
        serial_no -> Varchar,
        device_name -> Varchar,
        customer -> Bigint,
        password -> Nullable<Text>,
    }
}

table! {
    notes (id) {
        id -> Bigint,
        workorder_id -> Bigint,
        contents -> Text,
        user -> Bigint,
        posted -> Integer,
        next_update -> Nullable<Integer>,
    }
}

table! {
    stores (id) {
        id -> Bigint,
        contact_name -> Varchar,
        phone_number -> Varchar,
        email_address -> Varchar,
        address -> Varchar,
        city -> Varchar,
        state -> Varchar,
        zip -> Integer,
    }
}

table! {
    users (id) {
        id -> Bigint,
        google_id -> Nullable<Binary>,
        portal_id -> Nullable<Bigint>,
        first_name -> Varchar,
        last_name -> Varchar,
        email_address -> Varchar,
    }
}

table! {
    workorders (id) {
        id -> Bigint,
        active -> Bool,
        origin -> Bigint,
        created -> Integer,
        quoted -> Nullable<Integer>,
        workorder_status -> Integer,
        travel_status -> Integer,
        location -> Nullable<Varchar>,
        customer -> Bigint,
        device -> Bigint,
        brief -> Varchar,
    }
}

joinable!(devices -> customers (customer));
joinable!(notes -> users (user));
joinable!(notes -> workorders (workorder_id));
joinable!(workorders -> customers (customer));
joinable!(workorders -> devices (device));
joinable!(workorders -> stores (origin));

allow_tables_to_appear_in_same_query!(
    customers,
    devices,
    notes,
    stores,
    users,
    workorders,
);
