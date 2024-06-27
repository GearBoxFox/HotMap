import {Modal, Toast} from "bootstrap";
import {invoke} from "@tauri-apps/api";
import {createKeySelectorTemplate, Keys, sortedArray, sortedFormated} from "./ProgrammableKeys";

let keymap: any = null;
let prevIndex: number | null = 0;
let dirty: boolean = false;
let secondOpen: boolean = false;
let actionsDiv: HTMLDivElement;

let saveAlertModal: Modal;

const localStorageTheme = localStorage.getItem("theme");
const systemSettingDark = window.matchMedia("(prefers-color-scheme: dark)");

let currentThemeSetting: string;

window.addEventListener('error', function(event) {
    console.error('Caught an error:', event.message);
    return false;
});

document.addEventListener("DOMContentLoaded", () => {
    currentThemeSetting = calculateSettingAsThemeString(localStorageTheme, systemSettingDark );
    populateKeymapButtons();

    saveAlertModal = new Modal(document.getElementById('save-alert')!, {backdrop: true});
    // populate events for adding macro buttons

    let addButtons = document.querySelectorAll("button.macro-add");
    for (let i = 0; i < addButtons.length; i++) {

        let button = addButtons.item(i)! as HTMLButtonElement;
        button.addEventListener("click", () => addMacroAction(button));
    }

    // add save button event listeners
    document.getElementById("save-btn")!.addEventListener("click", () => {
        invoke("save_keymap", {keymap: keymap}).then(() => {
            dirty = false;
            secondOpen = false;
            new Toast(document.getElementById("saveToast")!).show();
        });
    });

    document.getElementById("save-btn-modal")!.addEventListener("click", () => {

        invoke("save_keymap", {keymap: keymap}).then(() => {
            dirty = false;
            secondOpen = false;
            saveAlertModal.hide();
            new Toast(document.getElementById("saveToast")!).show();
        });
    });

    // get the div element that stores the current macro actions
    actionsDiv = document.getElementById("currentMacroActions")! as HTMLDivElement;

    // create button for dark mode toggle
    // target the button using the data attribute we added earlier
    const button = document.querySelector("[data-theme-toggle]")! as HTMLElement;

    button.addEventListener("click", () => {
        const newTheme = currentThemeSetting === "dark" ? "light" : "dark";

        // update the button text
        const newCta = newTheme === "dark" ? "Change to light theme" : "Change to dark theme";

        let newImg = document.createElement("img");
        if (newTheme === "dark") {
            newImg.src = "assets/bootstrap-icons-1.11.3/moon-stars.svg"
        } else {
            newImg.src = "assets/bootstrap-icons-1.11.3/sun.svg"
        }
        button.innerHTML = '';
        button.append(newImg);

        // use an aria-label if you are omitting text on the button
        // and using sun/moon icons, for example
        button.setAttribute("aria-label", newCta);

        // update theme attribute on HTML to switch theme in CSS
        document.querySelector("html")!.setAttribute("data-theme", newTheme);

        // update in local storage
        localStorage.setItem("theme", newTheme);

        // update the currentThemeSetting in memory
        currentThemeSetting = newTheme;
    });
})

let calculateSettingAsThemeString = (localStorageTheme: string | null, systemSettingDark: MediaQueryList) =>{
    if (localStorageTheme !== null) {
        return localStorageTheme;
    }

    if (systemSettingDark.matches) {
        return "dark";
    }

    return "light";
}

