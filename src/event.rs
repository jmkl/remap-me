use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::{Lazy, OnceCell};

use crate::setting::Macro;

type FunctionEventReceiver = Receiver<KeymapEvent>;
type FunctionEventHandler = Box<dyn Fn(KeymapEvent) + Send + Sync + 'static>;
static G_FUNCTION_CHANNEL: Lazy<(Sender<KeymapEvent>, FunctionEventReceiver)> =
    Lazy::new(unbounded);
static G_FUNCTION_EVENTHANDLER: OnceCell<Option<FunctionEventHandler>> = OnceCell::new();

#[derive(Debug)]
pub struct ModChangeEvent {
    pub is_mod: bool,
}
#[derive(Debug)]
pub struct FunctionEvent {
    pub function: String,
    pub macros:Option<Macro>
}
#[derive(Debug)]
pub struct KeyPressEvent {
    pub keys: Vec<String>,
}

#[derive(Debug)]
pub enum KeymapEvent {
    Func(FunctionEvent),
    Mod(ModChangeEvent),
    Key(KeyPressEvent)
}

impl KeymapEvent {
    pub fn receiver<'a>() -> &'a FunctionEventReceiver {
        &G_FUNCTION_CHANNEL.1
    }
    pub fn send(event: KeymapEvent) {
        if let Some(handler) = G_FUNCTION_EVENTHANDLER.get_or_init(|| None) {
            handler(event);
        } else {
            let _ = G_FUNCTION_CHANNEL.0.send(event);
        }
    }
}
