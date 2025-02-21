mod double_tap;
mod winutil;
pub mod event;
mod setting;
mod keywrapper;
use double_tap::DoubleTap;
use rdev::SimulateError;
pub use setting::{KeySetting, Macro, MacroKey};
pub use keywrapper::{string_to_key, KeyWrapper};

use std::{thread::{self, sleep}, time::Duration};
use parking_lot::Mutex;
use once_cell::sync::Lazy;
pub use rdev::{
    grab, simulate, Event, EventType, Key,Key::{F13,F14,F15,F16,F17,F18,F19,F20,F21,F22,F23,F24}
};


static DT: Lazy<Mutex<DoubleTap>> = Lazy::new(|| Mutex::new(DoubleTap::new(200)));
static IS_MOD: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static ALL_KEYS: Lazy<Mutex<Vec<MacroKey>>> = Lazy::new(|| Mutex::new(Vec::new()));
static MOD_KEY: Lazy<Mutex<Key>> = Lazy::new(|| Mutex::new(Key::Escape));
static KEYS: Lazy<Mutex<KeyState>> = Lazy::new(|| Mutex::new(KeyState::default()));

pub use event::{
    KeymapEvent::{self, Func, Mod},
    FunctionEvent, KeyPressEvent, ModChangeEvent,
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
fn _simulate(ev:&EventType){
    match simulate(ev){
        Ok(())=>{}
        Err(SimulateError)=>{ eprintln!("Error simulating KeyPress {ev:?}")}
    }
}
fn kc_press(key:Key) {
  
     _simulate(&EventType::KeyPress(key));
    sleep(Duration::from_millis(10));
    
}
fn kc_release(key:Key) {
  
    _simulate(&EventType::KeyRelease(key));
    sleep(Duration::from_millis(10));
    
}
fn kc_click(key:Key) {
  
     _simulate(&EventType::KeyPress(key));
    sleep(Duration::from_millis(10));
     _simulate(&EventType::KeyRelease(key));
    
}

pub fn update_mod(is_mod: bool) {
    *IS_MOD.lock() = is_mod;
    KeymapEvent::send(Mod(ModChangeEvent { is_mod }));
}

pub fn kc_macro(macros:Option<Macro>){
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
        kc_mod( mods, keys);
    }
}

pub fn kc( keys: Vec<Key>) {
    update_mod(false);

    for &key in keys.iter() {
        kc_click(key);
    }

    update_mod(true);
}
pub fn delay(millis:u64){
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

fn escape() {
    thread::spawn(|| {
        let mod_key = *MOD_KEY.lock();
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


pub struct RemapMe{
    pub mod_key:Lazy<Mutex<Key>>,
    pub key_setting:KeySetting,

}
pub struct KV<'a>{
    pub k:&'a str,
    pub v:&'a str
}
impl RemapMe{
    pub fn new()->Self{
        Self { 
            mod_key:Lazy::new(||Mutex::new(Key::Escape)),
            key_setting:KeySetting::new()
         }
    }
    pub fn set_mod_key(&self, key:Key){
        *MOD_KEY.lock() = key;
    }
    pub fn to_display(&self)->Vec<KV>{
        self.key_setting.keys.iter()
        .map(|x|KV{k:x.key.as_str(),v:x.function.as_str()})
        .collect::<Vec<KV>>()
        
    }



    pub fn spawn(&self){

        *ALL_KEYS.lock() = self.key_setting.keys.clone();      
        thread::spawn(||{
            _ = grab(move |event: Event| -> Option<Event> {
                let mut dt = DT.lock();
                let mod_key = *MOD_KEY.lock();
                match &event.event_type {
                    EventType::KeyPress(key) => match key {
                        k if k == &mod_key => {
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
    
                        if key == &mod_key {
                            if dt.locked {
                                return Some(event);
                            } else {
                                if dt.is() {
                                    update_mod(!is_mod);
                                } else {
                                    dt.locked = true;
                                    escape();
                                    return None;
                                }
                            }
                        } else {
                            return update_keys(kw, KeyMode::Released, event);
                        }
                        Some(event)
                    }
                    _ => Some(event),
                }
            });
        });
      
    }
}