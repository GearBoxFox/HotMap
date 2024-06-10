import {Collapse} from "bootstrap";
import {invoke} from "@tauri-apps/api";

let keybindingDiv
let keybindingDivCollapse: Collapse

document.addEventListener("DOMContentLoaded", () => {
    keybindingDiv = document.getElementById("collapseWidthExample") as HTMLElement;
    keybindingDivCollapse = new Collapse(keybindingDiv);
})

invoke("send_keymap").then((result) => {
    console.log(result)
    let buttonContainer = document.querySelector<HTMLDivElement>("#buttonContainer")!;

    for (let x = 0; x < result.button_count; x++) {
        console.log(x);
        let button = result.buttons[x];
        console.log(button.programmable_key);
        let editButton = document.createElement("a");
        editButton.innerHTML = '' +
            '<a class="list-group-item list-group-item-action" href="#" id="button' + x + '">' +
            button.programmable_key +
            '</a>';

        editButton.addEventListener("click", () => {
            keybindingDivCollapse.toggle();
        })

        buttonContainer.append(editButton);
    }
})
