use super::Code;

/// A logical keyboard key, i.e. the value the key produces after applying the keymap.
///
/// This is a re-export of [`keyboard_types::Key`], which implements the
/// [W3C UI Events `KeyboardEvent.key`][spec] value set. It is either a
/// [`Key::Named`] key such as [`NamedKey::Enter`], or a [`Key::Character`]
/// holding the text the key produces (`"a"`, `"5"`, `" "` for space).
///
/// Being *logical*, it deliberately does not distinguish keys that produce the
/// same value: the numpad Enter is [`NamedKey::Enter`], the left and right Shift
/// are both [`NamedKey::Shift`], and numpad `5` is `Character("5")`. When the
/// physical key matters — for games, or to treat the numpad separately — use
/// [`Code`] instead, via the `physical_key` field of [`crate::Event::Key`].
///
/// Most helpers live on the [`KeyExt`] extension trait.
///
/// [spec]: https://www.w3.org/TR/uievents-key/
pub use keyboard_types::{Key, NamedKey};

/// Helpers for working with [`Key`].
///
/// `keyboard_types::Key` is a foreign type, so egui's conveniences live here.
/// Bring the trait into scope to use them:
///
/// ```
/// use egui::{Key, KeyExt as _, NamedKey};
///
/// let key = Key::character('a');
/// assert!(key.is_char('A')); // case-insensitive
/// assert_eq!(Key::from_name("Enter"), Some(Key::Named(NamedKey::Enter)));
/// ```
pub trait KeyExt: Sized {
    /// Build a [`Key::Character`] from a single character.
    ///
    /// Characters are stored lowercased so that `a` and `A` compare equal;
    /// use [`Self::is_char`] or [`Self::matches`] to compare.
    fn character(c: char) -> Self;

    /// Parse a key name, e.g. `"Enter"`, `"ArrowLeft"`, `"a"` or `"+"`.
    ///
    /// Accepts the names produced by a web browser's `KeyboardEvent.key`,
    /// plus a few legacy aliases (`"Esc"`, `"Left"`, `"Return"`).
    fn from_name(name: &str) -> Option<Self>;

    /// The *logical* key that a physical key produces on a US keyboard layout.
    ///
    /// Folds the numpad into the main row, matching the W3C definition of
    /// `KeyboardEvent.key`: [`Code::NumpadEnter`] produces [`NamedKey::Enter`],
    /// and [`Code::Numpad5`] produces `Character("5")`.
    ///
    /// ```
    /// # use egui::{Code, Key, KeyExt as _, NamedKey};
    /// assert_eq!(Key::from_code(Code::NumpadEnter), Some(Key::Named(NamedKey::Enter)));
    /// assert_eq!(Key::from_code(Code::Numpad5), Some(Key::character('5')));
    /// ```
    ///
    /// Integrations use this as a fallback when the platform cannot report a logical
    /// key, e.g. on layouts without Latin characters, so that standard shortcuts such
    /// as `Ctrl` + `V` keep working. See <https://github.com/emilk/egui/issues/3653>.
    fn from_code(code: Code) -> Option<Self>;

    /// The [`NamedKey`], if this is one.
    fn named(&self) -> Option<NamedKey>;

    /// The single character this key produces, lowercased.
    ///
    /// `None` for named keys and for multi-character values (e.g. dead keys).
    fn as_char(&self) -> Option<char>;

    /// Is this the given named key?
    fn is_named(&self, named: NamedKey) -> bool;

    /// Does this key produce the given character, ignoring case?
    fn is_char(&self, c: char) -> bool;

    /// Compare two keys, ignoring the case of [`Key::Character`] values.
    ///
    /// This matters for shortcuts: holding shift turns `Character("s")` into
    /// `Character("S")`, but `Cmd+Shift+S` should still match a `S` binding.
    fn matches(&self, other: &Self) -> bool;

    /// A human-readable name, e.g. `"Enter"` or `"A"`.
    fn name(&self) -> String;

