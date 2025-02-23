use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};
#[derive(Default,Debug, Clone, Deserialize, Serialize)]
pub struct Macro{
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier:Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys:Vec<String>
}

impl Macro {
    fn is_empty(&self) -> bool {
        self.modifier.is_empty() && self.keys.is_empty()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MacroKey {
    pub key: String,
    #[serde(default,skip_serializing_if = "Vec::is_empty")]
    pub scope: Vec<String>,
    pub function: String,
    #[serde(default,skip_serializing_if = "Macro::is_empty")]
    pub macros:Macro
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeySetting {
    setting_path:String,
    pub keys: Vec<MacroKey>,
    
}

pub fn get_setting_path()->String{
  let current_dir = match std::env::current_exe() {
    Ok(path) => path.parent().unwrap().to_string_lossy().to_string(),
    Err(_) => "".to_string(),
};
format!("{}\\key_setting.json", current_dir)

}


impl KeySetting {
    fn default_keys()->Vec<MacroKey>{

        let default_setting = json!([
            {
                "key": "f12",
                "function": "toggle_click"
            },
            {
                "function": "zen_search",
                "key": "C-g"
              },
              {
                "function": "ps_newdoc",
                "key": "n"
              },
              {
                "function": "ps_save",
                "key": "C-s"
              },
              {
                "function": "ps_enable_shadow",
                "key": "e"
              },
              {
                "function": "ps_select_text",
                "key": "t"
              },
              {
                "function": "ps_select_subject",
                "key": "a"
              },
              {
                "function": "ps_join_text",
                "key": "j"
              },
              {
                "function": "ps_gaussian_blur",
                "key": "b"
              },
              {
                "function": "ps_split_text",
                "key": "/"
              },
              {
                "function": "ps_curve",
                "key": "c"
              },
              {
                "function": "ps_exposure",
                "key": "x"
              },
              {
                "function": "ps_color_balance",
                "key": "k"
              },
              {
                "function": "ps_hue_saturation",
                "key": "h"
              },
              {
                "function": "ps_fix_mask",
                "key": "f"
              },
              {
                "function": "ps_align_left",
                "key": "left"
              },
              {
                "function": "ps_align_center",
                "key": "up"
              },
              {
                "function": "ps_align_right",
                "key": "right"
              },
              {
                "function": "ps_justify_center",
                "key": "down"
              },
              {
                "function": "ps_margin",
                "key": "m"
              },
              {
                "function": "ps_move_photo",
                "key": "pageup"
              },
              {
                "function": "ps_move_bg",
                "key": "pagedown"
              },
              {
                "function": "ps_drop_shadow",
                "key": "d"
              },
              {
                "function": "ps_redbox",
                "key": "r"
              },
              {
                "function": "ps_scale",
                "key": "s"
              },
              {
                "function": "ps_distribute",
                "key": "space"
              },
              {
                "function": "macros",
                "key": "f11",
                "macros":{
                    "modifier":["C"],
                    "keys":["a"]
                }
              }
        ]);
      let result = serde_json::from_value(default_setting).unwrap();
      result
   }

    fn write_default(file_path: &str) {
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_path)
        {
            Ok(mut f) => {
                let default_config =
                    serde_json::to_string_pretty(&KeySetting::default_keys()).expect("Error writing default config");
                _ = f.write_all(default_config.as_bytes());
            }
            Err(err) => {
               //file already exist dont do anything
            }
        }
    }
    pub fn save_setting(&self,keys:Vec<MacroKey>)->std::io::Result<()>{
        let content = serde_json::to_string_pretty(&keys).expect("Error writing default config");
        fs::write(&get_setting_path(), content)?;
        Ok(())
    }
    pub fn update_setting(&mut self){
        let content = match fs::read_to_string(&get_setting_path()) {
            Ok(result) => result,
            Err(_) => panic!("Error reading file"),
        };
        let setting: KeySetting = serde_json::from_str(&content).unwrap();
        self.keys = setting.keys;

    }
    pub fn new() -> Self {
        let setting_path = get_setting_path();
        Self::write_default(&setting_path);
        let content = match fs::read_to_string(&setting_path) {
            Ok(result) => result,
            Err(_) => panic!("Error reading file"),
        };
        let setting: Vec<MacroKey> = serde_json::from_str(&content).unwrap();
       
 
       
       
     
        
        Self {
            keys: setting,
            setting_path
        }
    }
}
