exports.getDevices = (req, res) =>
    res.status(200).send(JSON.stringify({
        ok: true,
        message: [
            {
                serial: "123",
                name: "123",
                password: "123",
                customer_id: 1,
            },
        ]
    }))

exports.createDevice = (req, res) =>
    res.status(200).send(JSON.stringify(req.body))