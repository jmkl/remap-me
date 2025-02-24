# KEYBOARD REMAP

enable keymod by _double tapping_ the modifier key `RightControl` or by holding `RightAlt`



configuration file store in `keysetting.json`
```json
[
    {
        "key":"a",
        "function":"awesome_function",
    },
    {
        "key":"b",
        "function":"macros",
        "macros":["C-a","C-c","C-v"]
        
    }
]
```


### USAGE

```rust
use std::{thread::{self, sleep}, time::Duration};
use remap_me::{event::KeymapEvent, RemapMe, Key,kc,kc_macro, delay};

fn main(){
    let remaper = RemapMe::new();
    // spawning key receiver
    thread::spawn(||{
        let receiver = KeymapEvent::Receiver();
        loop{
            if let Ok(ev)= receiver.try_recv(){
                match ev{
                    KeymapEvent::Func(ev)=>match ev.function.as_str(){
                        "awesome_function"=>{
                            kc(vec![Key::F12])
                        }
                        "macros"=>{
                            kc_macro(ev.macros);
                        }
                        _=>{}
                    }
                    KeymapEvent::Mod(mc)=>{
                        let is_mod_active = mc.is_mod;
                    }
                    KeymapEvent::Key(kb)=>{
                        let pressed_keys = kb.keys;
                    }
                }
            }
            delay(10);
        }
    })
    remaper.spawn();
}
```

```
cargo run --example simple
```

### Key's List
| string        | Key |
|------------   |-----------|
| C              | Key::ControlLeft |
| M              | Key::MetaLeft |
| A              | Key::Alt |
| S              | Key::ShiftLeft |
| backspace      | Key::Backspace |
| capslock       | Key::CapsLock |
| delete         | Key::Delete |
| down           | Key::DownArrow |
| end            | Key::End |
| escape         | Key::Escape |
| f1             | Key::F1 |
| f10            | Key::F2 |
| f11            | Key::F3 |
| f12            | Key::F4 |
| f2             | Key::F2 |
| f3             | Key::F3 |
| f4             | Key::F4 |
| f5             | Key::F5 |
| f6             | Key::F6 |
| f7             | Key::F7 |
| f8             | Key::F8 |
| f9             | Key::F9 |
| f13            | Key::F13 |
| f14            | Key::F14 |
| f15            | Key::F15 |
| f16            | Key::F16 |
| f17            | Key::F17 |
| f18            | Key::F18 |
| f19            | Key::F19 |
| f20            | Key::F20 |
| f21            | Key::F21 |
| f22            | Key::F22 |
| f23            | Key::F23 |
| f24            | Key::F24 |
| home           | Key::Home |
| left           | Key::LeftArrow |
| pagedown       | Key::PageDown |
| pageup         | Key::PageUp |
| return         | Key::Return |
| right          | Key::RightArrow |
| space          | Key::Space |
| tab            | Key::Tab |
| up             | Key::UpArrow |
| printscreen    | Key::PrintScreen |
| pause          | Key::Pause |
| numlock        | Key::NumLock |
| `              | Key::BackQuote |
| 1              | Key::Num1 |
| 2              | Key::Num2 |
| 3              | Key::Num3 |
| 4              | Key::Num4 |
| 5              | Key::Num5 |
| 6              | Key::Num6 |
| 7              | Key::Num7 |
| 8              | Key::Num8 |
| 9              | Key::Num9 |
| 0              | Key::Num0 |
| minus          | Key::Minus |
| =              | Key::Equal |
| q              | Key::KeyQ |
| w              | Key::KeyW |
| e              | Key::KeyE |
| r              | Key::KeyR |
| t              | Key::KeyT |
| y              | Key::KeyY |
| u              | Key::KeyU |
| i              | Key::KeyI |
| o              | Key::KeyO |
| p              | Key::KeyP |
| [              | Key::LeftBracket |
| ]              | Key::RightBracket |
| a              | Key::KeyA |
| s              | Key::KeyS |
| d              | Key::KeyD |
| f              | Key::KeyF |
| g              | Key::KeyG |
| h              | Key::KeyH |
| j              | Key::KeyJ |
| k              | Key::KeyK |
| l              | Key::KeyL |
| ;              | Key::SemiColon |
| '              | Key::BackSlash |
| z              | Key::KeyZ |
| x              | Key::KeyX |
| c              | Key::KeyC |
| v              | Key::KeyV |
| b              | Key::KeyB |
| n              | Key::KeyN |
| m              | Key::KeyM |
| ,              | Key::Comma |
| .              | Key::Dot |
| /              | Key::Slash |
| insert         | Key::Insert |
| kpreturn       | Key::Return |
| kpminus        | Key::KpMinus |
| kpplus         | Key::KpPlus |
| kpmultiply     | Key::KpMultiply |
| kpdivide       | Key::KpDivide |
| kp0            | Key::Kp0 |
| kp1            | Key::Kp1 |
| kp2            | Key::Kp2 |
| kp3            | Key::Kp3 |
| kp4            | Key::Kp4 |
| kp5            | Key::Kp5 |
| kp6            | Key::Kp6 |
| kp7            | Key::Kp7 |
| kp8            | Key::Kp8 |
| kp9            | Key::Kp9 |
| kpdelete       | Key::Delete |
