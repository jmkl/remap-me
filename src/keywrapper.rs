use rdev::Key;

#[derive(Debug)]
pub struct KeyWrapper(pub Key);

impl std::fmt::Display for KeyWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r: &str = match &self.0 {
            Key::Alt | Key::AltGr => "A",
            Key::ControlLeft | Key::ControlRight => "C",
            Key::MetaLeft | Key::MetaRight => "M",
            Key::ShiftLeft | Key::ShiftRight => "S",
            Key::Backspace => "backspace",
            Key::CapsLock => "capslock",
            Key::Delete => "delete",
            Key::DownArrow => "down",
            Key::End => "end",
            Key::Escape => "escape",
            Key::F1 => "f1",
            Key::F10 => "f10",
            Key::F11 => "f11",
            Key::F12 => "f12",
            Key::F2 => "f2",
            Key::F3 => "f3",
            Key::F4 => "f4",
            Key::F5 => "f5",
            Key::F6 => "f6",
            Key::F7 => "f7",
            Key::F8 => "f8",
            Key::F9 => "f9",
            Key::F13 => "f13",
            Key::F14 => "f14",
            Key::F15 => "f15",
            Key::F16 => "f16",
            Key::F17 => "f17",
            Key::F18 => "f18",
            Key::F19 => "f19",
            Key::F20 => "f20",
            Key::F21 => "f21",
            Key::F22 => "f22",
            Key::F23 => "f23",
            Key::F24 => "f24",
            Key::Home => "home",
            Key::LeftArrow => "left",
            Key::PageDown => "pagedown",
            Key::PageUp => "pageup",
            Key::Return => "return",
            Key::RightArrow => "right",
            Key::Space => "space",
            Key::Tab => "tab",
            Key::UpArrow => "up",
            Key::PrintScreen => "printscreen",
            Key::ScrollLock => "scrolllock",
            Key::Pause => "pause",
            Key::NumLock => "numlock",
            Key::BackQuote => "`",
            Key::Num1 => "1",
            Key::Num2 => "2",
            Key::Num3 => "3",
            Key::Num4 => "4",
            Key::Num5 => "5",
            Key::Num6 => "6",
            Key::Num7 => "7",
            Key::Num8 => "8",
            Key::Num9 => "9",
            Key::Num0 => "0",
            Key::Minus => "minus",
            Key::Equal => "=",
            Key::KeyQ => "q",
            Key::KeyW => "w",
            Key::KeyE => "e",
            Key::KeyR => "r",
            Key::KeyT => "t",
            Key::KeyY => "y",
            Key::KeyU => "u",
            Key::KeyI => "i",
            Key::KeyO => "o",
            Key::KeyP => "p",
            Key::LeftBracket => "[",
            Key::RightBracket => "]",
            Key::KeyA => "a",
            Key::KeyS => "s",
            Key::KeyD => "d",
            Key::KeyF => "f",
            Key::KeyG => "g",
            Key::KeyH => "h",
            Key::KeyJ => "j",
            Key::KeyK => "k",
            Key::KeyL => "l",
            Key::SemiColon => ";",
            Key::Quote => "'",
            Key::BackSlash => "\\",
            Key::IntlBackslash => "intlbackslash",
            Key::KeyZ => "z",
            Key::KeyX => "x",
            Key::KeyC => "c",
            Key::KeyV => "v",
            Key::KeyB => "b",
            Key::KeyN => "n",
            Key::KeyM => "m",
            Key::Comma => ",",
            Key::Dot => ".",
            Key::Slash => "/",
            Key::Insert => "insert",
            Key::KpReturn => "kpreturn",
            Key::KpMinus => "kpminus",
            Key::KpPlus => "kpplus",
            Key::KpMultiply => "kpmultiply",
            Key::KpDivide => "kpdivide",
            Key::Kp0 => "kp0",
            Key::Kp1 => "kp1",
            Key::Kp2 => "kp2",
            Key::Kp3 => "kp3",
            Key::Kp4 => "kp4",
            Key::Kp5 => "kp5",
            Key::Kp6 => "kp6",
            Key::Kp7 => "kp7",
            Key::Kp8 => "kp8",
            Key::Kp9 => "kp9",
            Key::KpDelete => "kpdelete",
            Key::Function => "function",
            Key::ModKey =>"mod",
            _ => "unknown",
        };

        write!(f, "{}", r)
    }
}

