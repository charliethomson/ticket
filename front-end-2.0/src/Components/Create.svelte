<script>
    import Input from "./Input.svelte"

    export let title

    function handleCreate() {
        alert("Something has been created")
    }
</script>

<style>
    * {
        color: white;
        font-family: "Fira Sans", sans-serif;
        font-size: 33px;
        font-weight: bold;
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
        padding: 20px;
        background-color: #212121;
        border-radius: 10px;
        text-align: center;
    }
</style>

<div>
    <div class="create">
        <div class="title">{title}</div>
        <slot />
    </div>

    <div class="button" on:click={handleCreate}><span>Submit </span></div>
</div>
