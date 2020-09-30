<script>
    // export let search
    export let title
    export let create
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
    .title {
        text-align: center;
    }
    .search {
        width: 468px;
        height: 198px;
        background-color: #212121;
        border-radius: 10px;
        padding: 40px;
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

<div class="search">
    <div class="title">Search for a {title}</div>
    <input type="text" />
</div>
<div class="or">OR</div>
<div class="button" on:click={create}><span>Create a {title}</span></div>
