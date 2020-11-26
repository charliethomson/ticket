const baseUrl = `http://${process.env.isProd ? "offsite.repair" : "localhost:8080"}/api/`
const delay = x => new Promise(resolve => setTimeout(() => resolve(x), 1000))

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

export const getWorkorders = async () => await GET('workorders')
export const getDevices = async () => await GET('devices')
export const getCustomer = async () => await GET('customer')
export const getStores = async () => await GET('stores')
export const getNotes = async () => await GET('notes')

export const createNote = async (body) => await POST('notes', body)

