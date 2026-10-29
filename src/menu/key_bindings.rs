use std::fmt::{Display, Formatter};
use minifb::Key;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum Keys {
    Key0 = 0,
    Key1 = 1,
    Key2 = 2,
    Key3 = 3,
    Key4 = 4,
    Key5 = 5,
    Key6 = 6,
    Key7 = 7,
    Key8 = 8,
    Key9 = 9,

    A = 10,
    B = 11,
    C = 12,
    D = 13,
    E = 14,
    F = 15,
    G = 16,
    H = 17,
    I = 18,
    J = 19,
    K = 20,
    L = 21,
    M = 22,
    N = 23,
    O = 24,
    P = 25,
    Q = 26,
    R = 27,
    S = 28,
    T = 29,
    U = 30,
    V = 31,
    W = 32,
    X = 33,
    Y = 34,
    Z = 35,

    F1,
    F2,
    F3,
    F4,
    F5,
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

    Down,
    Left,
    Right,
    Up,
    Apostrophe,
    Backquote,

    Backslash,
    Comma,
    Equal,
    LeftBracket,
    Minus,
    Period,
    RightBracket,
    Semicolon,

    Slash,
    Backspace,
    Delete,
    End,
    Enter,

    Escape,

    Home,
    Insert,
    Menu,

    PageDown,
    PageUp,

    Pause,
    Space,
    Tab,
    NumLock,
    CapsLock,
    ScrollLock,
    LeftShift,
    RightShift,
    LeftCtrl,
    RightCtrl,

    NumPad0,
    NumPad1,
    NumPad2,
    NumPad3,
    NumPad4,
    NumPad5,
    NumPad6,
    NumPad7,
    NumPad8,
    NumPad9,
    NumPadDot,
    NumPadSlash,
    NumPadAsterisk,
    NumPadMinus,
    NumPadPlus,
    NumPadEnter,

    LeftAlt,
    RightAlt,

    LeftSuper,
    RightSuper,

    /// Used when an Unknown key has been pressed
    Unknown,

    Count = 107,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionKey {
    Forwards {key: Key},
    Backwards {key: Key},
    Left {key: Key},
    Right {key: Key},
    Sprint {key: Key},
    Jump {key: Key},
    CrouchSlide {key: Key},
    RocketJump {key: Key},
    Interact {key: Key},
    CrosshairPosL {key: Key},
    CrosshairPosR {key: Key},
    TurnLeft{key: Key},
    TurnRight{key: Key},
    FineAim{key: Key},
    FireBullet{key: Key},
    FireArrow{key: Key},
    GodMode{key: Key},
    Lock{key: Key},
    Exit{key: Key},

}
impl ActionKey {
    pub fn get_key(&self) -> &Key {
        match self {
            ActionKey::Forwards { key }
            | ActionKey::Backwards { key }
            | ActionKey::Left { key }
            | ActionKey::Right { key }
            | ActionKey::Sprint { key }
            | ActionKey::Jump { key }
            | ActionKey::CrouchSlide { key }
            | ActionKey::RocketJump { key }
            | ActionKey::Interact { key }
            | ActionKey::CrosshairPosL { key }
            | ActionKey::CrosshairPosR { key }
            | ActionKey::TurnLeft { key }
            | ActionKey::TurnRight { key }
            | ActionKey::FineAim { key }
            | ActionKey::FireBullet { key }
            | ActionKey::FireArrow { key }
            | ActionKey::GodMode { key }
            | ActionKey::Lock { key }
            | ActionKey::Exit { key } => key,
        }
    }
    pub fn change_key(&mut self, new_key: &Key){
        match self {
            ActionKey::Forwards { key}
                |ActionKey::Backwards { key}
                |ActionKey::Right { key}
                |ActionKey::Left { key}
                |ActionKey::Jump { key}
                |ActionKey::Sprint { key}
                |ActionKey::CrouchSlide { key}
                |ActionKey::RocketJump { key}
                |ActionKey::Interact { key}
                |ActionKey::CrosshairPosL { key}
                |ActionKey::CrosshairPosR { key}
                |ActionKey::TurnLeft { key}
                |ActionKey::TurnRight { key}
                |ActionKey::FineAim { key}
                |ActionKey::FireBullet { key}
                |ActionKey::FireArrow { key}
                |ActionKey::GodMode { key}
                |ActionKey::Lock { key}
                |ActionKey::Exit { key} => *key = *new_key,

        }
    }
}
// impl Display for ActionKey{
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.key_to_str())
//     }
// }

