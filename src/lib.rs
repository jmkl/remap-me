pub mod event;
mod keywrapper;
mod mod_double_tap;
mod mod_hold;
mod setting;
mod winutil;
use mod_double_tap::{DoubleTap, TapMode};

pub use keywrapper::{string_to_key, KeyWrapper};
use mod_hold::Hold;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rdev::SimulateError;
pub use rdev::{
    grab, simulate, Event, EventType, Key,
    Key::{F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24},
};
pub use setting::get_setting_path;
pub use setting::{KeySetting, Macro, MacroKey};
use std::{
    thread::{self, sleep},
    time::Duration,
};

static DT: Lazy<Mutex<DoubleTap>> = Lazy::new(|| Mutex::new(DoubleTap::new(200)));
static HOLD: Lazy<Mutex<Hold>> = Lazy::new(|| Mutex::new(Hold::new()));
static IS_MOD: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static ALL_KEYS: Lazy<Mutex<Vec<MacroKey>>> = Lazy::new(|| Mutex::new(Vec::new()));
static DT_KEY: Lazy<Mutex<Key>> = Lazy::new(|| Mutex::new(Key::ControlRight));
static HOLD_KEY: Lazy<Mutex<Key>> = Lazy::new(|| Mutex::new(Key::AltGr));
static KEYS: Lazy<Mutex<KeyState>> = Lazy::new(|| Mutex::new(KeyState::default()));

pub use event::{
    FunctionEvent, KeyPressEvent,
    KeymapEvent::{self, Func, Mod},
    ModChangeEvent,
};
pub use winutil::get_foreground_app;

enum KeyMode {
    Pressed,
    Released,
}

#[derive(Default, Debug)]
struct KeyState {
    keys: Vec<String>,
}
fn _simulate(ev: &EventType) {
    match simulate(ev) {
        Ok(()) => {}
        Err(SimulateError) => {
            eprintln!("Error simulating KeyPress {ev:?}")
        }
    }
}
fn kc_press(key: Key) {
    _simulate(&EventType::KeyPress(key));
    sleep(Duration::from_millis(10));
}
fn kc_release(key: Key) {
    _simulate(&EventType::KeyRelease(key));
    sleep(Duration::from_millis(10));
}
fn kc_click(key: Key) {
    _simulate(&EventType::KeyPress(key));
    sleep(Duration::from_millis(10));
    _simulate(&EventType::KeyRelease(key));
}

pub fn update_mod(is_mod: bool) {
    *IS_MOD.lock() = is_mod;
    KeymapEvent::send(Mod(ModChangeEvent { is_mod }));
}

pub fn kc_macro(macros: Option<Macro>) {
    if let Some(macros) = macros {
        let mods = macros
            .modifier
            .iter()
            .map(|x| string_to_key(x))
            .collect::<Vec<_>>();
        let keys = macros
            .keys
            .iter()
            .map(|x| string_to_key(x))
            .collect::<Vec<_>>();
        kc_mod(mods, keys);
    }
}

pub fn kc(keys: Vec<Key>) {
    update_mod(false);

    for &key in keys.iter() {
        kc_click(key);
    }

    update_mod(true);
}
pub fn delay(millis: u64) {
    sleep(Duration::from_millis(millis));
}

pub fn kc_mod(mods: Vec<Key>, keys: Vec<Key>) {
    update_mod(false);
    for &mod_ in mods.iter() {
        kc_press(mod_);
    }
    for &key in keys.iter() {
        kc_click(key);
    }
    for &mod_ in mods.iter() {
        kc_release(mod_);
    }
    update_mod(true);
}

fn send_original_hold_key() {
    thread::spawn(|| {
        let hold_key = *HOLD_KEY.lock();
        kc_click(hold_key);
        HOLD.lock().should_lock(false);
    });
}

fn send_original_doubletap_key() {
    thread::spawn(|| {
        let mod_key = *DT_KEY.lock();
        kc_click(mod_key);
        // let mut enigo = Enigo::new(&Settings::default()).unwrap();
        // enigo.key(enigo::Key::Escape, Click).unwrap();
        DT.lock().locked = false;
    });
}

fn is_mod() -> bool {
    *IS_MOD.lock()
}

fn sort_key(k: &str) -> String {
    let mut keys = k.split("-").collect::<Vec<_>>();
    keys.sort();
    keys.join("-")
}

