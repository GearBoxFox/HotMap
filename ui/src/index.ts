import {Collapse} from "bootstrap";
import {invoke} from "@tauri-apps/api";
import {emit, listen} from "@tauri-apps/api/event";

let keybindingDiv
let keybindingDivCollapse: Collapse

document.addEventListener("DOMContentLoaded", () => {
    keybindingDiv = document.getElementById("collapseWidthExample") as HTMLElement;
    keybindingDivCollapse = new Collapse(keybindingDiv);
    emit("reload-keymap").then()
})

let unlistenLoadKeymap = await listen('load-keymap', () => {
    invoke("send_keymap").then((result) => {
        console.log(result)
        // add currently programmed macros to display
        let buttonContainer = document.querySelector<HTMLDivElement>("#buttonContainer")!;

        // loop through each saved macro and display it
        for (let x = 0; x < result.button_count; x++) {
            let button = result.buttons[x];
            let editButton = document.createElement("a");
            editButton.className = "list-group-item list-group-item-action";
            editButton.id = "button" + x;
            editButton.textContent = button.programmable_key

            // event listener for opening config window
            editButton.addEventListener("click", () => {
                keybindingDivCollapse.toggle();
            })

            buttonContainer.append(editButton);
        }

        // add a new macro button
        let addButton = document.createElement("a");
        addButton.className = "list-group-item list-group-item-action";
        addButton.textContent = "+"

        addButton.addEventListener("click", () => {
            invoke("add_button",
                {
                    button: {
                        programmable_key: "MACRO" + (Number)(result.button_count + 1),
                        macro_type: "Once",
                        actions: ["None"]
                    }
                }).then(() => {
                // reload the keymap on the frontend
                unlistenLoadKeymap();
                emit("reload-keymap").then()
            })
        })

        buttonContainer.append(addButton);
    })
})
