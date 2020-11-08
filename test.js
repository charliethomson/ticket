
async function makePost(ext, data) {
    const url = "http://offsite.repair/api/" + ext;
    return await fetch(url, {
        method: "POST",
        mode: "cors",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
    }).then(res => res.json())
}

const customer = {
    first_name: "first_name",
    last_name: "last_name",
    phone_number: "555-555-1234",
    email_address: "email@address.com",
}

const store = {
    name: "name",
    phone_number: "555-555-5555",
    email: "email@address.com",
    address: "address",
    city: "city",
    state: "NO",
    zip: "22222",
}

const customer_response = await makePost("customers", customer);
let customer_id;
if (customer_response.ok) {
    customer_id = customer_response.message;
} else {
    throw `Failed to create customer: ${customer_response.message}`
}

const device = {
    serial: "serial",
    name: "name",
    customer_id: customer_id,
    password: "password",
}

const store_response = await makePost("stores", store);
let store_id;
if (store_response.ok) {
    store_id = store_response.message;
} else {
    throw `Failed to create store: ${store_response.message}`
}

const device_response = await makePost("devices", device);
let device_id;
if (device_response.ok) {
    device_id = device_response.message;
} else {
    throw `Failed to create device: ${device_response.message}`
}

const workorder = {
    origin: store_id,
    customer: customer_id,
    device: device_id,
    brief: "This is a test :)",
}

console.log("POSTING TO THE API: ", workorder);
const workorder_response = await makePost("workorders", workorder);
let workorder_id;
if (workorder_response.ok) {
    workorder_id = workorder_response.message;
} else {
    throw `Failed to create workorder: ${workorder_response.message}`
}
console.log(`Successfully created workorder number ${workorder_id}`);
const url = `http://offsite.repair/api/workorders?id=${workorder_id}`;
console.log(`Attempting to get the workorder back (${url})`)

const response = await fetch(url).then(res => res.json());
console.log(`Response:`, response)
if (!response.ok) {
    console.error("Failed to retrieve the workorder we inserted");
} else {
    console.log("Got a workorder, is it right?");
    console.log(response.message[0].workorder_id == workorder_id);
    console.log(response.message[0].customer.id == customer_id);
    console.log(response.message[0].origin.id == store_id);
    console.log(response.message[0].device.id == device_id);
    console.log(response.message[0].brief == "This is a test :)");
}