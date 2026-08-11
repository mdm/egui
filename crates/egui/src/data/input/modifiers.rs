use super::ModifierNames;
use crate::os::OperatingSystem;

/// State of the modifier keys. These must be fed to egui.
///
/// This is a re-export of [`keyboard_types::Modifiers`], i.e. the modifier set from the
/// [W3C UI Events spec][spec]. It records only what is physically down —
/// [`Modifiers::META`] is the ⌘ key on Mac and the Windows/Super key elsewhere.
///
/// It deliberately has no notion of a "command" key, because that is
/// platform-dependent and therefore not a property of the keyboard state.
/// To match a shortcut, build a [`ModifierPattern`] and use
/// [`ModifierPattern::matches_logically`] or [`ModifierPattern::matches_exact`],
/// which resolve "command" against the current [`OperatingSystem`].
///
/// To access the [`Modifiers`] you can use the [`crate::Context::input`] function
///
/// ```rust
/// # let ctx = egui::Context::default();
/// let modifiers = ctx.input(|i| i.modifiers);
/// ```
///
/// NOTE: For cross-platform uses, ALT+SHIFT is a bad combination of modifiers
/// as on mac that is how you type special characters,
/// so those key presses are usually not reported to egui.
///
/// [spec]: https://www.w3.org/TR/uievents-key/#keys-modifier
pub use keyboard_types::Modifiers;

/// The modifiers that a user actually holds down, as opposed to lock states
/// such as [`Modifiers::CAPS_LOCK`] or [`Modifiers::NUM_LOCK`].
///
/// egui never treats a lock state as "a modifier is down"; otherwise having Num Lock
/// on would, for instance, stop the arrow keys from moving keyboard focus.
const HELD_MODIFIERS: Modifiers = Modifiers::ALT
    .union(Modifiers::CONTROL)
    .union(Modifiers::SHIFT)
    .union(Modifiers::META);

/// Helpers on the raw modifier state that need to know the [`OperatingSystem`].
pub trait ModifiersExt {
    /// Is any of alt/ctrl/shift/meta held down?
    ///
    /// Lock states such as Caps Lock and Num Lock do not count.
    fn any(&self) -> bool;

    /// Is the platform's "command" key down? ⌘ on Mac, Ctrl elsewhere.
    fn command(&self, os: OperatingSystem) -> bool;

    /// Is the Mac ⌘ Command key down? Always `false` off Mac.
    fn mac_cmd(&self, os: OperatingSystem) -> bool;

    /// Is shift the only thing held down?
    fn shift_only(&self) -> bool;

    /// Is the "command" key the only thing held down (besides possibly ctrl on Mac)?
    fn command_only(&self, os: OperatingSystem) -> bool;

    /// Are alt, ctrl, shift and command all held down?
    fn all(&self, os: OperatingSystem) -> bool;
}

impl ModifiersExt for Modifiers {
    #[inline]
    fn any(&self) -> bool {
        self.intersects(HELD_MODIFIERS)
    }

    #[inline]
    fn command(&self, os: OperatingSystem) -> bool {
        if os.is_mac() {
            self.contains(Self::META)
        } else {
            self.contains(Self::CONTROL)
        }
    }

    #[inline]
    fn mac_cmd(&self, os: OperatingSystem) -> bool {
        os.is_mac() && self.contains(Self::META)
    }

    #[inline]
    fn shift_only(&self) -> bool {
        self.intersection(HELD_MODIFIERS) == Self::SHIFT
    }

    #[inline]
    fn command_only(&self, os: OperatingSystem) -> bool {
        !self.contains(Self::ALT) && !self.contains(Self::SHIFT) && self.command(os)
    }

    #[inline]
    fn all(&self, os: OperatingSystem) -> bool {
        self.contains(Self::ALT)
            && self.contains(Self::CONTROL)
            && self.contains(Self::SHIFT)
            && self.command(os)
    }
}

// ----------------------------------------------------------------------------

