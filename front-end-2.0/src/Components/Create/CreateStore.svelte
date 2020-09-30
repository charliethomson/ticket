<script>
    import Create from "../Create.svelte"
    import Input from "../Input.svelte"

    import { toggleCreate, toggleSearch } from "../../stores"

    function handleSubmit() {
        $toggleCreate = false
        $toggleSearch = false
    }

    function handleSearch() {
        $toggleCreate = !$toggleCreate
        $toggleSearch = !$toggleSearch
    }
</script>

<style>
    .address {
        display: flex;
        flex-direction: row;
    }
    .address > * {
        margin-left: 5px;
        margin-right: 5px;
    }
    .city {
        flex-grow: 3;
        flex-basis: 0;
    }
    .state {
        flex-grow: 1;
        flex-basis: 0;
    }
    .zip {
        flex-grow: 1;
        flex-basis: 0;
    }
</style>

<Create title={'Store'} submit={handleSubmit} search={handleSearch}>
    <Input label={'Name'} />
    <Input label={'Phone #'} />
    <Input label={'Email'} />
    <Input label={'Address'} />
    <div class="address">
        <div class="city">
            <Input label={'City'} />
        </div>
        <div class="state">
            <Input label={'State'} />
        </div>
        <div class="zip">
            <Input label={'Zip'} />
        </div>
    </div>
</Create>
