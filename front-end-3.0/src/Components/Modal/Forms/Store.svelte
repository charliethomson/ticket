<script>
    export let handleEnd
    export let handleClick

    const letterRegex = /^[a-z ,.'-]+$/i
    const stateRegex = /^[a-z]+$/i
    const phoneRegex = /^(\+\d{1,2}\s)?\(?\d{3}\)?[\s.-]\d{3}[\s.-]\d{4}$/
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    const addressRegex = /^\d+\s[A-z]+\s[A-z]+/g
    const zipRegex = /^\d{5}$/

    let store = {
        name: "",
        phone: "",
        email: "",
        address: "",
        city: "",
        state: "",
        zip: null,
    }

    $: isNameValid = letterRegex.exec(store.name) ? true : false
    $: isPhoneValid = phoneRegex.exec(store.phone) ? true : false
    $: isEmailValid = emailRegex.exec(store.email) ? true : false
    $: isAddressValid = addressRegex.exec(store.address) ? true : false
    $: isCityValid = letterRegex.exec(store.city) ? true : false
    $: isStateValid = stateRegex.exec(store.state) ? true : false
    $: isZipValid = zipRegex.exec(store.zip) ? true : false

    function validation() {
        if (
            isNameValid &&
            isPhoneValid &&
            isEmailValid &&
            isAddressValid &&
            isCityValid &&
            isStateValid &&
            isZipValid
        ) {
            return true
        } else {
            return false
        }
    }

    function handleFunctions() {
        if (validation() == false) {
            alert("Check your inputs on the form")
        } else {
            handleClick()
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

        margin-bottom: 10px;
        width: 100%;
        border: 0;
        outline: 0;
        background: transparent;
        border-bottom: 2px solid #e3e3e3;
    }
    .invalid {
        border-bottom: 2px solid #f44336;
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

    .address {
        display: flex;
        flex-direction: row;
    }
    .city {
        flex-grow: 3;
        flex-basis: 0;
        margin-right: 20px;
    }
    .state {
        flex-grow: 1;
        flex-basis: 0;
        margin-right: 20px;
    }
    .zip {
        flex-grow: 1;
        flex-basis: 0;
    }
    .create {
        height: 400px;
    }
</style>

<div class="title-bar">
    <div class="title">Store</div>
    <div class="exit" on:click={handleEnd}>X</div>
</div>
<div class="box">
    <div class="name">Search</div>
    <input type="text" />
</div>
<div class="or">- OR -</div>

<div class="create box">
    <div class="name">Create</div>

    <form action="#0" method="post">
        <label for="name">Point of contact: </label>
        <input
            type="text"
            bind:value={store.name}
            class={isNameValid ? '' : 'invalid '}
            required />

        <label for="phone">Phone #: </label>
        <input
            type="text"
            bind:value={store.phone}
            class="{isPhoneValid ? '' : 'invalid '}}required" />

        <label for="email">Email: </label>
        <input
            type="text"
            bind:value={store.email}
            class={isEmailValid ? '' : 'invalid '}
            required />

        <label for="address">Address: </label>
        <input
            type="text"
            bind:value={store.address}
            class={isAddressValid ? '' : 'invalid '}
            required />

        <div class="address">
            <div class="city">
                <label for="city">City: </label>
                <input
                    type="text"
                    bind:value={store.city}
                    class="{isCityValid ? '' : 'invalid '}required" />
            </div>
            <div class="state">
                <label for="state">State: </label>
                <input
                    type="text"
                    bind:value={store.state}
                    minlength="2"
                    maxlength="2"
                    class={isStateValid ? '' : 'invalid '}
                    required />
            </div>
            <div class="zip">
                <label for="zip">Zip: </label>
                <input
                    type="text"
                    bind:value={store.zip}
                    minlength="5"
                    maxlength="5"
                    class={isZipValid ? '' : 'invalid '}
                    required />
            </div>
        </div>
    </form>
</div>
<div class="button" on:click={handleFunctions}>Submit</div>
