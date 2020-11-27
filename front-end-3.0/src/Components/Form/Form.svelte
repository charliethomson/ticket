<script>
    import { createStore, createCustomer, createDevice, createWorkorder} from '../../utils'
    import { form } from './formData'
    import Container from "../Helpers/Container.svelte"
    import Input from './Input.svelte'
    import { alertContent } from "../../stores"

    const letter = s => (/^[a-z ,.'-]+$/i).test(s)
    const state = s => (/^[a-z]+$/i).test(s)
    const phone = s => (/^(\+\d{1,2}\s)?\(?\d{3}\)?[\s.-]\d{3}[\s.-]\d{4}$/).test(s)
    const email = s => (/^[^\s@]+@[^\s@]+\.[^\s@]+$/).test(s)
    const address = s => (/^\d+\w*\s*(?:(?:[\-\/]?\s*)?\d*(?:\s*\d+\/\s*)?\d+)?\s+/).test(s)
    const zip = s => (/^\d{5}$/).test(s)
    const notWhitespace = s => (/^(?!\s*$).+/).test(s)

    $: disabled = !letter(form.store.name)
               || !phone(form.store.phone_number)
               || !email(form.store.email)
               || !address(form.store.address)
               || !letter(form.store.city)
               || !state(form.store.state)
               || !zip(form.store.zip)
               || !letter(form.customer.first_name)
               || !letter(form.customer.last_name)
               || !phone(form.customer.phone_number)
               || !email(form.customer.email_address)
               || !notWhitespace(form.device.serial)
               || !notWhitespace(form.device.name)
               || !notWhitespace(form.device.password)
               || !notWhitespace(form.additional.brief)
               || !notWhitespace(form.additional.quoted_time)

    $: buttonText = disabled ? "Please finish filling out the form to continue." : "Create Workorder"

    // FIXME: fix workorder creation for prod
    async function handleCreate() {
        if (disabled) {
            $alertContent = "Your input in the form is invalid."
        } else {
            $alertContent = ""
            buttonText = "Workorders are loading..."
            form.device.customer_id = (await createCustomer(form.customer)).message
            await createWorkorder({
                origin: (await createStore(form.store)).message,
                customer: form.device.customer_id,
                device: (await createDevice(form.device)).message,
                brief: form.additional.brief,
            }).then(res => {
                window.location.hash = "#/"
            })
        }
    }
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
    button {
        text-align: center;
        width: 100%;
        font-weight: bold;
        cursor: pointer;
        padding: 20px;
        border-radius: 10px;
        background-color: #388e3c;
        font-size: 16px;
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
                    <Input label="Point of contact" bind:value={form.store.name} valid={letter} />
                    <Input label="Phone #:" bind:value={form.store.phone_number} valid={phone} />
                    <Input label="Email" bind:value={form.store.email} valid={email} />
                    <Input label="Address" bind:value={form.store.address} valid={address} />
                    <Input label="City" bind:value={form.store.city} valid={letter} />
                    <Input label="State" bind:value={form.store.state} valid={state} maxlength="2" />
                    <Input label="Zip" bind:value={form.store.zip} valid={zip} maxlength="5" />
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
                    <Input label="First Name" bind:value={form.customer.first_name} valid={letter} />
                    <Input label="Last Name" bind:value={form.customer.last_name} valid={letter} />
                    <Input label="Phone #" bind:value={form.customer.phone_number} valid={phone} />
                    <Input label="Email" bind:value={form.customer.email_address} valid={email} />
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
                    <Input label="Serial/IMEI" bind:value={form.device.serial} valid={notWhitespace} />
                    <Input label="Make/Model" bind:value={form.device.name} valid={notWhitespace} />
                    <Input label="Password" bind:value={form.device.password} valid={notWhitespace} />
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
                    <Input label="Brief Description" bind:value={form.additional.brief} valid={notWhitespace} size="55" />
                </div>
                <div class="time-quote">
                    <Input label="Time quoted" bind:value={form.additional.quoted_time} valid={notWhitespace} size="55" />
                </div>
            </div>
                <button {disabled} on:click={handleCreate}>{buttonText}</button>
        </div>
    </div>
</Container>