    /// A symbol if there is a good one, else the same as [`Self::name`].
    fn symbol_or_name(&self) -> String;
}

impl KeyExt for Key {
    fn character(c: char) -> Self {
        Self::Character(c.to_lowercase().collect())
    }

    fn from_name(name: &str) -> Option<Self> {
        // Legacy / alternative spellings first, then the spec names.
        let named = match name {
            "Down" | "⏷" => NamedKey::ArrowDown,
            "Left" | "⏴" => NamedKey::ArrowLeft,
            "Right" | "⏵" => NamedKey::ArrowRight,
            "Up" | "⏶" => NamedKey::ArrowUp,
            "Esc" => NamedKey::Escape,
            "Return" => NamedKey::Enter,
            _ => {
                if let Ok(named) = name.parse::<NamedKey>() {
                    named
                } else {
                    // A character key. The spec spells space as `" "`.
                    let mut chars = name.chars();
                    let c = chars.next()?;
                    return chars.next().is_none().then(|| Self::character(c));
                }
            }
        };
        Some(Self::Named(named))
    }

    fn from_code(code: Code) -> Option<Self> {
        let named = match code {
            Code::ArrowDown => NamedKey::ArrowDown,
            Code::ArrowLeft => NamedKey::ArrowLeft,
            Code::ArrowRight => NamedKey::ArrowRight,
            Code::ArrowUp => NamedKey::ArrowUp,

            Code::Escape => NamedKey::Escape,
            Code::Tab => NamedKey::Tab,
            Code::Backspace => NamedKey::Backspace,
            Code::Enter | Code::NumpadEnter => NamedKey::Enter,

            Code::Insert => NamedKey::Insert,
            Code::Help => NamedKey::Help,
            Code::Delete => NamedKey::Delete,
            Code::Home => NamedKey::Home,
            Code::End => NamedKey::End,
            Code::PageUp => NamedKey::PageUp,
            Code::PageDown => NamedKey::PageDown,

            Code::Copy => NamedKey::Copy,
            Code::Cut => NamedKey::Cut,
            Code::Paste => NamedKey::Paste,

            Code::BrowserBack => NamedKey::BrowserBack,
            Code::ContextMenu => NamedKey::ContextMenu,
            Code::CapsLock => NamedKey::CapsLock,
            Code::NumLock => NamedKey::NumLock,
            Code::ScrollLock => NamedKey::ScrollLock,
            Code::PrintScreen => NamedKey::PrintScreen,
            Code::Pause => NamedKey::Pause,

            // The logical key carries no left/right distinction — that is what
            // the physical `Code` is for.
            Code::ShiftLeft | Code::ShiftRight => NamedKey::Shift,
            Code::ControlLeft | Code::ControlRight => NamedKey::Control,
            Code::AltLeft | Code::AltRight => NamedKey::Alt,
            Code::MetaLeft | Code::MetaRight => NamedKey::Meta,

            Code::F1 => NamedKey::F1,
            Code::F2 => NamedKey::F2,
            Code::F3 => NamedKey::F3,
            Code::F4 => NamedKey::F4,
            Code::F5 => NamedKey::F5,
            Code::F6 => NamedKey::F6,
            Code::F7 => NamedKey::F7,
            Code::F8 => NamedKey::F8,
            Code::F9 => NamedKey::F9,
            Code::F10 => NamedKey::F10,
            Code::F11 => NamedKey::F11,
            Code::F12 => NamedKey::F12,
            Code::F13 => NamedKey::F13,
            Code::F14 => NamedKey::F14,
            Code::F15 => NamedKey::F15,
            Code::F16 => NamedKey::F16,
            Code::F17 => NamedKey::F17,
            Code::F18 => NamedKey::F18,
            Code::F19 => NamedKey::F19,
            Code::F20 => NamedKey::F20,
            Code::F21 => NamedKey::F21,
            Code::F22 => NamedKey::F22,
            Code::F23 => NamedKey::F23,
            Code::F24 => NamedKey::F24,
            Code::F25 => NamedKey::F25,
            Code::F26 => NamedKey::F26,
            Code::F27 => NamedKey::F27,
            Code::F28 => NamedKey::F28,
            Code::F29 => NamedKey::F29,
            Code::F30 => NamedKey::F30,
            Code::F31 => NamedKey::F31,
            Code::F32 => NamedKey::F32,
            Code::F33 => NamedKey::F33,
            Code::F34 => NamedKey::F34,
            Code::F35 => NamedKey::F35,

            // Character-producing keys, as they behave on a US layout.
            // NOTE: `:`, `|`, `?`, `!`, `{`, `}` have no physical key of their own —
            // they are shifted keys, so they never appear here.
            _ => {
                let c = match code {
                    Code::Space => ' ',
                    Code::Comma | Code::NumpadComma => ',',
                    Code::Minus | Code::NumpadSubtract => '-',
                    Code::Period | Code::NumpadDecimal => '.',
                    Code::NumpadAdd => '+',
                    Code::Equal | Code::NumpadEqual => '=',
                    Code::Semicolon => ';',
                    Code::Backslash => '\\',
                    Code::Slash | Code::NumpadDivide => '/',
                    Code::NumpadMultiply => '*',
                    Code::BracketLeft => '[',
                    Code::BracketRight => ']',
                    Code::Backquote => '`',
                    Code::Quote => '\'',

                    Code::Digit0 | Code::Numpad0 => '0',
                    Code::Digit1 | Code::Numpad1 => '1',
                    Code::Digit2 | Code::Numpad2 => '2',
                    Code::Digit3 | Code::Numpad3 => '3',
                    Code::Digit4 | Code::Numpad4 => '4',
                    Code::Digit5 | Code::Numpad5 => '5',
                    Code::Digit6 | Code::Numpad6 => '6',
                    Code::Digit7 | Code::Numpad7 => '7',
                    Code::Digit8 | Code::Numpad8 => '8',
                    Code::Digit9 | Code::Numpad9 => '9',

                    Code::KeyA => 'a',
                    Code::KeyB => 'b',
                    Code::KeyC => 'c',
                    Code::KeyD => 'd',
                    Code::KeyE => 'e',
                    Code::KeyF => 'f',
                    Code::KeyG => 'g',
                    Code::KeyH => 'h',
                    Code::KeyI => 'i',
                    Code::KeyJ => 'j',
                    Code::KeyK => 'k',
                    Code::KeyL => 'l',
                    Code::KeyM => 'm',
                    Code::KeyN => 'n',
                    Code::KeyO => 'o',
                    Code::KeyP => 'p',
                    Code::KeyQ => 'q',
                    Code::KeyR => 'r',
                    Code::KeyS => 's',
                    Code::KeyT => 't',
                    Code::KeyU => 'u',
                    Code::KeyV => 'v',
                    Code::KeyW => 'w',
                    Code::KeyX => 'x',
                    Code::KeyY => 'y',
                    Code::KeyZ => 'z',

                    _ => return None,
                };
                return Some(Self::character(c));
            }
        };
        Some(Self::Named(named))
    }

