exports.notes = (req, res) =>
    res.send(
        {
            user: {
                first_name: "Charlie",
                last_name: "Thomson",
            },
            contents:
                "this is a workorder note, this is a workorder note, this is a workorder note, this is a workorder note",
            created: 1605187812,
        },
    )