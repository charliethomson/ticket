<style>
    .alert {
        font-family: "Montserrat", sans-serif;
        display: flex;
        justify-content: center;
        align-items: center;
        position: fixed;
        bottom: 10px;
        right: 10px;
        border-radius: 10px;
        padding: 40px;
        width: 350px;
        height: 100px;
        font-weight: bold;
        background-color: rgba(244, 67, 54, 0.4);
        color: #e3e3e3;
        box-shadow: 4px 4px 10px 10px rgba(0, 0, 0, 0.5);
    }
</style>

<div class="alert">
    <slot />
</div>
