const prod = process.env.isProd || false // change to true to test prod server
const baseUrl = `http://${prod ? "offsite.repair" : "localhost:8080"}/api/`
const delay = x => new Promise(resolve => prod ? resolve(x) : setTimeout(() => resolve(x), 1000))

// HTTP REQUESTS
const GET = (url) =>
    fetch(baseUrl + url)
        .then(data => data.json())
        .then(delay)
const POST = (url, body) => 
    fetch(baseUrl + url, {
        method: "POST",
        mode: "cors",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(body),
    })
        .then(data => data.json())
        .then(delay)
// REQUEST -> GET ALL
export const getWorkorders = async () => (await GET('workorders')).message
export const getDevices = async () => (await GET('devices')).message
export const getCustomers = async () => (await GET('customer')).message
export const getStores = async () => (await GET('stores')).message
//TODO: backend: getNotes should accept an ID and only return workorders with that ID
export const getNotes = async (id) => (await GET('notes/'+ id)).message
export const getUsers = async () => (await GET('users')).message
// REQUEST -> CREATE
export const createWorkorder = async (body) => (await POST('workorders', body)).message
export const createDevice = async (body) => (await POST('devices', body)).message
export const createCustomer = async (body) => (await POST('customers', body)).message
export const createStore = async (body) => (await POST('stores', body)).message
export const createNote = async (body) => (await POST('notes', body)).message
// REQUEST -> GET ONE
export const getWorkorder = (id) => getWorkorders().then(workorders => workorders.find(workorder => workorder.workorder_id == id))
export const getDevice = (id) => getDevices().then(devices => devices.find(device => device.id == id))
export const getCustomer = (id) => getCustomers().then(customers => customers.find(customer => customer.id == id))
export const getStore = (id) => getStores().then(stores => stores.find(store => store.id == id))
// TODO: backend: notes should have unique id's
// export const getNote = async (id) => await getNotes().then(notes => notes.find(note => note.id == id))
export const getUser = async (id) => await getUsers().then(users => users.find(user => user.id == id))