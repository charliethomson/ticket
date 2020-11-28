const express = require('express')
const cors = require('cors')
const { getWorkorders, createWorkorder } = require('./endpoints/workorders')
const { getDevices, createDevice } = require('./endpoints/devices')
const { getCustomers, createCustomer } = require('./endpoints/customers')
const { getStores, createStore } = require('./endpoints/stores')
const { getNotes, createNote } = require('./endpoints/notes')
const { getUsers } = require('./endpoints/users')
const port = 8080

const app = express()
app.use(cors())
app.use(express.json())

app.get('/api/workorders', getWorkorders)
app.get('/api/devices', getDevices)
app.get('/api/customers', getCustomers)
app.get('/api/stores', getStores)
app.get('/api/notes/:id', getNotes)
app.get('/api/users', getUsers)

app.post('/api/workorders', createWorkorder)
app.post('/api/devices', createDevice)
app.post('/api/customers', createCustomer)
app.post('/api/stores', createStore)
app.post('/api/notes', createNote)
  
app.listen(port, () => {
    console.log(`Local server listening @ http://localhost:${port}`)
})