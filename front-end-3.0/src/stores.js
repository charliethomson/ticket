import { writable } from "svelte/store"



export let activeWorkorder = writable(0)
export let workorderExpanded = writable(false)
export let seeModal = writable(false)
export let isFormValid = writable(true)
export let isNoteValid = writable(true)