/// A pattern to match held [`Modifiers`] against, e.g. for a keyboard shortcut.
///
/// Unlike [`Modifiers`], this understands the platform-dependent "command" key,
/// which is what lets a single shortcut definition work on both Mac and elsewhere:
/// `ModifierPattern::COMMAND` is ⌘ on Mac and Ctrl on Windows/Linux.
///
/// Match with [`Self::matches_logically`] (usually what you want) or
/// [`Self::matches_exact`].
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ModifierPattern {
    /// Either of the alt keys are down (option ⌥ on Mac).
    pub alt: bool,

    /// Either of the control keys are down.
    /// When checking for keyboard shortcuts, consider using [`Self::command`] instead.
    pub ctrl: bool,

    /// Either of the shift keys are down.
    pub shift: bool,

    /// The Mac ⌘ Command key. Only ever matches on Mac.
    pub mac_cmd: bool,

    /// Matches the ⌘ Command key on Mac, and the Ctrl key elsewhere.
    ///
    /// This is so that egui can, for instance, select all text by checking for `command + A`
    /// and it will work on both Mac and Windows.
    pub command: bool,
}

impl core::fmt::Debug for ModifierPattern {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_none() {
            return write!(f, "ModifierPattern::NONE");
        }

        let Self {
            alt,
            ctrl,
            shift,
            mac_cmd,
            command,
        } = *self;

        let mut debug = f.debug_struct("ModifierPattern");
        if alt {
            debug.field("alt", &true);
        }
        if ctrl {
            debug.field("ctrl", &true);
        }
        if shift {
            debug.field("shift", &true);
        }
        if mac_cmd {
            debug.field("mac_cmd", &true);
        }
        if command {
            debug.field("command", &true);
        }
        debug.finish()
    }
}

impl ModifierPattern {
    pub const NONE: Self = Self {
        alt: false,
        ctrl: false,
        shift: false,
        mac_cmd: false,
        command: false,
    };

    pub const ALT: Self = Self {
        alt: true,
        ctrl: false,
        shift: false,
        mac_cmd: false,
        command: false,
    };
    pub const CTRL: Self = Self {
        alt: false,
        ctrl: true,
        shift: false,
        mac_cmd: false,
        command: false,
    };
    pub const SHIFT: Self = Self {
        alt: false,
        ctrl: false,
        shift: true,
        mac_cmd: false,
        command: false,
    };

    /// The Mac ⌘ Command key
    pub const MAC_CMD: Self = Self {
        alt: false,
        ctrl: false,
        shift: false,
        mac_cmd: true,
        command: false,
    };

    /// On Mac: ⌘ Command key, elsewhere: Ctrl key
    pub const COMMAND: Self = Self {
        alt: false,
        ctrl: false,
        shift: false,
        mac_cmd: false,
        command: true,
    };

    /// ```
    /// # use egui::ModifierPattern;
    /// assert_eq!(
    ///     ModifierPattern::CTRL | ModifierPattern::ALT,
    ///     ModifierPattern { ctrl: true, alt: true, ..Default::default() }
    /// );
    /// assert_eq!(
    ///     ModifierPattern::ALT.plus(ModifierPattern::CTRL),
    ///     ModifierPattern::CTRL.plus(ModifierPattern::ALT),
    /// );
    /// ```
    #[inline]
    pub const fn plus(self, rhs: Self) -> Self {
        Self {
            alt: self.alt | rhs.alt,
            ctrl: self.ctrl | rhs.ctrl,
            shift: self.shift | rhs.shift,
            mac_cmd: self.mac_cmd | rhs.mac_cmd,
            command: self.command | rhs.command,
        }
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        self == &Self::NONE
    }

    #[inline]
    pub fn any(&self) -> bool {
        !self.is_none()
    }

