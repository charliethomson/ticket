<script>
    import { isFormValid } from "../../../stores"

    export let handleEnd
    export let handleClick

    const notWhitespaceRegex = /^(?!\s*$).+/

    let response
    let device = {
        serial: "",
        model: "",
        time_quote: "",
        brief: "",
        password: "",
    }

    $: isSerialValid = notWhitespaceRegex.exec(device.serial) ? true : false
    $: isModelValid = notWhitespaceRegex.exec(device.model) ? true : false
    $: isTimeValid = notWhitespaceRegex.exec(device.time_quote) ? true : false
    $: isBriefValid = notWhitespaceRegex.exec(device.brief) ? true : false
    $: isPasswordValid = notWhitespaceRegex.exec(device.password) ? true : false

    function validation() {
        if (
            isSerialValid &&
            isModelValid &&
            isTimeValid &&
            isBriefValid &&
            isPasswordValid
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
            handleEnd()
            // TODO: Talk to charlie about this shit lmfao
            // We are on completely different pages ab devices >:-)
            response = postData("http://offsite.repair/api/devices", device)
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
        <input
            type="text"
            bind:value={device.serial}
            class={isSerialValid ? '' : 'invalid '}
            required />

        <label for="name">Make/Model: </label>
        <input
            type="text"
            bind:value={device.model}
            class={isModelValid ? '' : 'invalid '}
            required />

        <label for="name">Time quote: </label>
        <input
            type="text"
            bind:value={device.time_quote}
            class={isTimeValid ? '' : 'invalid '}
            required />
        <!-- TODO: Need to do a calender thing for this -->

        <label for="name">Brief Description: </label>
        <input
            type="text"
            bind:value={device.brief}
            class={isBriefValid ? '' : 'invalid '}
            required />

        <label for="name">Password: </label>
        <input
            type="text"
            bind:value={device.password}
            class={isPasswordValid ? '' : 'invalid '}
            required />
    </div>
</div>
<div class="button" on:click={handleFunctions}>Submit</div>
