<script>
    import Search from "../../Components/Search.svelte"
    import { toggleCreate, toggleSearch } from "../../stores"

    function handleCreate() {
        $toggleCreate = !$toggleCreate
        $toggleSearch = !$toggleSearch

        console.log($toggleCreate)
        console.log($toggleSearch)
    }
</script>

<Search title="store" create={handleCreate} />
