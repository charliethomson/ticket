const app = require('express')()
const { workorders } = require('./endpoints/workorders')
const { devices } = require('./endpoints/devices')
const { customers } = require('./endpoints/customers')
const { stores } = require('./endpoints/stores')
const { notes } = require('./endpoints/notes')
const port = 8080

app.get('/api/workorders', workorders)
app.get('/api/devices', devices)
app.get('/api/customers', customers)
app.get('/api/stores', stores)
app.get('/api/notes', notes)
  
app.listen(port, () => {
    console.log(`Local server listening @ http://localhost:${port}`)
})