    fn named(&self) -> Option<NamedKey> {
        match self {
            Self::Named(named) => Some(*named),
            Self::Character(_) => None,
        }
    }

    fn as_char(&self) -> Option<char> {
        match self {
            Self::Character(s) => {
                let mut chars = s.chars();
                let c = chars.next()?;
                chars.next().is_none().then(|| c.to_ascii_lowercase())
            }
            Self::Named(_) => None,
        }
    }

    fn is_named(&self, named: NamedKey) -> bool {
        self.named() == Some(named)
    }

    fn is_char(&self, c: char) -> bool {
        self.as_char() == Some(c.to_ascii_lowercase())
    }

    fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Named(a), Self::Named(b)) => a == b,
            (Self::Character(a), Self::Character(b)) => a.eq_ignore_ascii_case(b),
            _ => false,
        }
    }

    fn name(&self) -> String {
        match self {
            Self::Named(named) => named.to_string(),
            Self::Character(s) => {
                if s == " " {
                    "Space".to_owned()
                } else {
                    s.to_uppercase()
                }
            }
        }
    }

    fn symbol_or_name(&self) -> String {
        match self {
            Self::Named(NamedKey::ArrowDown) => "⏷".to_owned(),
            Self::Named(NamedKey::ArrowLeft) => "⏴".to_owned(),
            Self::Named(NamedKey::ArrowRight) => "⏵".to_owned(),
            Self::Named(NamedKey::ArrowUp) => "⏶".to_owned(),
            _ => self.name(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_code_folds_numpad() {
        // The numpad folds into the main row, matching the W3C definition of the
        // *logical* `KeyboardEvent.key`…
        assert_eq!(
            Key::from_code(Code::NumpadEnter),
            Some(Key::Named(NamedKey::Enter))
        );
        assert_eq!(Key::from_code(Code::Numpad5), Some(Key::character('5')));
        assert_eq!(
            Key::from_code(Code::NumpadDivide),
            Some(Key::character('/'))
        );
        assert_eq!(Key::from_code(Code::NumpadAdd), Some(Key::character('+')));

        // …while the physical codes stay distinct, which is the whole point of
        // having a separate physical representation.
        assert_ne!(Code::NumpadEnter, Code::Enter);
        assert_ne!(Code::Numpad5, Code::Digit5);
        assert_eq!(
            Key::from_code(Code::Enter),
            Key::from_code(Code::NumpadEnter)
        );
        assert_eq!(Key::from_code(Code::Digit5), Key::from_code(Code::Numpad5));
    }

    #[test]
    fn from_code_drops_left_right_distinction() {
        // The logical key has no sides — that is what `Code` is for.
        assert_eq!(
            Key::from_code(Code::ShiftLeft),
            Key::from_code(Code::ShiftRight)
        );
        assert_eq!(
            Key::from_code(Code::MetaLeft),
            Some(Key::Named(NamedKey::Meta))
        );
        assert_ne!(Code::ShiftLeft, Code::ShiftRight);
    }

    #[test]
    fn character_keys_are_case_insensitive() {
        // Shift+S yields "S", but a `S` shortcut must still match.
        let lower = Key::Character("s".to_owned());
        let upper = Key::Character("S".to_owned());
        assert!(lower.matches(&upper));
        assert!(upper.is_char('s'));
        assert!(lower.is_char('S'));
        assert_eq!(upper.as_char(), Some('s'));
    }

    #[test]
    fn from_name_roundtrips() {
        for name in ["Enter", "ArrowLeft", "Escape", "Tab", "F5", "PageDown"] {
            let key = Key::from_name(name).unwrap_or_else(|| panic!("failed to parse {name:?}"));
            assert_eq!(key.name(), name);
        }
        // Legacy aliases:
        assert_eq!(Key::from_name("Esc"), Key::from_name("Escape"));
        assert_eq!(Key::from_name("Return"), Key::from_name("Enter"));
        assert_eq!(Key::from_name("Left"), Key::from_name("ArrowLeft"));
        // Characters:
        assert_eq!(Key::from_name("a"), Some(Key::character('a')));
        assert_eq!(Key::from_name("A"), Some(Key::character('a')));
        assert_eq!(Key::from_name("+"), Some(Key::character('+')));
        assert_eq!(Key::from_name(" "), Some(Key::character(' ')));
    }

    #[test]
    fn space_is_a_character_key() {
        // Per the W3C spec the space bar produces `" "`, not a named key.
        assert_eq!(Key::from_code(Code::Space), Some(Key::character(' ')));
        assert_eq!(Key::character(' ').name(), "Space");
    }
}
