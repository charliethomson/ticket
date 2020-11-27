<script>
    import ExpandedWorkorder from "../Workorder/ExpandedWorkorder.svelte"

    // TODO: move data to separate file
    const travelStatuses = [
        {
            status: "Arrived",
            color: "green",
        },
        {
            status: "En-route",
            color: "yellow",
        },
        {
            status: "OG store",
            color: "yellow",
        },
    ]

    const statuses = [
        {
            status: "Awaiting Repair",
            color: "red",
        },
        {
            status: "Quality Inspection",
            color: "red",
        },
        {
            status: "Need to Order Parts",
            color: "red",
        },
        {
            status: "Awaiting Callback",
            color: "yellow",
        },
        {
            status: "Awaiting Device",
            color: "yellow",
        },
        {
            status: "Awaiting Parts",
            color: "yellow",
        },
        {
            status: "Repair in Progress",
            color: "blue",
        },
        {
            status: "Repaired",
            color: "green",
        },
        {
            status: "Unrepairable",
            color: "green",
        },
    ]
</script>

<ExpandedWorkorder {statuses} {travelStatuses} />