pub fn get_key_str(key: &Key) -> &str {
    let str = match key {
        Key::Key0 => "0",
        Key::Key1 => "1",
        Key::Key2 => "2",
        Key::Key3 => "3",
        Key::Key4 => "4",
        Key::Key5 => "5",
        Key::Key6 => "6",
        Key::Key7 => "7",
        Key::Key8 => "8",
        Key::Key9 => "9",

        Key::A => "A",
        Key::B => "B",
        Key::C => "C",
        Key::D => "D",
        Key::E => "E",
        Key::F => "F",
        Key::G => "G",
        Key::H => "H",
        Key::I => "I",
        Key::J => "J",
        Key::K => "K",
        Key::L => "L",
        Key::M => "M",
        Key::N => "N",
        Key::O => "O",
        Key::P => "P",
        Key::Q => "Q",
        Key::R => "R",
        Key::S => "S",
        Key::T => "T",
        Key::U => "U",
        Key::V => "V",
        Key::W => "W",
        Key::X => "X",
        Key::Y => "Y",
        Key::Z => "Z",

        Key::F1 => "F1",
        Key::F2 => "F2",
        Key::F3 => "F3",
        Key::F4 => "F4",
        Key::F5 => "F5",
        Key::F6 => "F6",
        Key::F7 => "F7",
        Key::F8 => "F8",
        Key::F9 => "F9",
        Key::F10 => "F10",
        Key::F11 => "F11",
        Key::F12 => "F12",
        Key::F13 => "F13",
        Key::F14 => "F14",
        Key::F15 => "F15",

        Key::Down => "Down",
        Key::Left => "Left",
        Key::Right => "Right",
        Key::Up => "Up",

        Key::Apostrophe => "'",
        Key::Backquote => "`",
        Key::Backslash => "\\",
        Key::Comma => ",",
        Key::Equal => "=",
        Key::LeftBracket => "[",
        Key::Minus => "-",
        Key::Period => ".",
        Key::RightBracket => "]",
        Key::Semicolon => ";",
        Key::Slash => "/",

        Key::Backspace => "Backspace",
        Key::Delete => "Delete",
        Key::End => "End",
        Key::Enter => "Enter",
        Key::Escape => "Escape",
        Key::Home => "Home",
        Key::Insert => "Insert",
        Key::Menu => "Menu",

        Key::PageDown => "Page Down",
        Key::PageUp => "Page Up",

        Key::Pause => "Pause",
        Key::Space => "Space",
        Key::Tab => "Tab",
        Key::NumLock => "Num Lock",
        Key::CapsLock => "Caps Lock",
        Key::ScrollLock => "Scroll Lock",

        Key::LeftShift => "Left Shift",
        Key::RightShift => "Right Shift",
        Key::LeftCtrl => "Left Ctrl",
        Key::RightCtrl => "Right Ctrl",

        Key::NumPad0 => "NumPad 0",
        Key::NumPad1 => "NumPad 1",
        Key::NumPad2 => "NumPad 2",
        Key::NumPad3 => "NumPad 3",
        Key::NumPad4 => "NumPad 4",
        Key::NumPad5 => "NumPad 5",
        Key::NumPad6 => "NumPad 6",
        Key::NumPad7 => "NumPad 7",
        Key::NumPad8 => "NumPad 8",
        Key::NumPad9 => "NumPad 9",

        Key::NumPadDot => "NumPad .",
        Key::NumPadSlash => "NumPad /",
        Key::NumPadAsterisk => "NumPad *",
        Key::NumPadMinus => "NumPad -",
        Key::NumPadPlus => "NumPad +",
        Key::NumPadEnter => "NumPad Enter",

        Key::LeftAlt => "Left Alt",
        Key::RightAlt => "Right Alt",

        Key::LeftSuper => "Left Super",
        Key::RightSuper => "Right Super",

        Key::Unknown => "Unknown",
        Key::Count => "Count",
    };
    &str
}
impl Display for Keys{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let str = match self {
                Keys::Key0 => "0",
                Keys::Key1 => "1",
                Keys::Key2 => "2",
                Keys::Key3 => "3",
                Keys::Key4 => "4",
                Keys::Key5 => "5",
                Keys::Key6 => "6",
                Keys::Key7 => "7",
                Keys::Key8 => "8",
                Keys::Key9 => "9",

