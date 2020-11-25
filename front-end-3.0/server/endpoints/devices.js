exports.devices = (req, res) =>
    res.status(200).send(JSON.stringify(
        {
            serial: "123",
            name: "123",
            password: "123",
            customer_id: null,
        },
    ))