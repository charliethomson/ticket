exports.getUsers = (req, res) =>
    res.status(200).send(JSON.stringify({
        ok: true,
        message: [
          {
            id: 1,
            first_name: "Charlie",
            last_name: "Thomson",
            email: "c.thomson@ubreakifix.com"
          },
          {
            id: 2,
            first_name: "Justin",
            last_name: "Moore",
            email: "j.moore@ubreakifix.com"
          },
        ] 
    }))