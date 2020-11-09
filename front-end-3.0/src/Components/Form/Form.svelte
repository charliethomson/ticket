<script>
    import Container from "../Container.svelte"
    import { isFormValid } from "../../stores"

    let store = {
        props: {
            name: "Justin",
            phone_number: "540-308-3687",
            email: "vexedrecks@gmail.com",
            address: "5603 hickory tree lane",
            city: "mineral",
            state: "va",
            zip: "23117",
        },
        response: {},
    }

    let customer = {
        props: {
            first_name: "asd",
            last_name: "asd",
            phone_number: "555-555-1234",
            email_address: "a@a.a",
        },
        response: {},
    }

    let device = {
        props: {
            serial: "123",
            name: "123",
            password: "123",
            customer_id: null,
        },
        response: {},
    }

    let additional = {
        brief: "",
        quoted_time: "",
    }

    const letterRegex = /^[a-z ,.'-]+$/i
    const stateRegex = /^[a-z]+$/i
    const phoneRegex = /^(\+\d{1,2}\s)?\(?\d{3}\)?[\s.-]\d{3}[\s.-]\d{4}$/
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    const addressRegex = /^\d+\w*\s*(?:(?:[\-\/]?\s*)?\d*(?:\s*\d+\/\s*)?\d+)?\s+/
    const zipRegex = /^\d{5}$/
    const notWhitespaceRegex = /^(?!\s*$).+/

    //Store
    $: isNameValid = letterRegex.exec(store.props.name)
    $: isStorePhoneValid = phoneRegex.exec(store.props.phone_number)
    $: isStoreEmailValid = emailRegex.exec(store.props.email)
    $: isAddressValid = addressRegex.exec(store.props.address)
    $: isCityValid = letterRegex.exec(store.props.city)
    $: isStateValid = stateRegex.exec(store.props.state)
    $: isZipValid = zipRegex.exec(store.props.zip)

    //Customer
    $: isFirstNameValid = letterRegex.exec(customer.props.first_name)
    $: isLastNameValid = letterRegex.exec(customer.props.last_name)
    $: isCustomerPhoneValid = phoneRegex.exec(customer.props.phone_number)
    $: isCustomerEmailValid = emailRegex.exec(customer.props.email_address)

    //Device
    $: isSerialValid = notWhitespaceRegex.exec(device.props.serial)
    $: isDeviceNameValid = notWhitespaceRegex.exec(device.props.name)
    $: isPasswordValid = notWhitespaceRegex.exec(device.props.password)

    //Additonal
    $: isBriefValid = notWhitespaceRegex.exec(additional.brief)
    $: isQuotedTimeValid = notWhitespaceRegex.exec(additional.quoted_time)

    function checkForm() {
        $isFormValid =
            isNameValid &&
            isStorePhoneValid &&
            isStorePhoneValid &&
            isAddressValid &&
            isCityValid &&
            isZipValid &&
            isFirstNameValid &&
            isLastNameValid &&
            isCustomerPhoneValid &&
            isCustomerEmailValid &&
            isPasswordValid &&
            isBriefValid &&
            isQuotedTimeValid
    }

    async function postData(url, data) {
        const baseUrl = "http://offsite.repair/api/"
        const response = await fetch(baseUrl + url, {
            method: "POST",
            mode: "cors",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
        })
        const res = await response.json()
        return res
    }

    async function handleCreate() {
        checkForm()
        if ($isFormValid) {
            store.response = await postData("stores", store.props)
            customer.response = await postData("customers", customer.props)
            device.props.customer_id = customer.response.message
            device.response = await postData("devices", device.props)
            const workorder = {
                origin: store.response.message,
                customer: customer.response.message,
                device: device.response.message,
                brief: additional.brief,
            }
            return await postData("workorders", workorder)
        } else {
            return
        }
    }
    $: console.log($isFormValid)
</script>

<style>
    .form {
        width: 100%;
        display: flex;
        justify-content: space-around;
    }
    .form-data {
        display: flex;
        flex-direction: column;
    }
    .title {
        font-size: 35px;
        font-weight: bold;
        text-align: center;
    }
    .bar {
        width: 100%;
        border-bottom: 2px solid #e3e3e3;
        margin-top: 20px;
    }
    .container {
        margin-top: 20px;
        display: flex;
    }
    .create {
        display: flex;
        flex-direction: column;
    }
    .create-title {
        font-size: 25px;
        text-align: center;
        margin-bottom: 20px;
        font-weight: bold;
    }
    .search {
        display: flex;
        flex-direction: column;
    }
    .search-title {
        font-size: 25px;
        text-align: center;
        margin-bottom: 20px;
        font-weight: bold;
    }
    .additional {
        margin-top: 40px;
        margin-bottom: 40px;
        display: flex;
        justify-content: center;
    }
    .alone {
        width: 100%;
    }
    .together {
        margin-right: 40px;
    }
    .brief {
        margin-right: 40px;
    }

    input {
        margin-top: 10px;
        margin-bottom: 10px;
        font-family: inherit;
        color: white;
        padding: 10px;
        background: transparent;
        outline: 0;
        width: 100%;

        border: 2px solid #e3e3e3;
        border-radius: 10px;
    }
    .invalid:focus {
        border: 2px solid red;
    }
    .valid:focus {
        border: 2px solid #388e3c;
    }

    .button {
        text-align: center;
        width: 100%;
        font-weight: bold;
        cursor: pointer;
        padding: 20px;
        border-radius: 10px;
        background-color: #388e3c;
    }
</style>

<Container>
    <div class="form">
        <div class="form-data">
            <div class="title">Store</div>
            <div class="bar" />
            <div class="container">
                <div class="create together">
                    <div class="create-title">Create</div>

                    <label for="contact">Point of contact: </label>
                    <input
                        type="text"
                        bind:value={store.props.name}
                        class={isNameValid ? 'valid' : 'invalid '} />

                    <label for="phone">Phone #: </label>
                    <input
                        type="text"
                        bind:value={store.props.phone_number}
                        class={isStorePhoneValid ? 'valid' : 'invalid '} />

                    <label for="email">Email: </label>
                    <input
                        type="text"
                        bind:value={store.props.email}
                        class={isStoreEmailValid ? 'valid' : 'invalid '} />

                    <label for="address">Address: </label>
                    <input
                        type="text"
                        bind:value={store.props.address}
                        class={isAddressValid ? 'valid' : 'invalid '} />

                    <label for="city">City: </label>
                    <input
                        type="text"
                        bind:value={store.props.city}
                        class={isCityValid ? 'valid' : 'invalid '} />

                    <label for="state">State: </label>
                    <input
                        type="text"
                        bind:value={store.props.state}
                        maxlength="2"
                        class={isStateValid ? 'valid' : 'invalid '} />

                    <label for="zip">Zip: </label>
                    <input
                        type="text"
                        maxlength="5"
                        bind:value={store.props.zip}
                        class={isZipValid ? 'valid' : 'invalid '} />
                </div>
                <div class="search">
                    <div class="search-title">Search</div>
                    <label for="search">Add an existing store:</label>
                    <input type="text" />
                </div>
            </div>
        </div>

        <div class="form-data">
            <div class="title">Customer</div>
            <div class="bar" />
            <div class="container">
                <div class="create together">
                    <div class="create-title">Create</div>

                    <label for="first-name">First Name: </label>
                    <input
                        type="text"
                        bind:value={customer.props.first_name}
                        class={isFirstNameValid ? 'valid' : 'invalid '} />

                    <label for="last-name">Last Name: </label>
                    <input
                        type="text"
                        bind:value={customer.props.last_name}
                        class={isLastNameValid ? 'valid' : 'invalid '} />

                    <label for="last-name">Phone #: </label>
                    <input
                        type="text"
                        bind:value={customer.props.phone_number}
                        class={isCustomerPhoneValid ? 'valid' : 'invalid '} />

                    <label for="last-name">Email: </label>
                    <input
                        type="text"
                        bind:value={customer.props.email_address}
                        class={isCustomerEmailValid ? 'valid' : 'invalid '} />
                </div>
                <div class="search">
                    <div class="search-title">Search</div>
                    <label for="search">Add an existing customer:</label>
                    <input type="text" />
                </div>
            </div>
        </div>

        <div class="form-data">
            <div class="title">Device</div>
            <div class="bar" />
            <div class="container">
                <div class="create">
                    <div class="create-title">Create</div>

                    <label for="serial">Serial/IMEI: </label>
                    <input
                        type="text"
                        bind:value={device.props.serial}
                        class={isSerialValid ? 'valid' : 'invalid '} />

                    <label for="make">Make/Model: </label>
                    <input
                        type="text"
                        bind:value={device.props.name}
                        class={isDeviceNameValid ? 'valid' : 'invalid '} />

                    <label for="password">Password: </label>
                    <input
                        type="text"
                        bind:value={device.props.password}
                        class={isPasswordValid ? 'valid' : 'invalid '} />
                </div>
            </div>
        </div>
    </div>
    <div class="form">
        <div class="form-data alone">
            <div class="title">Additional Information</div>
            <div class="bar" />

            <div class="additional">
                <div class="brief">
                    <label for="contact">Brief Description: </label>
                    <input
                        type="text"
                        size="55"
                        bind:value={additional.brief}
                        class={isBriefValid ? 'valid' : 'invalid '} />
                </div>

                <div class="time-quote">
                    <label for="zip">Time quoted: </label>
                    <input
                        type="text"
                        size="55"
                        bind:value={additional.quoted_time}
                        class={isQuotedTimeValid ? 'valid' : 'invalid '} />
                </div>
            </div>
            <div
                class="button"
                on:click={() => handleCreate().then((res) => console.log(res))}>
                Create Workorder
            </div>
        </div>
    </div>
</Container>