    /// Checks that the `ctrl/cmd` matches, and that the `shift/alt` of the pattern is a subset
    /// of the pressed keys.
    ///
    /// This means that if the pattern has not set `shift`, then the pressed modifiers can have
    /// `shift` set or not.
    ///
    /// The reason is that many logical keys require `shift` or `alt` on some keyboard layouts.
    /// For instance, in order to press `+` on an English keyboard, you need to press `shift` and `=`,
    /// but a Swedish keyboard has dedicated `+` key.
    /// So if you want to make a [`KeyboardShortcut`](crate::KeyboardShortcut) looking for `Cmd` + `+`, it makes sense
    /// to ignore the shift key.
    /// Similarly, the `Alt` key is sometimes used to type special characters.
    ///
    /// However, if the pattern explicitly requires the `shift` or `alt` keys
    /// to be pressed, then they must be pressed.
    ///
    /// # Example:
    /// ```
    /// # use egui::{ModifierPattern, Modifiers, os::OperatingSystem};
    /// # let pressed = Modifiers::empty();
    /// # let os = OperatingSystem::Windows;
    /// if (ModifierPattern::ALT | ModifierPattern::SHIFT).matches_logically(pressed, os) {
    ///     // Alt and Shift are pressed, but not ctrl/command
    /// }
    /// ```
    ///
    /// ## Behavior:
    /// ```
    /// # use egui::{ModifierPattern, Modifiers, os::OperatingSystem};
    /// # let win = OperatingSystem::Windows;
    /// # let mac = OperatingSystem::Mac;
    /// assert!(ModifierPattern::CTRL.matches_logically(Modifiers::CONTROL, win));
    /// assert!(!(ModifierPattern::CTRL | ModifierPattern::SHIFT).matches_logically(Modifiers::CONTROL, win));
    /// assert!(ModifierPattern::CTRL.matches_logically(Modifiers::CONTROL | Modifiers::SHIFT, win));
    /// // On Windows, `command` is Ctrl:
    /// assert!(ModifierPattern::COMMAND.matches_logically(Modifiers::CONTROL, win));
    /// // On Mac, `command` is ⌘ (META):
    /// assert!(ModifierPattern::COMMAND.matches_logically(Modifiers::META, mac));
    /// assert!(ModifierPattern::MAC_CMD.matches_logically(Modifiers::META, mac));
    /// // …but a Mac-only shortcut never matches off Mac:
    /// assert!(!ModifierPattern::MAC_CMD.matches_logically(Modifiers::META, win));
    /// ```
    pub fn matches_logically(&self, pressed: Modifiers, os: OperatingSystem) -> bool {
        if self.alt && !pressed.contains(Modifiers::ALT) {
            return false;
        }
        if self.shift && !pressed.contains(Modifiers::SHIFT) {
            return false;
        }

        self.cmd_ctrl_matches(pressed, os)
    }

    /// Check for equality but with proper handling of [`Self::command`].
    ///
    /// Note that this will require the `shift` and `alt` keys to match, even though
    /// these modifiers are sometimes required to produce some logical keys.
    /// For instance, to press `+` on an English keyboard, you need to press `shift` and `=`,
    /// but on a Swedish keyboard you can press the dedicated `+` key.
    /// Therefore, you often want to use [`Self::matches_logically`] instead.
    ///
    /// ## Behavior:
    /// ```
    /// # use egui::{ModifierPattern, Modifiers, os::OperatingSystem};
    /// # let win = OperatingSystem::Windows;
    /// assert!(ModifierPattern::CTRL.matches_exact(Modifiers::CONTROL, win));
    /// assert!(!ModifierPattern::CTRL.matches_exact(Modifiers::CONTROL | Modifiers::SHIFT, win));
    /// assert!(!(ModifierPattern::CTRL | ModifierPattern::SHIFT).matches_exact(Modifiers::CONTROL, win));
    /// ```
    pub fn matches_exact(&self, pressed: Modifiers, os: OperatingSystem) -> bool {
        // alt and shift must always match the pattern:
        if self.alt != pressed.contains(Modifiers::ALT)
            || self.shift != pressed.contains(Modifiers::SHIFT)
        {
            return false;
        }

        self.cmd_ctrl_matches(pressed, os)
    }

    /// Check if any of the modifiers match.
    ///
    /// Returns true if at least one modifier required by the pattern is pressed.
    ///
    /// ## Behavior:
    /// ```
    /// # use egui::{ModifierPattern, Modifiers, os::OperatingSystem};
    /// # let win = OperatingSystem::Windows;
    /// assert!(ModifierPattern::CTRL.matches_any(Modifiers::CONTROL, win));
    /// assert!((ModifierPattern::CTRL | ModifierPattern::SHIFT).matches_any(Modifiers::CONTROL, win));
    /// ```
    pub fn matches_any(&self, pressed: Modifiers, os: OperatingSystem) -> bool {
        if self.alt && pressed.contains(Modifiers::ALT) {
            return true;
        }
        if self.shift && pressed.contains(Modifiers::SHIFT) {
            return true;
        }
        if self.ctrl && pressed.contains(Modifiers::CONTROL) {
            return true;
        }
        if self.mac_cmd && pressed.mac_cmd(os) {
            return true;
        }
        if self.command && (pressed.contains(Modifiers::CONTROL) || pressed.command(os)) {
            return true;
        }
        false
    }

