use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum ShortcutModifier {
    Ctrl,
    Alt,
    Shift,
    Super,
}

impl fmt::Display for ShortcutModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShortcutModifier::Ctrl => {
                write!(f, "Ctrl")
            }
            ShortcutModifier::Alt => {
                write!(f, "Alt")
            }
            ShortcutModifier::Shift => {
                write!(f, "Shift")
            }
            ShortcutModifier::Super => {
                write!(f, "Super")
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum ShortcutKey {
    C,
    I,
    R,
    Q,
}

impl fmt::Display for ShortcutKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShortcutKey::C => {
                write!(f, "C")
            }
            ShortcutKey::I => {
                write!(f, "I")
            }
            ShortcutKey::R => {
                write!(f, "R")
            }
            ShortcutKey::Q => {
                write!(f, "Q")
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ShortcutEvent {
    pub shortcut_modifier: ShortcutModifier,
    pub shortcut_key: ShortcutKey,
}

impl ShortcutEvent {
    pub fn company() -> ShortcutEvent {
        ShortcutEvent {
            shortcut_key: ShortcutKey::C,
            shortcut_modifier: ShortcutModifier::Alt,
        }
    }
    pub fn role() -> ShortcutEvent {
        ShortcutEvent {
            shortcut_modifier: ShortcutModifier::Alt,
            shortcut_key: ShortcutKey::R,
        }
    }
    pub fn interview() -> ShortcutEvent {
        ShortcutEvent {
            shortcut_key: ShortcutKey::I,
            shortcut_modifier: ShortcutModifier::Alt,
        }
    }
    pub fn questions() -> ShortcutEvent {
        ShortcutEvent {
            shortcut_key: ShortcutKey::Q,
            shortcut_modifier: ShortcutModifier::Alt,
        }
    }
}

impl fmt::Display for ShortcutEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}", self.shortcut_modifier, self.shortcut_key)
    }
}
