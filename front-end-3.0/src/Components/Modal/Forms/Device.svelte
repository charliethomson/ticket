<script>
    export let handleEnd
    export let handleClick

    let device = {
        serial: "",
        model: "",
        time_quote: "",
        brief: "",
        password: "",
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

        <label for="name">Serial/IMEI: </label>
        <input type="text" bind:value={device.serial} required />

        <label for="name">Make/Model: </label>
        <input type="text" bind:value={device.model} required />

        <label for="name">Time quote: </label>
        <input type="text" bind:value={device.time_quote} required />

        <label for="name">Brief Description: </label>
        <input type="text" bind:value={device.brief} required />

        <label for="name">Password: </label>
        <input type="text" bind:value={device.password} required />
    </div>
</div>
<div class="button" on:click={handleEnd}>Submit</div>
