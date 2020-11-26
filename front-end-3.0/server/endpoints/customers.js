exports.getCustomers = (req, res) =>
    res.status(200).send(JSON.stringify([
        {
            first_name: "asd",
            last_name: "asd",
            phone_number: "555-555-1234",
            email_address: "a@a.a",
        },
    ]))