exports.getNotes = (req, res) =>
    res.status(200).send(JSON.stringify({
        ok: true,
        message: [
            {
                user: 1,
                contents:
                    "this is a workorder note, this is a workorder note, this is a workorder note, this is a workorder note",
                created: 1605187812,
            },
        ]
    }))

exports.createNote = (req, res) =>
    res.status(200).send(JSON.stringify({
        ok: true,
        message: req.body
    }))

