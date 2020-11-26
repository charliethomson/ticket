exports.getCustomers = (req, res) =>
    res.status(200).send(JSON.stringify({
        ok: true,
        message: [
            {
                first_name: "asd",
                last_name: "asd",
                phone_number: "555-555-1234",
                email_address: "a@a.a",
            },
        ]
    }))

exports.createCustomer = (req, res) =>
    res.status(200).send(JSON.stringify(req.body))