    /// Checks only cmd/ctrl, not alt/shift.
    ///
    /// This takes care to properly handle the difference between
    /// [`Self::ctrl`], [`Self::command`] and [`Self::mac_cmd`].
    pub fn cmd_ctrl_matches(&self, pressed: Modifiers, os: OperatingSystem) -> bool {
        let ctrl_down = pressed.contains(Modifiers::CONTROL);
        let command_down = pressed.command(os);

        if self.mac_cmd {
            // Mac-specific match:
            if !pressed.mac_cmd(os) {
                return false;
            }
            if self.ctrl != ctrl_down {
                return false;
            }
            return true;
        }

        if !self.ctrl && !self.command {
            // the pattern explicitly doesn't want any ctrl/command:
            return !ctrl_down && !command_down;
        }

        // if the pattern is looking for command, then `ctrl` may or may not be set depending on platform.
        // if the pattern is looking for `ctrl`, then `command` may or may not be set depending on platform.
        if self.ctrl && !ctrl_down {
            return false;
        }
        if self.command && !command_down {
            return false;
        }

        true
    }

    /// Whether another pattern is contained in this one, with proper handling of
    /// [`Self::command`].
    ///
    /// ```
    /// # use egui::ModifierPattern;
    /// assert!(ModifierPattern::default().contains(ModifierPattern::default()));
    /// assert!(ModifierPattern::CTRL.contains(ModifierPattern::default()));
    /// assert!(ModifierPattern::CTRL.contains(ModifierPattern::CTRL));
    /// assert!(ModifierPattern::CTRL.contains(ModifierPattern::COMMAND));
    /// assert!(ModifierPattern::MAC_CMD.contains(ModifierPattern::COMMAND));
    /// assert!(ModifierPattern::COMMAND.contains(ModifierPattern::MAC_CMD));
    /// assert!(ModifierPattern::COMMAND.contains(ModifierPattern::CTRL));
    /// assert!(!(ModifierPattern::ALT | ModifierPattern::CTRL).contains(ModifierPattern::SHIFT));
    /// assert!((ModifierPattern::CTRL | ModifierPattern::SHIFT).contains(ModifierPattern::CTRL));
    /// assert!(!ModifierPattern::CTRL.contains(ModifierPattern::CTRL | ModifierPattern::SHIFT));
    /// ```
    pub fn contains(&self, query: Self) -> bool {
        if query == Self::default() {
            return true;
        }

        let Self {
            alt,
            ctrl,
            shift,
            mac_cmd,
            command,
        } = *self;

        if alt && query.alt {
            return self.contains(Self {
                alt: false,
                ..query
            });
        }
        if shift && query.shift {
            return self.contains(Self {
                shift: false,
                ..query
            });
        }

        if (ctrl || command) && (query.ctrl || query.command) {
            return self.contains(Self {
                command: false,
                ctrl: false,
                ..query
            });
        }
        if (mac_cmd || command) && (query.mac_cmd || query.command) {
            return self.contains(Self {
                mac_cmd: false,
                command: false,
                ..query
            });
        }

        false
    }

