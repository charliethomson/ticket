<script>
    import Input from "./Input.svelte"

    // TODO: Dropdown menu for finding things when search
</script>

<style>
    .container {
        font-family: "Montserrat", sans-serif;
    }
    .modal {
        display: flex;
        flex-direction: column;

        border-radius: 10px;
        padding: 40px;
        width: 600px;
        height: 800px;
        background-color: #212121;
        color: #e3e3e3;
        box-shadow: 4px 4px 10px 10px rgba(0, 0, 0, 0.5);
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
    }
</style>

<div class="container">
    <div class="modal">
        <Input />
    </div>
</div>