pub fn string_to_key(key: &str) -> Key {
    match key {
        "C" => Key::ControlLeft,
        "M" => Key::MetaLeft,
        "A" => Key::Alt,
        "S" => Key::ShiftLeft,
        "backspace" => Key::Backspace,
        "capslock" => Key::CapsLock,
        "delete" => Key::Delete,
        "down" => Key::DownArrow,
        "end" => Key::End,
        "escape" => Key::Escape,
        "f1" => Key::F1,
        "f10" => Key::F2,
        "f11" => Key::F3,
        "f12" => Key::F4,
        "f2" => Key::F2,
        "f3" => Key::F3,
        "f4" => Key::F4,
        "f5" => Key::F5,
        "f6" => Key::F6,
        "f7" => Key::F7,
        "f8" => Key::F8,
        "f9" => Key::F9,
        "f13" => Key::F13,
        "f14" => Key::F14,
        "f15" => Key::F15,
        "f16" => Key::F16,
        "f17" => Key::F17,
        "f18" => Key::F18,
        "f19" => Key::F19,
        "f20" => Key::F20,
        "f21" => Key::F21,
        "f22" => Key::F22,
        "f23" => Key::F23,
        "f24" => Key::F24,
        "home" => Key::Home,
        "left" => Key::LeftArrow,
        "pagedown" => Key::PageDown,
        "pageup" => Key::PageUp,
        "return" => Key::Return,
        "right" => Key::RightArrow,
        "space" => Key::Space,
        "tab" => Key::Tab,
        "up" => Key::UpArrow,
        "printscreen" => Key::PrintScreen,
        "pause" => Key::Pause,
        "numlock" => Key::NumLock,
        "`" => Key::BackQuote,
        "1" => Key::Num1,
        "2" => Key::Num2,
        "3" => Key::Num3,
        "4" => Key::Num4,
        "5" => Key::Num5,
        "6" => Key::Num6,
        "7" => Key::Num7,
        "8" => Key::Num8,
        "9" => Key::Num9,
        "0" => Key::Num0,
        "minus" => Key::Minus,
        "=" => Key::Equal,
        "q" => Key::KeyQ,
        "w" => Key::KeyW,
        "e" => Key::KeyE,
        "r" => Key::KeyR,
        "t" => Key::KeyT,
        "y" => Key::KeyY,
        "u" => Key::KeyU,
        "i" => Key::KeyI,
        "o" => Key::KeyO,
        "p" => Key::KeyP,
        "[" => Key::LeftBracket,
        "]" => Key::RightBracket,
        "a" => Key::KeyA,
        "s" => Key::KeyS,
        "d" => Key::KeyD,
        "f" => Key::KeyF,
        "g" => Key::KeyG,
        "h" => Key::KeyH,
        "j" => Key::KeyJ,
        "k" => Key::KeyK,
        "l" => Key::KeyL,
        ";" => Key::SemiColon,
        "'" => Key::BackSlash,
        "z" => Key::KeyZ,
        "x" => Key::KeyX,
        "c" => Key::KeyC,
        "v" => Key::KeyV,
        "b" => Key::KeyB,
        "n" => Key::KeyN,
        "m" => Key::KeyM,
        "," => Key::Comma,
        "." => Key::Dot,
        "/" => Key::Slash,
        "insert" => Key::Insert,
        "kpreturn" => Key::Return,
        "kpminus" => Key::KpMinus,
        "kpplus" => Key::KpPlus,
        "kpmultiply" => Key::KpMultiply,
        "kpdivide" => Key::KpDivide,
        "kp0" => Key::Kp0,
        "kp1" => Key::Kp1,
        "kp2" => Key::Kp2,
        "kp3" => Key::Kp3,
        "kp4" => Key::Kp4,
        "kp5" => Key::Kp5,
        "kp6" => Key::Kp6,
        "kp7" => Key::Kp7,
        "kp8" => Key::Kp8,
        "kp9" => Key::Kp9,
        "kpdelete" => Key::Delete,
        "mod"=>Key::ModKey,
        _ => Key::Unknown(0)
    }
}