                Keys::A => "A",
                Keys::B => "B",
                Keys::C => "C",
                Keys::D => "D",
                Keys::E => "E",
                Keys::F => "F",
                Keys::G => "G",
                Keys::H => "H",
                Keys::I => "I",
                Keys::J => "J",
                Keys::K => "K",
                Keys::L => "L",
                Keys::M => "M",
                Keys::N => "N",
                Keys::O => "O",
                Keys::P => "P",
                Keys::Q => "Q",
                Keys::R => "R",
                Keys::S => "S",
                Keys::T => "T",
                Keys::U => "U",
                Keys::V => "V",
                Keys::W => "W",
                Keys::X => "X",
                Keys::Y => "Y",
                Keys::Z => "Z",

                Keys::F1 => "F1",
                Keys::F2 => "F2",
                Keys::F3 => "F3",
                Keys::F4 => "F4",
                Keys::F5 => "F5",
                Keys::F6 => "F6",
                Keys::F7 => "F7",
                Keys::F8 => "F8",
                Keys::F9 => "F9",
                Keys::F10 => "F10",
                Keys::F11 => "F11",
                Keys::F12 => "F12",
                Keys::F13 => "F13",
                Keys::F14 => "F14",
                Keys::F15 => "F15",

                Keys::Down => "Down",
                Keys::Left => "Left",
                Keys::Right => "Right",
                Keys::Up => "Up",

                Keys::Apostrophe => "'",
                Keys::Backquote => "`",
                Keys::Backslash => "\\",
                Keys::Comma => ",",
                Keys::Equal => "=",
                Keys::LeftBracket => "[",
                Keys::Minus => "-",
                Keys::Period => ".",
                Keys::RightBracket => "]",
                Keys::Semicolon => ";",
                Keys::Slash => "/",

                Keys::Backspace => "Backspace",
                Keys::Delete => "Delete",
                Keys::End => "End",
                Keys::Enter => "Enter",
                Keys::Escape => "Escape",
                Keys::Home => "Home",
                Keys::Insert => "Insert",
                Keys::Menu => "Menu",

                Keys::PageDown => "Page Down",
                Keys::PageUp => "Page Up",

                Keys::Pause => "Pause",
                Keys::Space => "Space",
                Keys::Tab => "Tab",
                Keys::NumLock => "Num Lock",
                Keys::CapsLock => "Caps Lock",
                Keys::ScrollLock => "Scroll Lock",

                Keys::LeftShift => "Left Shift",
                Keys::RightShift => "Right Shift",
                Keys::LeftCtrl => "Left Ctrl",
                Keys::RightCtrl => "Right Ctrl",

                Keys::NumPad0 => "NumPad 0",
                Keys::NumPad1 => "NumPad 1",
                Keys::NumPad2 => "NumPad 2",
                Keys::NumPad3 => "NumPad 3",
                Keys::NumPad4 => "NumPad 4",
                Keys::NumPad5 => "NumPad 5",
                Keys::NumPad6 => "NumPad 6",
                Keys::NumPad7 => "NumPad 7",
                Keys::NumPad8 => "NumPad 8",
                Keys::NumPad9 => "NumPad 9",

                Keys::NumPadDot => "NumPad .",
                Keys::NumPadSlash => "NumPad /",
                Keys::NumPadAsterisk => "NumPad *",
                Keys::NumPadMinus => "NumPad -",
                Keys::NumPadPlus => "NumPad +",
                Keys::NumPadEnter => "NumPad Enter",

                Keys::LeftAlt => "Left Alt",
                Keys::RightAlt => "Right Alt",

                Keys::LeftSuper => "Left Super",
                Keys::RightSuper => "Right Super",

                Keys::Unknown => "Unknown",
                Keys::Count => "Count",
            };

            write!(f, "{str}")
        }
    }
