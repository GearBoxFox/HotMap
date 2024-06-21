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

    // manages the display for the current macro actions
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

    // For each action
    for (let x = 0; x < button.actions.length; x++) {
        let actionType = button.actions[x];
        let newAction = document.createElement("label");
        let newDiv = document.createElement("div");

        newDiv.className = "list-group-item";
        newAction.className = "bold"

        // check action type
        if (actionType == "None") {
            newAction.textContent = "None";
            newDiv.append(newAction);
        } else if (actionType.hasOwnProperty("Delay")) {
            newAction.textContent = "Delay: " + actionType.Delay + " ms";
            newDiv.append(newAction);
        } else if (actionType.hasOwnProperty("Tap")) {
            newAction.textContent = "Tap: ";
            let selector = document.createElement("select");

            let option = document.createElement("option");
            option.value = "1";
            option.textContent = "Option 1";

            selector.className = "macro-select";
            selector.options.add(option);

            newDiv.append(newAction, selector);
        } else if (actionType.hasOwnProperty("Press")) {
            newAction.textContent = "Press: " + actionType.Press
            newDiv.append(newAction);
        } else if (actionType.hasOwnProperty("Release")) {
            newAction.textContent = "Release: " + actionType.Release
            newDiv.append(newAction);
        } else if (actionType.hasOwnProperty("Print")) {
            newAction.textContent = "Print: " + actionType.Print
            newDiv.append(newAction);
        } else {
            newAction.textContent = "Unknown Action"
            newDiv.append(newAction);
        }

        actionsDiv.append(newDiv);
    }
}

// method to add a macro action to the keybind
let addMacroAction = () => {
    
}
