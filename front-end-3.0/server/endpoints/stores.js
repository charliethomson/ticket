exports.stores = (req, res) =>
    res.send(
        {
            name: "Justin's Store",
            phone_number: "540-308-3687",
            email: "vexedrecks@gmail.com",
            address: "5603 hickory tree lane",
            city: "mineral",
            state: "va",
            zip: "23117",
        },
    )