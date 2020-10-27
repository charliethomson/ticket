<script>
    import Store from "./Forms/Store.svelte"
    import Customer from "./Forms/Customer.svelte"
    import Device from "./Forms/Device.svelte"

    import { seeModal, isFormValid } from "../../stores"

    let activeIndex = 1

    function handleClick() {
        activeIndex++
        $isFormValid = true
    }

    function handleEnd() {
        $isFormValid = true
        $seeModal = false
    }
</script>

{#if activeIndex == 1}
    <div class="container">
        <Store {handleEnd} {handleClick} />
    </div>
{:else if activeIndex == 2}
    <Customer {handleEnd} {handleClick} />
{:else}
    <Device {handleEnd} {handleClick} />
{/if}

<!-- TODO: Do I even need this component? -->
