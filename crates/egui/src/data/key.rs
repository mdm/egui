use super::Code;

/// Keyboard keys.
///
/// egui usually uses logical keys, i.e. after applying any user keymap.
///
/// Because these are *logical*, physically distinct keys can map to the same
/// `Key`: both Enter and the numpad Enter are [`Key::Enter`]. If you need to
/// tell them apart, use the physical [`Code`] instead.\
// See comment at the end of `Key { … }` on how to add new keys.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Key {
    // ----------------------------------------------
    // Commands:
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,

    Escape,
    Tab,
    Backspace,
    Enter,
    Space,

    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,

    Copy,
    Cut,
    Paste,

    // ----------------------------------------------
    // Punctuation:
    /// `:`
    Colon,

    /// `,`
    Comma,

    /// `\`
    Backslash,

    /// `/`
    Slash,

    /// `|`, a vertical bar
    Pipe,

    /// `?`
    Questionmark,

    // '!'
    Exclamationmark,

    // `[`
    OpenBracket,

    // `]`
    CloseBracket,

    // `{`
    OpenCurlyBracket,

    // `}`
    CloseCurlyBracket,

    /// Also known as "backquote" or "grave"
    Backtick,

    /// `-`
    Minus,

    /// `.`
    Period,

    /// `+`
    Plus,

    /// `=`
    Equals,

    /// `;`
    Semicolon,

    /// `'`
    Quote,

    // ----------------------------------------------
    // Digits:
    /// `0` (from main row or numpad)
    Num0,

    /// `1` (from main row or numpad)
    Num1,

    /// `2` (from main row or numpad)
    Num2,

    /// `3` (from main row or numpad)
    Num3,

    /// `4` (from main row or numpad)
    Num4,

    /// `5` (from main row or numpad)
    Num5,

    /// `6` (from main row or numpad)
    Num6,

    /// `7` (from main row or numpad)
    Num7,

    /// `8` (from main row or numpad)
    Num8,

    /// `9` (from main row or numpad)
    Num9,

    // ----------------------------------------------
    // Letters:
    A, // Used for cmd+A (select All)
    B,
    C, // |CMD COPY|
    D, // |CMD BOOKMARK|
    E, // |CMD SEARCH|
    F, // |CMD FIND firefox & chrome|
    G, // |CMD FIND chrome|
    H, // |CMD History|
    I, // italics
    J, // |CMD SEARCH firefox/DOWNLOAD chrome|
    K, // Used for ctrl+K (delete text after cursor)
    L,
    M,
    N,
    O, // |CMD OPEN|
    P, // |CMD PRINT|
    Q,
    R, // |CMD REFRESH|
    S, // |CMD SAVE|
    T, // |CMD TAB|
    U, // Used for ctrl+U (delete text before cursor)
    V, // |CMD PASTE|
    W, // Used for ctrl+W (delete previous word)
    X, // |CMD CUT|
    Y,
    Z, // |CMD UNDO|

    // ----------------------------------------------
    // Function keys:
    F1,
    F2,
    F3,
    F4,
    F5, // |CMD REFRESH|
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,

    /// Back navigation key from multimedia keyboard.
    /// Android sends this key on Back button press.
    /// Does not work on Web.
    BrowserBack,

    // ----------------------------------------------
    // Modifier keys (exposed as distinct left/right variants so that
    // games and input-capture UIs can bind them independently). egui's
    // `Modifiers` struct still collapses both sides for the common case
    // (e.g. "Ctrl+C"); these variants are emitted only as physical
    // `Event::Key` presses.
    /// Left Shift key.
    ShiftLeft,

    /// Right Shift key.
    ShiftRight,

    /// Left Control key.
    ControlLeft,

    /// Right Control key.
    ControlRight,

    /// Left Alt / Option key.
    AltLeft,

    /// Right Alt / `AltGr` / Option key.
    AltRight,

    /// Left Super / Meta / Command / Windows key.
    SuperLeft,

    /// Right Super / Meta / Command / Windows key.
    SuperRight,

    // ----------------------------------------------
    // International keys — physical positions that only exist on
    // non-US keyboards.
    /// ISO 102nd key: physically located between the left Shift and Z
    /// on ISO layouts. On French AZERTY it produces `<>|`; on UK
    /// QWERTY a secondary `\` / `|`. Missing from US ANSI keyboards.
    IntlBackslash,
    // When adding keys, remember to also update:
    // * Key::ALL
    // * Key::from_name
    // * Key::from_code (maps the physical `Code` to this logical key)
    // Note that the physical mapping in `crates/egui-winit/src/lib.rs` is derived
    // from `keyboard_types::Code` and does *not* need updating for a new `Key`.
    // You should test that it works using the "Input Event History" window in the egui demo app.
    // Make sure to test both natively and on web!
    // Also: don't add keys last; add them to the group they best belong to.
}

