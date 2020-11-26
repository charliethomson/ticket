const baseUrl = `http://${process.env.isProd ? "offsite.repair" : "localhost:8080"}/api/`
const delay = x => new Promise(resolve => setTimeout(() => resolve(x), 1000))

// HTTP REQUESTS
const GET = (url) =>
    fetch(baseUrl + url)
        .then(data => data.json())
        .then(delay)
const POST = async (url, body) => 
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
export const getWorkorders = async () => await GET('workorders')
export const getDevices = async () => await GET('devices')
export const getCustomers = async () => await GET('customer')
export const getStores = async () => await GET('stores')
export const getNotes = async () => await GET('notes')
// REQUEST -> CREATE
export const createNote = async (body) => await POST('notes', body)
// REQUEST -> GET ONE
export const getWorkorder = async (id) => GET('workorders').then(workorders => workorders[id])
export const getDevice = async (id) => await GET('devices').then(devices => devices[id])
export const getCustomer = async (id) => await GET('customer').then(customers => customers[id])
export const getStore = async (id) => await GET('stores').then(stores => stores[id])
export const getNote = async (id) => await GET('notes').then(notes => notes[id])