use crate::{Key, KeyExt as _};

use super::{ModifierNames, ModifierPattern};

/// A keyboard shortcut, e.g. `Ctrl+Alt+W`.
///
/// Can be used with [`crate::InputState::consume_shortcut`]
/// and [`crate::Context::format_shortcut`].
///
/// NOTE: this is not `Copy` and cannot be built in a `const`, because
/// [`Key::Character`] owns a `String`. Declare shared shortcuts with
/// [`std::sync::LazyLock`] instead of `const`.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct KeyboardShortcut {
    pub modifiers: ModifierPattern,

    pub logical_key: Key,
}

impl KeyboardShortcut {
    pub fn new(modifiers: ModifierPattern, logical_key: Key) -> Self {
        Self {
            modifiers,
            logical_key,
        }
    }

    pub fn format(&self, names: &ModifierNames<'_>, is_mac: bool) -> String {
        let mut s = names.format(&self.modifiers, is_mac);
        if !s.is_empty() {
            s += names.concat;
        }
        if names.is_short {
            s += &self.logical_key.symbol_or_name();
        } else {
            s += &self.logical_key.name();
        }
        s
    }
}

#[test]
fn format_kb_shortcut() {
    let cmd_shift_f = KeyboardShortcut::new(
        ModifierPattern::COMMAND | ModifierPattern::SHIFT,
        Key::character('f'),
    );
    assert_eq!(
        cmd_shift_f.format(&ModifierNames::NAMES, false),
        "Ctrl+Shift+F"
    );
    assert_eq!(
        cmd_shift_f.format(&ModifierNames::NAMES, true),
        "Shift+Cmd+F"
    );
    assert_eq!(cmd_shift_f.format(&ModifierNames::SYMBOLS, false), "⌃⇧F");
    assert_eq!(cmd_shift_f.format(&ModifierNames::SYMBOLS, true), "⇧⌘F");
}
