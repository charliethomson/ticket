<script>
    export let name = ""
    export let date = 0
    export let notes = ""

    const posted = new Date(date * 1000).toLocaleString()
</script>

<style>
    .container {
        margin-top: 40px;
    }
    .top {
        display: flex;
        border-bottom: 1px solid #e3e3e3;
        margin-bottom: 10px;
        margin-top: 10px;
    }
    .name {
        margin-right: 10px;
        font-size: 20px;
        font-weight: bold;
    }
    .date {
        align-self: center;
    }
    .notes {
        margin-left: 10px;
    }
</style>

<div class="container">
    <div class="top">
        <div class="name">{name}</div>
        <div class="date">{posted}</div>
    </div>
    <div class="notes">{notes}</div>
</div>
