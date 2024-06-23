export enum MacroKeys {
    MACROUNKNOWN,
    MACRO1,
    MACRO2,
    MACRO3,
    MACRO4,
    MACRO5,
    MACRO6,
    MACRO7,
    MACRO8,
    MACRO9,
    MACRO10,
    MACRO11,
    MACRO12,
    MACRO13,
    MACRO14,
    MACRO15,
    MACRO16,
    MACRO17,
    MACRO18,
    MACRO19,
    MACRO20,
    MACRO21,
    MACRO22,
    MACRO23,
    MACRO24,
    MACRO25,
    MACRO26,
    MACRO27,
    MACRO28,
    MACRO29,
    MACRO30,
    MACRO31,
}

export enum Keys {
    /// Alt key on Linux and Windows (option key on macOS)
    Alt,
    Backspace,
    CapsLock,
    ControlLeft,
    ControlRight,
    Delete,
    DownArrow,
    End,
    Escape,
    F1,
    F10,
    F11,
    F12,
    F2,
    F3,
    KeyA,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    Home,
    /// also known as "windows", "super", and "command"
    LeftArrow,
    /// also known as "windows", "super", and "command"
    MetaLeft,
    MetaRight,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    UpArrow,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    BackQuote,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equal,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    LeftBracket,
    RightBracket,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    SemiColon,
    Quote,
    BackSlash,
    IntlBackslash,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Dot,
    Slash,
    Insert,
    KpReturn,
    KpMinus,
    KpPlus,
    KpMultiply,
    KpDivide,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDelete,
    Function,
    Unknown
}

export let sortedArray: any = [];

// 104 is the number of elements in the 'Keys' enum
// this is a crappy way of getting this, but TS has no better option
// copy over each element to sort
for (let i = 0; i < 104; i++) {
    let text = Keys[i];
    sortedArray.push(text);
}

sortedArray.sort();

export let createKeySelectorTemplate = () => {
    let keySelectorTemplate: HTMLSelectElement = document.createElement('select');
    let tempArray: any = [];

    for (let i = 0; i < 104; i++) {

        // format out any unused whitespaces
        let text = sortedArray[i];
        if (text.startsWith("Key")) {
            text = text.replace("Key", "Key ");
        } else if (text.startsWith("Kp")) {
            text = text.replace("Kp", "Keypad ")
        } else if (text.startsWith("Control")) {
            text = text.replace("Control", "Control ");
        } else if (text.startsWith("Meta")) {
            text = text.replace("Meta", "Meta ");
        } else if (text.startsWith("Page")) {
            text = text.replace("Page", "Page ");
        } else if (text.startsWith("Shift")) {
            text = text.replace("Shift", "Shift ");
        }

        tempArray.push(text);
    }

    for (let i = 0; i < tempArray.length; i++) {
        let option: HTMLOptionElement = document.createElement('option');
        option.value = tempArray[i];
        option.textContent = tempArray[i];

        keySelectorTemplate.options.add(option);
    }

    return keySelectorTemplate;
}