let populateKeymapButtons = () => {

    invoke("send_keymap").then((result) => {
        keymap = result;

        // add currently programmed macros to display
        let buttonContainer = document.querySelector<HTMLDivElement>("#buttonContainer")!;
        buttonContainer.innerHTML = '';

        // loop through each saved macro and display it
        for (let x = 0; x < keymap.button_count; x++) {
            let button = keymap.buttons[x];
            let editButton = document.createElement("a");
            editButton.className = "list-group-item list-group-item-action content-box";
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
        addButton.className = "list-group-item list-group-item-action content-box-rounded";
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
    if (index != prevIndex && prevIndex != null) {
        // check if there are unsaved changes
        if (dirty && !secondOpen) {
            secondOpen = true;
            saveAlertModal.show();
            return;
        } else {
            dirty = false;
            secondOpen = false;
            populateKeymapButtons();
        }
    } else {
        secondOpen = false;
        dirty = false;
    }

    prevIndex = index;

    // copy current macro actions
    let button = keymap.buttons[index];
    actionsDiv.innerHTML = '';

    // label the currently active macro
    document.getElementById("macroLabel")!.textContent = "Macro " + String(index + 1);

    // For each action
    for (let x = 0; x < button.actions.length; x++) {
        let actionType = button.actions[x];
        addVisualMacro(actionType, x);
    }
}

let addVisualMacro = (actionType: any, index: number) => {
    let newAction = document.createElement("label");
    let newDiv = document.createElement("div");

    newDiv.className = "list-group-item macro-current content-box";
    newDiv.id = String(index);
    newAction.className = "fw-bold text-capitalize fs-6"

    // check action type
    if (actionType == "None") {
        newAction.textContent = "None";
        newDiv.append(newAction);
    } else if (actionType.hasOwnProperty("Delay")) {
        newAction.textContent = "Delay (ms): ";
        let input = document.createElement("input")
        input.type = "text"
        input.alt = "milliseconds"
        input.value = String(actionType.Delay);

        input.className = "macro-input";

        input.addEventListener("change", () => updateMacroAction(index, input));

        newDiv.append(newAction, input);
    } else if (actionType.hasOwnProperty("Tap")) {
        newAction.textContent = "Tap: ";
        let selector = createKeySelectorTemplate();
        selector.selectedIndex = Object.values(sortedArray).indexOf(actionType.Tap);

        // when selected element changes, update the keymap
        selector.addEventListener("change", () => updateMacroAction(index, selector))

        selector.className = "macro-select form-select";

        newDiv.append(newAction, selector);
    } else if (actionType.hasOwnProperty("Press")) {
        newAction.textContent = "Press: ";
        let selector = createKeySelectorTemplate();
        selector.selectedIndex = Object.values(sortedArray).indexOf(actionType.Press);

        // when selected element changes, update the keymap
        selector.addEventListener("change", () => {
            updateMacroAction(index, selector);
        });

        selector.className = "macro-select form-select";

        newDiv.append(newAction, selector);
        console.log("Added new 'Press' action");
    } else if (actionType.hasOwnProperty("Release")) {
        newAction.textContent = "Release: ";
        let selector = createKeySelectorTemplate();
        selector.selectedIndex = Object.values(sortedArray).indexOf(actionType.Release);

        // when selected element changes, update the keymap
        selector.addEventListener("change", () => updateMacroAction(index, selector))

        selector.className = "macro-select form-select";

        newDiv.append(newAction, selector);
    } else if (actionType.hasOwnProperty("Print")) {
        newAction.textContent = "Print: ";
        let input = document.createElement("input");
        input.type = "text"
        input.alt = "Print"
        input.value = actionType.Print;

        input.className = "macro-input";

        input.addEventListener("change", () => updateMacroAction(index, input));
        newDiv.append(newAction, input);
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

    upButton.addEventListener("click", () => reorderMacro(index, true));

    let downButton = document.createElement('img');
    downButton.src = "../assets/bootstrap-icons-1.11.3/caret-down.svg";
    downButton.className = "macro-edit rounded";

    downButton.addEventListener("click", () => reorderMacro(index, false));

    let removeButton = document.createElement('img');
    removeButton.src = "../assets/bootstrap-icons-1.11.3/dash-lg.svg";
    removeButton.className = "macro-edit rounded";

    removeButton.addEventListener("mouseup", () => removeMacro(index));

    // add all new items to a div
    editDiv.append(upButton);
    editDiv.append(downButton);
    editDiv.append(removeButton);

    newDiv.append(editDiv)

    actionsDiv.append(newDiv);
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
        let action: any;
        switch (buttonClicked.id) {
            case "Delay":
                action = {Delay: 1.0};
                break;
            case "Tap":
                action = {Tap: Keys[Keys.KeyA].toString()};
                break;
            case "Press":
                action = {Press: Keys[Keys.KeyA].toString()};
                break;
            case "Release":
                action = {Release: Keys[Keys.KeyA].toString()};
                break;
            case "Print":
                action = {Print: "Hello, world!"};
                break;
            default:
                action = "None";
        }

        // set 'dirty' aka made changes
        dirty = true;

        // add action to visual and keymap
        button.actions.push(action);
        addVisualMacro(action, button.actions.length - 1);
    }
}

let updateMacroAction = (index: number, root: HTMLSelectElement | HTMLInputElement) => {
    console.log("Trying to edit macro");
    if (prevIndex != null) {
        if (root instanceof HTMLSelectElement) {
            console.log("Selector was opened");

            let action = keymap.buttons[prevIndex].actions[index];
            let selectedIndex =
                Object.values(sortedFormated).indexOf(root.selectedOptions.item(0)!.value)
            let actionValue = sortedArray[selectedIndex];

            // check action type and update accordingly
            if (action.hasOwnProperty("Tap")) {
                action.Tap = actionValue;
            } else if (action.hasOwnProperty("Press")) {
                action.Press = actionValue;
            } else if (action.hasOwnProperty("Release")) {
                action.Release = actionValue;
            }
            console.log("Selector opened finished");
        } else {
            let action = keymap.buttons[prevIndex].actions[index];
            console.log(action);

            if (action.hasOwnProperty("Delay")) {
                console.log(root.value);
                action.Delay = Number(root.value);
            } else if (action.hasOwnProperty("Print")) {
                action.Print = root.value;
            }
        }

        // set 'dirty' aka made changes
        dirty = true;
    }
}
