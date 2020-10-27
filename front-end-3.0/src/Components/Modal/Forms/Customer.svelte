<script>
    export let handleEnd
    export let handleClick

    let customer = {
        first_name: "",
        last_name: "",
        phone: "",
        email: "",
    }

    const nameRegex = /^[a-z]+$/i
    const phoneRegex = /^(\+\d{1,2}\s)?\(?\d{3}\)?[\s.-]\d{3}[\s.-]\d{4}$/
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/

    function checkInput() {
        console.log(customer.email)
        console.log(emailRegex.exec(customer.email))
    }

    function handleFunctions() {
        handleClick()
        checkInput()
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
        <input type="text" bind:value={customer.first_name} required />

        <label for="name">Last Name: </label>
        <input type="text" bind:value={customer.last_name} required />

        <label for="name">Phone #: </label>
        <input type="text" bind:value={customer.phone} required />

        <label for="name">Email: </label>
        <input type="text" bind:value={customer.email} required />
    </div>
</div>

<div class="button" on:click={handleFunctions} on:click={checkInput}>
    Submit
</div>
