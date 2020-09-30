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

    .create-device {
        width: 500px;
        background-color: #212121;
        border-radius: 10px;
        padding: 40px;
        margin-bottom: 20px;
    }

    .title {
        text-align: center;
    }
    label {
        font-weight: normal;
        font-size: 20px;
    }
</style>

<div>
    <div class="create-device">
        <div class="title">Device</div>
        <label for="serial">Serial/IMEI:</label>
        <input type="text" />
        <label for="model">Make/Model:</label>
        <input type="text" />
        <label for="time-quote">Time quote:</label>
        <input type="text" />
        <label for="description">Brief Description:</label>
        <input type="text" />
        <label for="password">Password:</label>
        <input type="text" />
    </div>
</div>