impl Key {
    /// All egui keys
    pub const ALL: &'static [Self] = &[
        // Commands:
        Self::ArrowDown,
        Self::ArrowLeft,
        Self::ArrowRight,
        Self::ArrowUp,
        Self::Escape,
        Self::Tab,
        Self::Backspace,
        Self::Enter,
        Self::Insert,
        Self::Delete,
        Self::Home,
        Self::End,
        Self::PageUp,
        Self::PageDown,
        Self::Copy,
        Self::Cut,
        Self::Paste,
        // Punctuation:
        Self::Space,
        Self::Colon,
        Self::Comma,
        Self::Minus,
        Self::Period,
        Self::Plus,
        Self::Equals,
        Self::Semicolon,
        Self::OpenBracket,
        Self::CloseBracket,
        Self::OpenCurlyBracket,
        Self::CloseCurlyBracket,
        Self::Backtick,
        Self::Backslash,
        Self::Slash,
        Self::Pipe,
        Self::Questionmark,
        Self::Exclamationmark,
        Self::Quote,
        // Digits:
        Self::Num0,
        Self::Num1,
        Self::Num2,
        Self::Num3,
        Self::Num4,
        Self::Num5,
        Self::Num6,
        Self::Num7,
        Self::Num8,
        Self::Num9,
        // Letters:
        Self::A,
        Self::B,
        Self::C,
        Self::D,
        Self::E,
        Self::F,
        Self::G,
        Self::H,
        Self::I,
        Self::J,
        Self::K,
        Self::L,
        Self::M,
        Self::N,
        Self::O,
        Self::P,
        Self::Q,
        Self::R,
        Self::S,
        Self::T,
        Self::U,
        Self::V,
        Self::W,
        Self::X,
        Self::Y,
        Self::Z,
        // Function keys:
        Self::F1,
        Self::F2,
        Self::F3,
        Self::F4,
        Self::F5,
        Self::F6,
        Self::F7,
        Self::F8,
        Self::F9,
        Self::F10,
        Self::F11,
        Self::F12,
        Self::F13,
        Self::F14,
        Self::F15,
        Self::F16,
        Self::F17,
        Self::F18,
        Self::F19,
        Self::F20,
        Self::F21,
        Self::F22,
        Self::F23,
        Self::F24,
        Self::F25,
        Self::F26,
        Self::F27,
        Self::F28,
        Self::F29,
        Self::F30,
        Self::F31,
        Self::F32,
        Self::F33,
        Self::F34,
        Self::F35,
        // Navigation keys:
        Self::BrowserBack,
        // Modifier keys (physical L/R):
        Self::ShiftLeft,
        Self::ShiftRight,
        Self::ControlLeft,
        Self::ControlRight,
        Self::AltLeft,
        Self::AltRight,
        Self::SuperLeft,
        Self::SuperRight,
        // International keys:
        Self::IntlBackslash,
    ];

    /// Converts `"A"` to `Key::A`, `Space` to `Key::Space`, etc.
    ///
    /// Makes sense for logical keys.
    ///
    /// This will parse the output of both [`Self::name`] and [`Self::symbol_or_name`],
    /// but will also parse single characters, so that both `"-"` and `"Minus"` will return `Key::Minus`.
    ///
    /// This should support both the names generated in a web browser,
    /// and by winit. Please test on both with `eframe`.
    pub fn from_name(key: &str) -> Option<Self> {
        Some(match key {
            "⏷" | "ArrowDown" | "Down" => Self::ArrowDown,
            "⏴" | "ArrowLeft" | "Left" => Self::ArrowLeft,
            "⏵" | "ArrowRight" | "Right" => Self::ArrowRight,
            "⏶" | "ArrowUp" | "Up" => Self::ArrowUp,

            "Escape" | "Esc" => Self::Escape,
            "Tab" => Self::Tab,
            "Backspace" => Self::Backspace,
            "Enter" | "Return" | "NumpadEnter" => Self::Enter,

            "Help" | "Insert" => Self::Insert,
            "Delete" => Self::Delete,
            "Home" => Self::Home,
            "End" => Self::End,
            "PageUp" => Self::PageUp,
            "PageDown" => Self::PageDown,

            "Copy" => Self::Copy,
            "Cut" => Self::Cut,
            "Paste" => Self::Paste,

            " " | "Space" => Self::Space,
            ":" | "Colon" => Self::Colon,
            "," | "Comma" | "NumpadComma" => Self::Comma,
            "-" | "−" | "Minus" | "NumpadSubtract" => Self::Minus,
            "." | "Period" | "NumpadDecimal" => Self::Period,
            "+" | "Plus" | "NumpadAdd" => Self::Plus,
            "=" | "Equal" | "Equals" | "NumpadEqual" => Self::Equals,
            ";" | "Semicolon" => Self::Semicolon,
            "\\" | "Backslash" => Self::Backslash,
            "/" | "Slash" | "NumpadDivide" => Self::Slash,
            "|" | "Pipe" => Self::Pipe,
            "?" | "Questionmark" => Self::Questionmark,
            "!" | "Exclamationmark" => Self::Exclamationmark,
            "[" | "OpenBracket" | "BracketLeft" => Self::OpenBracket,
            "]" | "CloseBracket" | "BracketRight" => Self::CloseBracket,
            "{" | "OpenCurlyBracket" => Self::OpenCurlyBracket,
            "}" | "CloseCurlyBracket" => Self::CloseCurlyBracket,
            "`" | "Backtick" | "Backquote" | "Grave" => Self::Backtick,
            "'" | "Quote" => Self::Quote,

            "0" | "Digit0" | "Numpad0" => Self::Num0,
            "1" | "Digit1" | "Numpad1" => Self::Num1,
            "2" | "Digit2" | "Numpad2" => Self::Num2,
            "3" | "Digit3" | "Numpad3" => Self::Num3,
            "4" | "Digit4" | "Numpad4" => Self::Num4,
            "5" | "Digit5" | "Numpad5" => Self::Num5,
            "6" | "Digit6" | "Numpad6" => Self::Num6,
            "7" | "Digit7" | "Numpad7" => Self::Num7,
            "8" | "Digit8" | "Numpad8" => Self::Num8,
            "9" | "Digit9" | "Numpad9" => Self::Num9,

            "a" | "A" | "KeyA" => Self::A,
            "b" | "B" | "KeyB" => Self::B,
            "c" | "C" | "KeyC" => Self::C,
            "d" | "D" | "KeyD" => Self::D,
            "e" | "E" | "KeyE" => Self::E,
            "f" | "F" | "KeyF" => Self::F,
            "g" | "G" | "KeyG" => Self::G,
            "h" | "H" | "KeyH" => Self::H,
            "i" | "I" | "KeyI" => Self::I,
            "j" | "J" | "KeyJ" => Self::J,
            "k" | "K" | "KeyK" => Self::K,
            "l" | "L" | "KeyL" => Self::L,
            "m" | "M" | "KeyM" => Self::M,
            "n" | "N" | "KeyN" => Self::N,
            "o" | "O" | "KeyO" => Self::O,
            "p" | "P" | "KeyP" => Self::P,
            "q" | "Q" | "KeyQ" => Self::Q,
            "r" | "R" | "KeyR" => Self::R,
            "s" | "S" | "KeyS" => Self::S,
            "t" | "T" | "KeyT" => Self::T,
            "u" | "U" | "KeyU" => Self::U,
            "v" | "V" | "KeyV" => Self::V,
            "w" | "W" | "KeyW" => Self::W,
            "x" | "X" | "KeyX" => Self::X,
            "y" | "Y" | "KeyY" => Self::Y,
            "z" | "Z" | "KeyZ" => Self::Z,

            "F1" => Self::F1,
            "F2" => Self::F2,
            "F3" => Self::F3,
            "F4" => Self::F4,
            "F5" => Self::F5,
            "F6" => Self::F6,
            "F7" => Self::F7,
            "F8" => Self::F8,
            "F9" => Self::F9,
            "F10" => Self::F10,
            "F11" => Self::F11,
            "F12" => Self::F12,
            "F13" => Self::F13,
            "F14" => Self::F14,
            "F15" => Self::F15,
            "F16" => Self::F16,
            "F17" => Self::F17,
            "F18" => Self::F18,
            "F19" => Self::F19,
            "F20" => Self::F20,
            "F21" => Self::F21,
            "F22" => Self::F22,
            "F23" => Self::F23,
            "F24" => Self::F24,
            "F25" => Self::F25,
            "F26" => Self::F26,
            "F27" => Self::F27,
            "F28" => Self::F28,
            "F29" => Self::F29,
            "F30" => Self::F30,
            "F31" => Self::F31,
            "F32" => Self::F32,
            "F33" => Self::F33,
            "F34" => Self::F34,
            "F35" => Self::F35,

            "BrowserBack" => Self::BrowserBack,

            "ShiftLeft" => Self::ShiftLeft,
            "ShiftRight" => Self::ShiftRight,
            "ControlLeft" => Self::ControlLeft,
            "ControlRight" => Self::ControlRight,
            "AltLeft" => Self::AltLeft,
            "AltRight" => Self::AltRight,

            "SuperLeft" | "MetaLeft" | "OSLeft" => Self::SuperLeft,
            "SuperRight" | "MetaRight" | "OSRight" => Self::SuperRight,

            "IntlBackslash" => Self::IntlBackslash,

            _ => return None,
        })
    }

    /// The *logical* key that a physical key produces on a US keyboard layout.
    ///
    /// This is the canonical way to go from a physical [`Code`] to a logical `Key`.
    /// It deliberately folds the numpad into the main row, matching the
    /// [W3C UI Events][spec] definition of `KeyboardEvent.key`: pressing the numpad
    /// Enter really does report a logical `"Enter"`, and numpad `5` reports `"5"`.
    ///
    /// ```
    /// # use egui::{Code, Key};
    /// assert_eq!(Key::from_code(Code::NumpadEnter), Some(Key::Enter));
    /// assert_eq!(Key::from_code(Code::Numpad5), Some(Key::Num5));
    /// ```
    ///
    /// Integrations use this as a fallback when the platform cannot report a logical
    /// key, e.g. on keyboard layouts without Latin characters, so that standard
    /// shortcuts such as `Ctrl` + `V` keep working.
    /// See <https://github.com/emilk/egui/issues/3653>.
    ///
    /// If you want to tell the numpad apart from the main row, don't use this —
    /// match on the [`Code`] itself, via the `physical_key` field of
    /// [`crate::Event::Key`] or via [`crate::InputState::code_down`].
    ///
    /// Returns `None` for physical keys that egui has no logical `Key` for.
    ///
    /// [spec]: https://www.w3.org/TR/uievents-key/
    pub fn from_code(code: Code) -> Option<Self> {
        Some(match code {
            Code::ArrowDown => Self::ArrowDown,
            Code::ArrowLeft => Self::ArrowLeft,
            Code::ArrowRight => Self::ArrowRight,
            Code::ArrowUp => Self::ArrowUp,

            Code::Escape => Self::Escape,
            Code::Tab => Self::Tab,
            Code::Backspace => Self::Backspace,
            Code::Enter | Code::NumpadEnter => Self::Enter,

            Code::Insert | Code::Help => Self::Insert,
            Code::Delete => Self::Delete,
            Code::Home => Self::Home,
            Code::End => Self::End,
            Code::PageUp => Self::PageUp,
            Code::PageDown => Self::PageDown,

            Code::Copy => Self::Copy,
            Code::Cut => Self::Cut,
            Code::Paste => Self::Paste,

            // NOTE: there is no physical colon, pipe, question mark, exclamation mark
            // or curly bracket key on a US keyboard — those are all shifted keys.
            Code::Space => Self::Space,
            Code::Comma | Code::NumpadComma => Self::Comma,
            Code::Minus | Code::NumpadSubtract => Self::Minus,
            Code::Period | Code::NumpadDecimal => Self::Period,
            Code::NumpadAdd => Self::Plus,
            Code::Equal | Code::NumpadEqual => Self::Equals,
            Code::Semicolon => Self::Semicolon,
            Code::Backslash => Self::Backslash,
            Code::Slash | Code::NumpadDivide => Self::Slash,
            Code::BracketLeft => Self::OpenBracket,
            Code::BracketRight => Self::CloseBracket,
            Code::Backquote => Self::Backtick,
            Code::Quote => Self::Quote,

            Code::Digit0 | Code::Numpad0 => Self::Num0,
            Code::Digit1 | Code::Numpad1 => Self::Num1,
            Code::Digit2 | Code::Numpad2 => Self::Num2,
            Code::Digit3 | Code::Numpad3 => Self::Num3,
            Code::Digit4 | Code::Numpad4 => Self::Num4,
            Code::Digit5 | Code::Numpad5 => Self::Num5,
            Code::Digit6 | Code::Numpad6 => Self::Num6,
            Code::Digit7 | Code::Numpad7 => Self::Num7,
            Code::Digit8 | Code::Numpad8 => Self::Num8,
            Code::Digit9 | Code::Numpad9 => Self::Num9,

            Code::KeyA => Self::A,
            Code::KeyB => Self::B,
            Code::KeyC => Self::C,
            Code::KeyD => Self::D,
            Code::KeyE => Self::E,
            Code::KeyF => Self::F,
            Code::KeyG => Self::G,
            Code::KeyH => Self::H,
            Code::KeyI => Self::I,
            Code::KeyJ => Self::J,
            Code::KeyK => Self::K,
            Code::KeyL => Self::L,
            Code::KeyM => Self::M,
            Code::KeyN => Self::N,
            Code::KeyO => Self::O,
            Code::KeyP => Self::P,
            Code::KeyQ => Self::Q,
            Code::KeyR => Self::R,
            Code::KeyS => Self::S,
            Code::KeyT => Self::T,
            Code::KeyU => Self::U,
            Code::KeyV => Self::V,
            Code::KeyW => Self::W,
            Code::KeyX => Self::X,
            Code::KeyY => Self::Y,
            Code::KeyZ => Self::Z,

            Code::F1 => Self::F1,
            Code::F2 => Self::F2,
            Code::F3 => Self::F3,
            Code::F4 => Self::F4,
            Code::F5 => Self::F5,
            Code::F6 => Self::F6,
            Code::F7 => Self::F7,
            Code::F8 => Self::F8,
            Code::F9 => Self::F9,
            Code::F10 => Self::F10,
            Code::F11 => Self::F11,
            Code::F12 => Self::F12,
            Code::F13 => Self::F13,
            Code::F14 => Self::F14,
            Code::F15 => Self::F15,
            Code::F16 => Self::F16,
            Code::F17 => Self::F17,
            Code::F18 => Self::F18,
            Code::F19 => Self::F19,
            Code::F20 => Self::F20,
            Code::F21 => Self::F21,
            Code::F22 => Self::F22,
            Code::F23 => Self::F23,
            Code::F24 => Self::F24,
            Code::F25 => Self::F25,
            Code::F26 => Self::F26,
            Code::F27 => Self::F27,
            Code::F28 => Self::F28,
            Code::F29 => Self::F29,
            Code::F30 => Self::F30,
            Code::F31 => Self::F31,
            Code::F32 => Self::F32,
            Code::F33 => Self::F33,
            Code::F34 => Self::F34,
            Code::F35 => Self::F35,

            Code::BrowserBack => Self::BrowserBack,

            Code::ShiftLeft => Self::ShiftLeft,
            Code::ShiftRight => Self::ShiftRight,
            Code::ControlLeft => Self::ControlLeft,
            Code::ControlRight => Self::ControlRight,
            Code::AltLeft => Self::AltLeft,
            Code::AltRight => Self::AltRight,
            Code::MetaLeft => Self::SuperLeft,
            Code::MetaRight => Self::SuperRight,

            Code::IntlBackslash => Self::IntlBackslash,

            _ => return None,
        })
    }

    /// Emoji or name representing the key
    pub fn symbol_or_name(self) -> &'static str {
        // TODO(emilk): add support for more unicode symbols (see for instance https://wincent.com/wiki/Unicode_representations_of_modifier_keys).
        // Before we do we must first make sure they are supported in `Fonts` though,
        // so perhaps this functions needs to take a `supports_character: impl Fn(char) -> bool` or something.
        match self {
            Self::ArrowDown => "⏷",
            Self::ArrowLeft => "⏴",
            Self::ArrowRight => "⏵",
            Self::ArrowUp => "⏶",

            Self::Colon => ":",
            Self::Comma => ",",
            Self::Minus => crate::MINUS_CHAR_STR,
            Self::Period => ".",
            Self::Plus => "+",
            Self::Equals => "=",
            Self::Semicolon => ";",
            Self::Backslash => "\\",
            Self::Slash => "/",
            Self::Pipe => "|",
            Self::Questionmark => "?",
            Self::Exclamationmark => "!",
            Self::OpenBracket => "[",
            Self::CloseBracket => "]",
            Self::OpenCurlyBracket => "{",
            Self::CloseCurlyBracket => "}",
            Self::Backtick => "`",

            _ => self.name(),
        }
    }

    /// Human-readable English name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ArrowDown => "Down",
            Self::ArrowLeft => "Left",
            Self::ArrowRight => "Right",
            Self::ArrowUp => "Up",

            Self::Escape => "Escape",
            Self::Tab => "Tab",
            Self::Backspace => "Backspace",
            Self::Enter => "Enter",

            Self::Insert => "Insert",
            Self::Delete => "Delete",
            Self::Home => "Home",
            Self::End => "End",
            Self::PageUp => "PageUp",
            Self::PageDown => "PageDown",

            Self::Copy => "Copy",
            Self::Cut => "Cut",
            Self::Paste => "Paste",

            Self::Space => "Space",
            Self::Colon => "Colon",
            Self::Comma => "Comma",
            Self::Minus => "Minus",
            Self::Period => "Period",
            Self::Plus => "Plus",
            Self::Equals => "Equals",
            Self::Semicolon => "Semicolon",
            Self::Backslash => "Backslash",
            Self::Slash => "Slash",
            Self::Pipe => "Pipe",
            Self::Questionmark => "Questionmark",
            Self::Exclamationmark => "Exclamationmark",
            Self::OpenBracket => "OpenBracket",
            Self::CloseBracket => "CloseBracket",
            Self::OpenCurlyBracket => "OpenCurlyBracket",
            Self::CloseCurlyBracket => "CloseCurlyBracket",
            Self::Backtick => "Backtick",
            Self::Quote => "Quote",

            Self::Num0 => "0",
            Self::Num1 => "1",
            Self::Num2 => "2",
            Self::Num3 => "3",
            Self::Num4 => "4",
            Self::Num5 => "5",
            Self::Num6 => "6",
            Self::Num7 => "7",
            Self::Num8 => "8",
            Self::Num9 => "9",

            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
            Self::H => "H",
            Self::I => "I",
            Self::J => "J",
            Self::K => "K",
            Self::L => "L",
            Self::M => "M",
            Self::N => "N",
            Self::O => "O",
            Self::P => "P",
            Self::Q => "Q",
            Self::R => "R",
            Self::S => "S",
            Self::T => "T",
            Self::U => "U",
            Self::V => "V",
            Self::W => "W",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
            Self::F1 => "F1",
            Self::F2 => "F2",
            Self::F3 => "F3",
            Self::F4 => "F4",
            Self::F5 => "F5",
            Self::F6 => "F6",
            Self::F7 => "F7",
            Self::F8 => "F8",
            Self::F9 => "F9",
            Self::F10 => "F10",
            Self::F11 => "F11",
            Self::F12 => "F12",
            Self::F13 => "F13",
            Self::F14 => "F14",
            Self::F15 => "F15",
            Self::F16 => "F16",
            Self::F17 => "F17",
            Self::F18 => "F18",
            Self::F19 => "F19",
            Self::F20 => "F20",
            Self::F21 => "F21",
            Self::F22 => "F22",
            Self::F23 => "F23",
            Self::F24 => "F24",
            Self::F25 => "F25",
            Self::F26 => "F26",
            Self::F27 => "F27",
            Self::F28 => "F28",
            Self::F29 => "F29",
            Self::F30 => "F30",
            Self::F31 => "F31",
            Self::F32 => "F32",
            Self::F33 => "F33",
            Self::F34 => "F34",
            Self::F35 => "F35",

            Self::BrowserBack => "BrowserBack",

            Self::ShiftLeft => "ShiftLeft",
            Self::ShiftRight => "ShiftRight",
            Self::ControlLeft => "ControlLeft",
            Self::ControlRight => "ControlRight",
            Self::AltLeft => "AltLeft",
            Self::AltRight => "AltRight",
            Self::SuperLeft => "SuperLeft",
            Self::SuperRight => "SuperRight",

            Self::IntlBackslash => "IntlBackslash",
        }
    }
}

