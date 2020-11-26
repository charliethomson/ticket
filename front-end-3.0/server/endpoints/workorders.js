exports.getWorkorders = (req, res) =>
    res.status(200).send(JSON.stringify([
        {
            workorder_id: 0,
            active: true,
            origin: 1,
            travel_status: 1,
            created: 1605187812,
            quoted_time: 5,
            status: 3,
            customer: {
                first_name: "Bob",
                last_name: "Black",
                phone_number: "111-111-1111",
                email_address: "bobblack@gmail.com",
            },
            device: {
                serial: "C1231234123",
                name: "Macbook Air",
                customer_id: 1,
                password: "Blahblah",
            },
            brief: "Dropped and no worky 1",
        },
        {
            workorder_id: 1,
            active: true,
            origin: 1,
            travel_status: 1,
            created: 1605187812,
            quoted_time: 5,
            status: 1,
            customer: {
                first_name: "George",
                last_name: "Washington",
                phone_number: "111-111-1111",
                email_address: "bobblack@gmail.com",
            },
            device: {
                serial: "C1231234123",
                name: "Macbook Air",
                customer_id: 1,
                password: "Blahblah",
            },
            brief: "Dropped and no worky 2",
        },
    ]))