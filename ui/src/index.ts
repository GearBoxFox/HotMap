import {Collapse, Modal} from "bootstrap";
import {invoke} from "@tauri-apps/api";
import {createKeySelectorTemplate, Keys, sortedArray} from "./ProgrammableKeys";

let keybindingDiv
let keybindingDivCollapse: Collapse
let keymap: any = null;
let prevIndex: number | null = null;
let dirty: boolean = false;
let secondOpen: boolean = false;

let saveAlertModal: Modal;

document.addEventListener("DOMContentLoaded", () => {
    keybindingDiv = document.getElementById("collapseWidthExample") as HTMLElement;
    keybindingDivCollapse = new Collapse(keybindingDiv);
    populateKeymapButtons().then();

    saveAlertModal = new Modal(document.getElementById('save-alert')!, {backdrop: true});
    // populate events for adding macro buttons

    let addButtons = document.querySelectorAll("button.macro-add");
    for (let i = 0; i < addButtons.length; i++) {

        let button = addButtons.item(i)! as HTMLButtonElement;
        button.addEventListener("click", () => addMacroAction(button));
    }

    keybindingDivCollapse.hide();

    // add save button event listeners
    document.getElementById("save-btn")!.addEventListener("click", () => {
        dirty = false;
        secondOpen = false;
        invoke("save_keymap", {keymap: keymap}).then();
    })

    document.getElementById("save-btn-modal")!.addEventListener("click", () => {
        dirty = false;
        secondOpen = false;
        invoke("save_keymap", {keymap: keymap}).then(() => keybindingDivCollapse.hide());
    })
})