#[test]
fn test_key_from_name() {
    assert_eq!(
        Key::ALL.len(),
        Key::IntlBackslash as usize + 1,
        "Some keys are missing in Key::ALL"
    );

    for &key in Key::ALL {
        let name = key.name();
        assert_eq!(
            Key::from_name(name),
            Some(key),
            "Failed to roundtrip {key:?} from name {name:?}"
        );

        let symbol = key.symbol_or_name();
        assert_eq!(
            Key::from_name(symbol),
            Some(key),
            "Failed to roundtrip {key:?} from symbol {symbol:?}"
        );
    }
}

#[test]
fn test_key_from_code_folds_numpad() {
    // The numpad folds into the main row, matching the W3C definition of the
    // *logical* `KeyboardEvent.key`…
    for (code, key) in [
        (Code::NumpadEnter, Key::Enter),
        (Code::NumpadDivide, Key::Slash),
        (Code::NumpadSubtract, Key::Minus),
        (Code::NumpadDecimal, Key::Period),
        (Code::NumpadComma, Key::Comma),
        (Code::NumpadEqual, Key::Equals),
        (Code::Numpad0, Key::Num0),
        (Code::Numpad5, Key::Num5),
        (Code::Numpad9, Key::Num9),
    ] {
        assert_eq!(Key::from_code(code), Some(key));
    }

    // …while the physical codes themselves stay distinct, which is the whole
    // point of having a separate physical representation.
    assert_ne!(Code::NumpadEnter, Code::Enter);
    assert_ne!(Code::Numpad5, Code::Digit5);
    assert_ne!(Code::NumpadDivide, Code::Slash);

    // Both sides of a fold agree on the logical key:
    assert_eq!(
        Key::from_code(Code::Enter),
        Key::from_code(Code::NumpadEnter)
    );
    assert_eq!(Key::from_code(Code::Digit5), Key::from_code(Code::Numpad5));

    // `NumpadAdd` is `+`, which is *not* the same logical key as `=`:
    assert_eq!(Key::from_code(Code::NumpadAdd), Some(Key::Plus));
    assert_eq!(Key::from_code(Code::Equal), Some(Key::Equals));

    // egui has no logical key for these physical positions:
    assert_eq!(Key::from_code(Code::NumpadMultiply), None);
    assert_eq!(Key::from_code(Code::CapsLock), None);
}

#[test]
fn test_key_from_code_matches_from_name() {
    // `Key::from_name` accepts W3C `code` spellings for backwards compatibility.
    // Where it does, it must agree with `Key::from_code`, so that integrations
    // get the same logical key whichever path they take.
    for code in [
        Code::Enter,
        Code::NumpadEnter,
        Code::Numpad0,
        Code::Digit0,
        Code::KeyA,
        Code::KeyZ,
        Code::ArrowLeft,
        Code::F1,
        Code::F35,
        Code::IntlBackslash,
        Code::BracketLeft,
        Code::Backquote,
    ] {
        let name = code.to_string();
        assert_eq!(
            Key::from_code(code),
            Key::from_name(&name),
            "`Key::from_code({code:?})` disagrees with `Key::from_name({name:?})`"
        );
    }

    // The ⌘/Windows keys are `Meta*` in the W3C spec but `Super*` in egui:
    assert_eq!(Key::from_code(Code::MetaLeft), Some(Key::SuperLeft));
    assert_eq!(Key::from_code(Code::MetaRight), Some(Key::SuperRight));
}