fn update_keys(kw: KeyWrapper, mode: KeyMode, event: Event) -> Option<Event> {
    let mut sk = KEYS.lock();
    let key_str = kw.to_string();
    let all_keys = ALL_KEYS.lock();

    match mode {
        KeyMode::Pressed => {
            if is_mod() {
                if !sk.keys.contains(&key_str) {
                    sk.keys.push(key_str);
                    KeymapEvent::send(event::KeymapEvent::Key(KeyPressEvent {
                        keys: sk.keys.clone(),
                    }));
                }
                return None;
            } else {
                return Some(event);
            }
        }
        KeyMode::Released => {
            if is_mod() {
                //is mod state calculate all the keys
                let k = &mut sk.keys.clone();
                k.sort();
                let shortcut_key = k.join("-");
                let func_key = all_keys.iter().find(|x| {
                    let s = sort_key(x.key.as_str());
                    s == shortcut_key
                });

                if let Some(func_key) = func_key {
                    let has_scope = !func_key.scope.is_empty();
                    let app = get_foreground_app();
                    if has_scope {
                        if let Some(app) = app {
                            if func_key.scope.contains(&app) {
                                // in scope, execute key function
                                // execute key function
                                // set mod -> update_mod(true)
                                KeymapEvent::send(Func(FunctionEvent {
                                    function: func_key.function.to_string(),
                                    macros: Some(func_key.macros.clone()),
                                }));
                                update_mod(true);
                            } else {
                                //set mod -> update_mod(false)
                                update_mod(false);
                            }
                        }
                    } else {
                        //no scope given
                        //execute the key function
                        //set mod -> update_mod(true)
                        KeymapEvent::send(Func(FunctionEvent {
                            function: func_key.function.to_string(),
                            macros: Some(func_key.macros.clone()),
                        }));
                        update_mod(true);
                    }
                    //clear keys
                    sk.keys.clear();
                } else {
                    //set mod -> update_mod(false)
                    sk.keys.clear();
                    update_mod(false);
                }
                return None;
            } else {
                // mod is off..
                // clear all recorded keys and send the actual key event back
                sk.keys.clear();
                return Some(event);
            }
        }
    }
}

pub struct RemapMe {
    pub key_setting: KeySetting,
}
pub struct KV<'a> {
    pub k: &'a str,
    pub v: &'a str,
}
impl RemapMe {
    pub fn new() -> Self {
        Self {
            key_setting: KeySetting::new(),
        }
    }
    pub fn set_mod_key(&self, key: Key) {
        *DT_KEY.lock() = key;
    }
    pub fn set_hold_key(&self, key: Key) {
        *HOLD_KEY.lock() = key;
    }
    pub fn to_display(&self) -> Vec<KV> {
        self.key_setting
            .keys
            .iter()
            .map(|x| KV {
                k: x.key.as_str(),
                v: x.function.as_str(),
            })
            .collect::<Vec<KV>>()
    }

    pub fn spawn(&self) {
        *ALL_KEYS.lock() = self.key_setting.keys.clone();
        thread::spawn(|| {
            if let Err(err) = grab(move |event: Event| -> Option<Event> {
                let mut dt = DT.lock();
                let mut hold = HOLD.lock();
                let dt_key = *DT_KEY.lock();
                let hold_key = *HOLD_KEY.lock();
                match &event.event_type {
                    EventType::KeyPress(key) => match key {
                        a if a == &hold_key => {
                            if hold.is_pressed() {
                                None
                                //Some(event)
                            } else {
                                if hold.is_locked() {
                                    Some(event)
                                } else {
                                    hold.press();
                                    update_mod(true);
                                    None
                                }
                            }
                        }
                        k if k == &dt_key => {
                            if dt.locked {
                                Some(event)
                            } else {
                                dt.init();
                                None
                            }
                        }
                        _ => {
                            let is_mod = *IS_MOD.lock();
                            if is_mod {
                                update_keys(KeyWrapper(*key), KeyMode::Pressed, event)
                            } else {
                                Some(event)
                            }
                        }
                    },
                    EventType::KeyRelease(key) => {
                        let kw = KeyWrapper(*key);
                        let is_mod = *IS_MOD.lock();

                        if key == &hold_key {
                            if hold.is_pressed() && hold.is_locked() {
                                if hold.is_locked() {
                                    update_mod(false);
                                    match hold.release() {
                                        true => {
                                            return None;
                                        }
                                        false => {
                                            hold.should_lock(true);
                                            println!("send original");
                                            send_original_hold_key();
                                            return None;
                                        }
                                    }
                                }
                            } else {
                                println!("release {key:?}");
                                return Some(event);
                            }
                            return None;
                        }

                        if key == &dt_key {
                            if dt.locked {
                                return Some(event);
                            } else {
                                match dt.is() {
                                    TapMode::QuickTap | TapMode::Hold => {
                                        dt.locked = true;
                                        send_original_doubletap_key();
                                        return None;
                                    }
                                    TapMode::DoubleTap => {
                                        update_mod(!is_mod);
                                    }
                                }
                            }
                        } else {
                            return update_keys(kw, KeyMode::Released, event);
                        }
                        Some(event)
                    }
                    _ => Some(event),
                }
            }) {
                println!("Error grabbing::{err:?}");
            }
        });
    }
}