let populateKeymapButtons = async () => {

    invoke("send_keymap").then((result) => {
        keymap = result;

        // add currently programmed macros to display
        let buttonContainer = document.querySelector<HTMLDivElement>("#buttonContainer")!;
        buttonContainer.innerHTML = '';

        // loop through each saved macro and display it
        for (let x = 0; x < keymap.button_count; x++) {
            let button = keymap.buttons[x];
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
                        programmable_key: "MACRO" + (Number)(keymap.button_count + 1),
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

    // manages the display for the current macro actions
    // close if last clicked button
    if (index == prevIndex) {
        // check if there are unsaved changes
        if (dirty && !secondOpen) {
            secondOpen = true;
            saveAlertModal.show();
            return;
        } else {
            populateKeymapButtons().then();
            secondOpen = false;
            keybindingDivCollapse.hide();
        }
        // set to null so button can open again
        prevIndex = null;
        return;
    }

    // open collapse div
    prevIndex = index;
    keybindingDivCollapse.show();

    // copy current macro actions
    let button = keymap.buttons[index];
    let actionsDiv = document.getElementById("currentMacroActions")!;
    actionsDiv.innerHTML = '';

    // For each action
    for (let x = 0; x < button.actions.length; x++) {
        let actionType = button.actions[x];
        let newAction = document.createElement("label");
        let newDiv = document.createElement("div");

        newDiv.className = "list-group-item macro-current";
        newDiv.id = String(x);
        newAction.className = "fw-bold text-capitalize fs-6"

        // check action type
        if (actionType == "None") {
            newAction.textContent = "None";
            newDiv.append(newAction);
        } else if (actionType.hasOwnProperty("Delay")) {
            newAction.textContent = "Delay: " + actionType.Delay + " ms";
            newDiv.append(newAction);
        } else if (actionType.hasOwnProperty("Tap")) {
            newAction.textContent = "Tap: ";
            let selector = createKeySelectorTemplate();
            selector.selectedIndex = Object.values(sortedArray).indexOf(actionType.Tap);

            // when selected element changes, update the keymap
            selector.addEventListener("change", () => updateMacroAction(x, selector))

            selector.className = "macro-select form-select";

            newDiv.append(newAction, selector);
        } else if (actionType.hasOwnProperty("Press")) {
            newAction.textContent = "Press: ";
            let selector = createKeySelectorTemplate();
            selector.selectedIndex = Object.values(sortedArray).indexOf(actionType.Press);

            // when selected element changes, update the keymap
            selector.addEventListener("change", () => updateMacroAction(x, selector))

            selector.className = "macro-select form-select";

            newDiv.append(newAction, selector);
        } else if (actionType.hasOwnProperty("Release")) {
            newAction.textContent = "Release: ";
            let selector = createKeySelectorTemplate();
            selector.selectedIndex = Object.values(sortedArray).indexOf(actionType.Release);

            // when selected element changes, update the keymap
            selector.addEventListener("change", () => updateMacroAction(x, selector))

            selector.className = "macro-select form-select";

            newDiv.append(newAction, selector);
        } else if (actionType.hasOwnProperty("Print")) {
            newAction.textContent = "Print: " + actionType.Print
            newDiv.append(newAction);
        } else {
            newAction.textContent = "Unknown Action"
            newDiv.append(newAction);
        }


        // add remove and reorder buttons
        let editDiv = document.createElement('div');
        editDiv.className = "float-end"

        let upButton = document.createElement('img');
        upButton.src = "../assets/bootstrap-icons-1.11.3/caret-up.svg";
        upButton.className = "macro-edit rounded";

        upButton.addEventListener("click", () => reorderMacro(x, true));

        let downButton = document.createElement('img');
        downButton.src = "../assets/bootstrap-icons-1.11.3/caret-down.svg";
        downButton.className = "macro-edit rounded";

        downButton.addEventListener("click", () => reorderMacro(x, false));

        let removeButton = document.createElement('img');
        removeButton.src = "../assets/bootstrap-icons-1.11.3/dash-lg.svg";
        removeButton.className = "macro-edit rounded";

        removeButton.addEventListener("mouseup", () => removeMacro(x));

        // add all new items to a div
        editDiv.append(upButton);
        editDiv.append(downButton);
        editDiv.append(removeButton);

        newDiv.append(editDiv)

        actionsDiv.append(newDiv);
    }
}

let removeMacro = (index: number) => {
    // check if a macro button is open
    if (prevIndex != null) {
        let tempArray: any = [];

        // copy each action into a temp variable, expect for the specified index
        for (let i = 0; i < keymap.buttons[prevIndex].actions.length - 1; i++) {
            if (i < index) {
                tempArray.push(keymap.buttons[prevIndex].actions[i]);
            } else {
                tempArray.push(keymap.buttons[prevIndex].actions[i + 1]);
            }
        }

        // copy temp array into keymap
        keymap.buttons[prevIndex].actions = tempArray;

        // set 'dirty' aka made changes
        dirty = true;

        // reload macrobutton page
        let tempIndex = prevIndex;
        prevIndex = null;
        openConfigPanel(tempIndex);
    }
}

let reorderMacro = (startIndex: number, up: boolean) => {
    // check if a macro button is open
    if (prevIndex != null) {
        // check if trying to move the last action down
        if (startIndex == keymap.buttons[prevIndex].actions.length - 1 && !up) return;

        let tempArray = [];

        // copy each action into a temp variable, expect for the specified index
        for (let i = 0; i < keymap.buttons[prevIndex].actions.length; i++) {
            // if moving up and we're at the action above
            if (i == startIndex - 1 && up) {
                // copy our moving action first, then increment i again
                tempArray.push(keymap.buttons[prevIndex].actions[startIndex]);
                tempArray.push(keymap.buttons[prevIndex].actions[i]);
                i++;
            } else if (i == startIndex && !up) {
                // if moving down, copy our the action below first, then increment i again
                tempArray.push(keymap.buttons[prevIndex].actions[startIndex + 1]);
                tempArray.push(keymap.buttons[prevIndex].actions[startIndex]);
                i++;
            } else {
                tempArray.push(keymap.buttons[prevIndex].actions[i]);
            }
        }

        // copy temp array into keymap
        keymap.buttons[prevIndex].actions = tempArray;

        // set 'dirty' aka made changes
        dirty = true;

        // reload macrobutton page
        let tempIndex = prevIndex;
        prevIndex = null;
        openConfigPanel(tempIndex);
    }
}

// method to add a macro action to the keybind
let addMacroAction = (buttonClicked: HTMLButtonElement) => {
    // gets the currently active button
    if (prevIndex != null) {
        let button = keymap.buttons[prevIndex]!;

        // check the id of the button pressed and add the macro action
        switch (buttonClicked.id) {
            case "Delay":
                button.actions.push({Delay: 1.0});
                break;
            case "Tap":
                button.actions.push({Tap: Keys[Keys.KeyA].toString()});
                break;
            case "Press":
                button.actions.push({Press: Keys[Keys.KeyA].toString()});
                break;
            case "Release":
                button.actions.push({Release: Keys[Keys.KeyA].toString()});
                break;
            case "Print":
                button.actions.push({Print: "Hello, world!"});
                break;
            default:
                button.actions.push("None");
        }

        // set 'dirty' aka made changes
        dirty = true;

        // reload visual
        let temp = prevIndex;
        prevIndex = null
        openConfigPanel(temp);
    }
}

let updateMacroAction = (index: number, root: HTMLSelectElement | HTMLInputElement) => {
    if (prevIndex != null) {
        if (root instanceof HTMLSelectElement) {
            let action = keymap.buttons[prevIndex].actions[index];

            // check action type and update accordingly
            if (action.hasOwnProperty("Tap")) {
                action.Tap = root.selectedOptions.item(0)!.value;
            } else if (action.hasOwnProperty("Press")) {
                action.Press = root.selectedOptions.item(0)!.value;
            } else if (action.hasOwnProperty("Release")) {
                action.Release = root.selectedOptions.item(0)!.value;
            }
        }

        // set 'dirty' aka made changes
        dirty = true;
    }
}
