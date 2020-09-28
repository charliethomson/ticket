<script>
    import { customerSubmitted } from "../stores"

    let customerCreated = false

    function handleCreate() {
        customerCreated = !customerCreated
        alert("customerCreated is " + customerCreated)
    }

    function handleSubmit() {
        $customerSubmitted = !$customerSubmitted
        alert("$customerSubmitted is " + $customerSubmitted)
    }
</script>

<style>
    * {
        color: white;
        font-family: "Fira Sans", sans-serif;
        font-size: 33px;
        font-weight: bold;
    }
    input {
        font-family: inherit;
        font-size: 1.3rem;
        font-weight: lighter;

        width: 100%;
        border: 0;
        outline: 0;
        background: transparent;
        border-bottom: 2px solid white;
    }
    label {
        font-weight: normal;
        font-size: 20px;
    }

    .create-customer {
        width: 500px;
        background-color: #212121;
        border-radius: 10px;
        padding: 40px;
        margin-bottom: 20px;
    }
    .title {
        text-align: center;
    }

    .select-customer {
        width: 468px;
        height: 198px;
        background-color: #212121;
        border-radius: 10px;
        padding: 40px;
    }
    .or {
        margin-top: 20px;
        margin-bottom: 20px;
        text-align: center;
    }
    .button {
        padding: 20px;
        background-color: #212121;
        border-radius: 10px;
        text-align: center;
    }
</style>

<div>
    {#if customerCreated}
        <div class="create-customer">
            <div class="title">Create a customer</div>
            <label for="first-name">First name:</label>
            <input type="text" />
            <label for="last-name">Last name:</label>
            <input type="text" />
            <label for="phone#">Phone #:</label>
            <input type="text" />
            <label for="email">Email:</label>
            <input type="text" />
        </div>
        <div class="button" on:click={handleSubmit}><span>Submit</span></div>
        <div class="or">OR</div>

        <div class="button" on:click={handleCreate}>
            <span>Select a customer</span>
        </div>
    {:else}
        <div class="select-customer">
            <div class="title">Select a customer</div>
            <input type="text" />
        </div>
        <div class="or">OR</div>
        <div class="button" on:click={handleCreate}>
            <span>Create a customer</span>
        </div>
    {/if}
</div>
