import bootstrap from 'bootstrap';

let keybindingDiv
let keybindingDivCollapse: bootstrap.Collapse

document.addEventListener("DOMContentLoaded", () => {
    keybindingDiv = document.getElementById("collapseWidthExample") as HTMLElement;
    keybindingDivCollapse = new bootstrap.Collapse(keybindingDiv);
})

export function collapseClick() {
    keybindingDivCollapse.toggle();
}
