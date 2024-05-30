let keybindDiv
let bsCollapse

document.addEventListener("DOMContentLoaded", () => {
    keybindDiv = document.getElementById("collapseWidthExample");
    bsCollapse = new bootstrap.Collapse(keybindDiv);
})

function collapseClick() {
    console.log("I was clicked!")
    bsCollapse.toggle();
}
