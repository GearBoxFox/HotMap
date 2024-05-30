import {invoke} from ""

let keybindingDiv
let keybindingDivCollapse

document.addEventListener("DOMContentLoaded", () => {
    keybindingDiv = document.getElementById("collapseWidthExample");
    keybindingDivCollapse = new bootstrap.Collapse(keybindingDiv);
})

function collapseClick() {
    keybindingDivCollapse.toggle();
}
