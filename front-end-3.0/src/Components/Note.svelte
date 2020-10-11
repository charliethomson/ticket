<script>
    let user;
    export let created;
    export let contents;
    let next_update;

    const user_info = fetch("http://localhost:8080/api/users?id=${user}", {
        mode: "cors",
    })
        .then(async (response) => await response.json())
        .then((json) => {
            if (json.ok) {
                return json.message[0];
            } else {
                throw `Recieved Err response: ${json.message}`;
            }
        })
        .catch((e) => console.error(e));
</script>

<style>
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
    {#await user_info then { name }}
        <div class="top">
            <div class="name">{name}</div>
            <div class="date">{new Date(created * 1000)}</div>
        </div>
        <div class="notes">{contents}</div>
    {/await}
</div>
