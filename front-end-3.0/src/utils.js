const baseUrl = `http://${process.env.isProd ? "offsite.repair" : "localhost:8080"}/api/`

const GET = async (url) => {
    const response = await fetch(baseUrl + url)   
    const data = await response.json()
    return data
}

export const getWorkorders = async () => await GET('workorders')
export const getDevices = async () => await GET('devices')
export const getCustomer = async () => await GET('customer')
export const getStores = async () => await GET('stores')
export const getNotes = async () => await GET('notes')
