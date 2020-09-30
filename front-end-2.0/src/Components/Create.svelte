<script>
    export let title
    export let submit
    export let search
</script>

<style>
    * {
        color: white;
        font-family: "Fira Sans", sans-serif;
        font-size: 33px;
        font-weight: bold;
    }

    .container {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .create {
        width: 500px;
        background-color: #212121;
        border-radius: 10px;
        padding: 40px;
        margin-bottom: 20px;
    }

    .title {
        text-align: center;
    }

    .button {
        padding: 40px;
        background-color: #212121;
        border-radius: 10px;
        text-align: center;
    }

    .or {
        margin-left: 50px;
        margin-right: 50px;
        text-align: center;
    }
</style>

<div class="container">
    <div>
        <div class="create">
            <div class="title">{title}</div>
            <slot />
        </div>

        <div class="submit button" on:click={submit}><span>Submit</span></div>
    </div>

    <div class="or">OR</div>

    <div class="search button" on:click={search}>
        <span>Search for a store</span>
    </div>
</div>
