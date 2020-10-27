<script>
    import { isFormValid } from "../../../stores"
    export let handleEnd
    export let handleClick

    const letterRegex = /^[a-z ,.'-]+$/i
    const phoneRegex = /^(\+\d{1,2}\s)?\(?\d{3}\)?[\s.-]\d{3}[\s.-]\d{4}$/
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/

    let response
    let customer = {
        first_name: "",
        last_name: "",
        phone_number: "",
        email_address: "",
    }

    $: isFirstNameValid = letterRegex.exec(customer.first_name) ? true : false
    $: isLastNameValid = letterRegex.exec(customer.last_name) ? true : false
    $: isPhoneValid = phoneRegex.exec(customer.phone_number) ? true : false
    $: isEmailValid = emailRegex.exec(customer.email_address) ? true : false

    function validation() {
        if (
            isFirstNameValid &&
            isLastNameValid &&
            isPhoneValid &&
            isEmailValid
        ) {
            return true
        } else {
            $isFormValid = false
            return false
        }
    }
    async function postData(url = "", data = {}) {
        const response = await fetch(url, {
            method: "POST",
            mode: "cors",
            cache: "no-cache",
            credentials: "same-origin",
            headers: {
                "Content-Type": "application/json",
            },
            redirect: "follow",
            referrerPolicy: "no-referrer",
            body: JSON.stringify(data),
        })
        return response.json()
    }

    function handleFunctions() {
        if (validation() == false) {
            return
        } else {
            handleClick()
            response = postData("http://offsite.repair/api/customers", customer)
        }
    }
</script>

<style>
    .exit {
        display: flex;
        position: fixed;
        left: 540px;
        top: 20px;

        justify-content: center;
        align-items: center;
        width: 40px;
        height: 40px;
        cursor: pointer;

        font-weight: bold;
        border-radius: 10px;
        background-color: #f44336;
    }
    .title {
        font-size: 35px;
        font-weight: bold;
        margin-bottom: 20px;
        text-align: center;
    }
    .invalid {
        border-bottom: 2px solid #f44336;
    }

    .box {
        padding: 20px;
        border: 2px solid #e3e3e3;
        border-radius: 10px;
    }
    .name {
        font-size: 20px;
        font-weight: bold;
        text-align: center;
    }
    input {
        font-family: inherit;
        font-weight: lighter;
        color: white;

        padding-bottom: 5px;
        margin-bottom: 10px;
        width: 100%;
        border: 0;
        outline: 0;
        background: transparent;
        border-bottom: 2px solid #e3e3e3;
    }
    .or {
        font-size: 25px;
        font-weight: bold;
        text-align: center;
        margin-top: 20px;
        margin-bottom: 20px;
    }
    .button {
        font-weight: bold;
        font-size: 20px;
        text-align: center;
        cursor: pointer;
        padding: 10px;
        margin-top: 20px;
        border-radius: 10px;
        background-color: #388e3c;
    }
    .create {
        height: 400px;
    }
</style>

<div class="container">
    <div class="bar">
        <div class="title">Customer</div>
        <div class="exit" on:click={handleEnd}>X</div>
    </div>
    <div class="box">
        <div class="name">Search</div>
        <input type="text" />
    </div>

    <div class="or">- OR -</div>

    <div class="create box">
        <div class="name">Create</div>

        <label for="name">First Name: </label>
        <input
            type="text"
            bind:value={customer.first_name}
            class={isFirstNameValid ? '' : 'invalid '}
            required />

        <label for="name">Last Name: </label>
        <input
            type="text"
            bind:value={customer.last_name}
            class={isLastNameValid ? '' : 'invalid '}
            required />

        <label for="name">Phone #: </label>
        <input
            type="text"
            bind:value={customer.phone_number}
            class={isPhoneValid ? '' : 'invalid '}
            required />

        <label for="name">Email: </label>
        <input
            type="text"
            bind:value={customer.email_address}
            class={isEmailValid ? '' : 'invalid '}
            required />
    </div>
</div>

<div class="button" on:click={handleFunctions}>Submit</div>
