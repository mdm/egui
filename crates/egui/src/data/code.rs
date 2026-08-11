//! Physical key positions.

/// The physical position of a key on the keyboard, ignoring the active keymap.
///
/// This is a re-export of [`keyboard_types::Code`], which implements the
/// [W3C UI Events `KeyboardEvent.code`][spec] value set. Unlike [`crate::Key`],
/// which is *logical* and therefore folds physically distinct keys together
/// (`Code::NumpadEnter` and `Code::Enter` both produce [`crate::NamedKey::Enter`]),
/// `Code` keeps every physical key distinct.
///
/// Use this when the position of the key matters more than its label — for
/// instance in games, where WASD should stay under the same fingers on an
/// AZERTY layout, or when you want the numpad Enter to do something different
/// from the main Enter.
///
/// Use [`crate::Key`] for anything the user thinks of by name, such as
/// keyboard shortcuts.
///
/// [spec]: https://www.w3.org/TR/uievents-code/
pub use keyboard_types::Code;
