<script>
    import Text from "./Text.svelte"

    import { seeModal } from "../../stores"

    let activeIndex = 1

    function handleClick() {
        activeIndex++
    }

    function handleEnd() {
        $seeModal = false
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
    .create {
        height: 400px;
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
</style>

{#if activeIndex == 1}
    <div class="container">
        <div class="bar">
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
            <Text label={'Name'} />
            <Text label={'Phone #'} />
            <Text label={'Email'} />
            <Text label={'Address'} />
            <div class="address">
                <div class="city">
                    <Text label={'City'} />
                </div>
                <div class="state">
                    <Text label={'State'} />
                </div>
                <div class="zip">
                    <Text label={'Zip'} />
                </div>
            </div>
        </div>
        <div class="button" on:click={handleClick}>Submit</div>
    </div>
{:else if activeIndex == 2}
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
            <Text label={'First name'} />
            <Text label={'Last name'} />
            <Text label={'Phone #'} />
            <Text label={'Email'} />
        </div>
    </div>

    <div class="button" on:click={handleClick}>Submit</div>
{:else}
    <div class="container">
        <div class="bar">
            <div class="title">Device</div>
            <div class="exit" on:click={handleEnd}>X</div>
        </div>
        <div class="box">
            <div class="name">Search</div>
            <input type="text" />
        </div>

        <div class="or">- OR -</div>
        <div class="create box">
            <div class="name">Create</div>
            <Text label={'Serial/IMEI'} />
            <Text label={'Make/Model'} />
            <Text label={'Time quote'} />
            <Text label={'Brief Description'} />
            <Text label={'Password'} />
        </div>
    </div>
    <div class="button" on:click={handleEnd}>Submit</div>
{/if}

<!-- Store -->

<!-- Store -->
