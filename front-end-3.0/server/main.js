const express = require('express')
const cors = require('cors')
const { getWorkorders } = require('./endpoints/workorders')
const { getDevices } = require('./endpoints/devices')
const { getCustomers } = require('./endpoints/customers')
const { getStores } = require('./endpoints/stores')
const { getNotes, createNote } = require('./endpoints/notes')
const port = 8080

const app = express()
app.use(cors())
app.use(express.json())

app.get('/api/workorders', getWorkorders)
app.get('/api/devices', getDevices)
app.get('/api/customers', getCustomers)
app.get('/api/stores', getStores)
app.get('/api/notes', getNotes)
app.post('/api/notes', createNote)
  
app.listen(port, () => {
    console.log(`Local server listening @ http://localhost:${port}`)
})