<style>
    div {
        height: 100%;
        margin: 40px;
        margin-top: 0px;
        background-color: #212121;
        border-radius: 10px;
        padding: 40px;
        overflow-y: auto;
    }
    div::-webkit-scrollbar-track {
        box-shadow: inset 0 0 6px rgba(0, 0, 0, 0.3);
        border-radius: 10px;
        background-color: rgba(0, 0, 0, 0.3);
    }

    div::-webkit-scrollbar {
        width: 12px;
        border-radius: 10px;
        background-color: #e3e3e3;
    }

    div::-webkit-scrollbar-thumb {
        border-radius: 10px;
        box-shadow: inset 0 0 6px rgba(0, 0, 0, 0.3);
        background-color: rgba(0, 0, 0, 0.5);
    }
</style>

<div>
    <slot />
</div>
