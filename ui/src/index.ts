import {Collapse} from "bootstrap";
import {invoke} from "@tauri-apps/api";

let keybindingDiv
let keybindingDivCollapse: Collapse
let keymap: any = null;
let prevIndex: number | null = null;

document.addEventListener("DOMContentLoaded", () => {
    keybindingDiv = document.getElementById("collapseWidthExample") as HTMLElement;
    keybindingDivCollapse = new Collapse(keybindingDiv);
    populateKeymapButtons().then();
    keybindingDivCollapse.hide();
})

let populateKeymapButtons = async () => {

    invoke("send_keymap").then((result) => {
        keymap = result;

        // add currently programmed macros to display
        let buttonContainer = document.querySelector<HTMLDivElement>("#buttonContainer")!;
        buttonContainer.innerHTML = '';

        // loop through each saved macro and display it
        for (let x = 0; x < result.button_count; x++) {
            let button = result.buttons[x];
            let editButton = document.createElement("a");
            editButton.className = "list-group-item list-group-item-action";
            editButton.id = "button" + x;
            editButton.textContent = button.programmable_key

            // event listener for opening config window
            editButton.addEventListener("click", () => {
                openConfigPanel(x);
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
                populateKeymapButtons();
            })
        })

        buttonContainer.append(addButton);
    })
}

let openConfigPanel = (index: number) => {
    console.log(index);

    // close if last clicked button
    if (index == prevIndex) {
        keybindingDivCollapse.hide();
        // set to null so button can open again
        prevIndex = null;
        return;
    }

    // open collapse div
    prevIndex = index;
    keybindingDivCollapse.show();

    // copy current macro actions
    let button = keymap.buttons[index];
    // console.log(button);
    let actionsDiv = document.getElementById("currentMacroActions")!;
    actionsDiv.innerHTML = '';

    for (let x = 0; x < button.actions.length; x++) {
        let actionType = button.actions[x];
        let newAction: HTMLElement;

        console.log(actionType)

        if (actionType == "None") {
            newAction = document.createElement("a");
            newAction.textContent = "None"
            newAction.className = "list-group-item"
        }
        if (actionType.hasOwnProperty("Delay")) {
            newAction = document.createElement("a");
            newAction.textContent = "Delay: " + actionType.Delay + " ms"
            newAction.className = "list-group-item"
        }
        if (actionType.hasOwnProperty("Tap")) {
            newAction = document.createElement("a");
            newAction.textContent = "Tap: " + actionType.Tap
            newAction.className = "list-group-item"
        }
        if (actionType.hasOwnProperty("Press")) {
            newAction = document.createElement("a");
            newAction.textContent = "Press: " + actionType.Press
            newAction.className = "list-group-item"
        }
        if (actionType.hasOwnProperty("Release")) {
            newAction = document.createElement("a");
            newAction.textContent = "Release: " + actionType.Release
            newAction.className = "list-group-item"
        }
        if (actionType.hasOwnProperty("Print")) {
            newAction = document.createElement("a");
            newAction.textContent = "Print: " + actionType.Print
            newAction.className = "list-group-item"
        }

        actionsDiv.append(newAction);
    }
}