    /// The concrete [`Modifiers`] a user would have to hold to satisfy this pattern
    /// on the given platform.
    ///
    /// Useful when synthesizing input, e.g. in tests: `COMMAND` becomes
    /// [`Modifiers::META`] on Mac and [`Modifiers::CONTROL`] elsewhere.
    ///
    /// ```
    /// # use egui::{ModifierPattern, Modifiers, os::OperatingSystem};
    /// assert_eq!(
    ///     ModifierPattern::COMMAND.to_modifiers(OperatingSystem::Mac),
    ///     Modifiers::META
    /// );
    /// assert_eq!(
    ///     ModifierPattern::COMMAND.to_modifiers(OperatingSystem::Windows),
    ///     Modifiers::CONTROL
    /// );
    /// ```
    pub fn to_modifiers(self, os: OperatingSystem) -> Modifiers {
        let mut m = Modifiers::empty();
        m.set(Modifiers::ALT, self.alt);
        m.set(Modifiers::CONTROL, self.ctrl);
        m.set(Modifiers::SHIFT, self.shift);
        if self.mac_cmd {
            m.insert(Modifiers::META);
        }
        if self.command {
            m.insert(if os.is_mac() {
                Modifiers::META
            } else {
                Modifiers::CONTROL
            });
        }
        m
    }

    /// Show the modifier names, e.g. `Ctrl+Shift` or `⇧⌘`.
    pub fn ui(&self, ui: &mut crate::Ui) {
        ui.label(ModifierNames::NAMES.format(self, ui.ctx().os().is_mac()));
    }
}

impl core::ops::BitOr for ModifierPattern {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        self.plus(rhs)
    }
}

impl core::ops::BitOrAssign for ModifierPattern {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAC: OperatingSystem = OperatingSystem::Mac;
    const WIN: OperatingSystem = OperatingSystem::Windows;

    #[test]
    fn command_resolves_per_platform() {
        // ⌘ on Mac:
        assert!(ModifierPattern::COMMAND.matches_logically(Modifiers::META, MAC));
        assert!(!ModifierPattern::COMMAND.matches_logically(Modifiers::CONTROL, MAC));

        // Ctrl elsewhere:
        assert!(ModifierPattern::COMMAND.matches_logically(Modifiers::CONTROL, WIN));
        // The Windows/Super key is NOT "command" off Mac. This is the bug that the
        // old web backend had, where Super set `mac_cmd` on every platform.
        assert!(!ModifierPattern::COMMAND.matches_logically(Modifiers::META, WIN));
    }

    #[test]
    fn mac_cmd_is_mac_only() {
        assert!(ModifierPattern::MAC_CMD.matches_logically(Modifiers::META, MAC));
        assert!(!ModifierPattern::MAC_CMD.matches_logically(Modifiers::META, WIN));
        assert!(!ModifierPattern::MAC_CMD.matches_logically(Modifiers::CONTROL, MAC));
    }

    #[test]
    fn ctrl_and_command_are_distinct_on_mac() {
        // On Mac, Ctrl is a real key separate from ⌘ — this is what makes the
        // emacs-style Ctrl-A/E/P/N/B/F bindings coexist with ⌘A (select all).
        assert!(ModifierPattern::CTRL.matches_logically(Modifiers::CONTROL, MAC));
        assert!(!ModifierPattern::COMMAND.matches_logically(Modifiers::CONTROL, MAC));
        assert!(!ModifierPattern::CTRL.matches_logically(Modifiers::META, MAC));
    }

    #[test]
    fn shift_and_alt_are_subsets_for_logical_match() {
        // Pattern without shift matches even when shift is held…
        assert!(
            ModifierPattern::CTRL.matches_logically(Modifiers::CONTROL | Modifiers::SHIFT, WIN)
        );
        // …but matches_exact does not.
        assert!(!ModifierPattern::CTRL.matches_exact(Modifiers::CONTROL | Modifiers::SHIFT, WIN));
    }

    #[test]
    fn none_pattern_requires_no_cmd_or_ctrl() {
        assert!(ModifierPattern::NONE.matches_logically(Modifiers::empty(), WIN));
        assert!(!ModifierPattern::NONE.matches_logically(Modifiers::CONTROL, WIN));
        assert!(!ModifierPattern::NONE.matches_logically(Modifiers::META, MAC));
    }

    #[test]
    fn lock_states_are_not_held_modifiers() {
        // Num Lock being on must not count as "a modifier is down", or it would
        // break arrow-key focus navigation.
        assert!(!Modifiers::NUM_LOCK.any());
        assert!(!Modifiers::CAPS_LOCK.any());
        assert!(Modifiers::SHIFT.any());
        assert!((Modifiers::SHIFT | Modifiers::CAPS_LOCK).shift_only());
    }
}
