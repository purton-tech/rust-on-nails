import { SideDrawer } from './side-drawer'

// We can create a trigger to open drawers
document.querySelectorAll('[data-drawer-target]').forEach(async (row) => {
    // Detect when a user clicks a row
    row.addEventListener('click', (event) => {

        event.stopImmediatePropagation()
        const drawer = document.getElementById(row.getAttribute('data-drawer-target'))
        console.log(drawer)
        if (drawer instanceof SideDrawer) {
            drawer.open = true
        }
    })
})