<script>
    import { storeSubmitted } from "../stores"

    let storeCreated = false

    function handleCreate() {
        storeCreated = !storeCreated
        alert("storeCreated is " + storeCreated)
    }

    function handleSubmit() {
        $storeSubmitted = !$storeSubmitted
        alert("$storeSubmitted is " + $storeSubmitted)
    }
</script>

<style>
    * {
        color: white;
        font-family: "Fira Sans", sans-serif;
        font-size: 33px;
        font-weight: bold;
    }

    label {
        font-weight: normal;
        font-size: 20px;
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

    .create-store {
        width: 500px;
        background-color: #212121;
        border-radius: 10px;
        padding: 40px;
        margin-bottom: 20px;
    }
    .create-store > .title {
        text-align: center;
    }
    .address {
        display: flex;
        flex-direction: row;
    }
    .address > * {
        margin-left: 5px;
        margin-right: 5px;
    }
    .city {
        flex-grow: 3;
        flex-basis: 0;
    }
    .state {
        flex-grow: 1;
        flex-basis: 0;
    }
    .zip {
        flex-grow: 1;
        flex-basis: 0;
    }

    .select-store {
        width: 468px;
        height: 198px;
        background-color: #212121;
        border-radius: 10px;
        padding: 40px;
    }
    .title {
        text-align: center;
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

<div class="container">
    {#if storeCreated}
        <div>
            <div class="create-store">
                <div class="title">Store</div>
                <label for="name">Name:</label>
                <input type="text" />
                <label for="phone">Phone #:</label>
                <input type="text" />
                <label for="email">Email:</label>
                <input type="text" />
                <label for="address">Address:</label>
                <input type="text" />
                <div class="address">
                    <div class="city">
                        <label for="city">City:</label>
                        <input type="text" />
                    </div>
                    <div class="state">
                        <label for="state">State:</label>
                        <input type="text" />
                    </div>
                    <div class="zip">
                        <label for="zip">Zip:</label>
                        <input type="text" />
                    </div>
                </div>
            </div>
            <div class="button" on:click={handleSubmit}>
                <span>Submit</span>
            </div>
        </div>
        <div class="or">OR</div>

        <div class="button" on:click={handleCreate}>
            <span>Search for a store</span>
        </div>
    {:else}
        <div class="select-store">
            <div class="title">Select a store</div>
            <input type="text" />
        </div>
        <div class="or">OR</div>
        <div class="button" on:click={handleCreate}>
            <span>Create a store</span>
        </div>
    {/if}
</div>
