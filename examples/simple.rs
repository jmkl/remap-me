use std::{thread::{self, sleep}, time::Duration};
use std::io::{stdout, Write};
use crossterm::{cursor::{self, MoveTo}, execute, terminal::{Clear, ClearType}};

use rdev::Key::{self,F13,F14,F15,F16,F17,F18,F19,F20,F21,F22,F23,F24};
use remap_me::{event::KeymapEvent, get_foreground_app, kc, kc_macro, kc_mod, string_to_key, update_mod, RemapMe};



fn update_ui(content:&str){
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0), Clear(ClearType::CurrentLine)).unwrap();
    execute!(stdout,cursor::Hide);
    print!("{content}");
    stdout.flush().unwrap();

}

fn zen_browser_search(){
    let is_zen = match get_foreground_app() {
        Some(app) => app == "zen.exe",
        None => false,
    };
    if !is_zen {
        kc_mod(vec![Key::MetaLeft], vec![Key::Num2]);
    }
    sleep(Duration::from_millis(100));
    kc_mod(vec![Key::ControlLeft], vec![Key::KeyL]);
    sleep(Duration::from_millis(100));
    kc_mod(vec![Key::ShiftLeft], vec![Key::Num2]);
    kc(vec![Key::KeyI,Key::Tab]);
 
}


fn spawn_listener() {
    thread::spawn(|| {
        let receiver = KeymapEvent::receiver();
      
        loop {
            if let Ok(ev) = receiver.try_recv() {
              

                match ev {
                    
                    KeymapEvent::Func(ev) => match ev.function.as_str() {
                        
                        //PHOTOSHOP
                        "ps_newdoc" => {
                            kc(vec![ F24, F21, F14]);
                        }
                        "ps_save" => {
                            kc(vec![ F24, F21, F13]);
                        }
                        "ps_enable_shadow" => {
                            kc(vec![ F24, F21, F20]);
                        }
                        "ps_select_text" => {
                            kc(vec![ F24, F20, F20]);
                        }
                        "ps_select_subject" => {
                            kc(vec![ F24, F23, F21]);
                        }
                        "ps_join_text" => {
                            kc(vec![ F24, F23, F16]);
                        }
                        "ps_gaussian_blur" => {
                            kc(vec![ F24, F23, F15]);
                        }
                        "ps_split_text" => {
                            kc(vec![ F24, F23, F22]);
                        }
                        "ps_curve" => {
                            kc(vec![ F24, F21, F15]);
                        }
                        "ps_exposure" => {
                            kc(vec![ F24, F21, F19]);
                        }
                        "ps_color_balance" => {
                            kc(vec![ F24, F21, F18]);
                        }
                        "ps_hue_saturation" => {
                            kc(vec![ F24, F21, F17]);
                        }
                        "ps_fix_mask" => {
                            kc(vec![ F24, F23, F14]);
                        }
                        "ps_align_left" => {
                            kc(vec![ F17]);
                        }
                        "ps_align_center" => {
                            kc(vec![ F18]);
                        }
                        "ps_align_right" => {
                            kc(vec![ F19]);
                        }
                        "ps_justify_center" => {
                            kc(vec![ F22]);
                        }
                        "ps_margin" => {
                            kc(vec![ F20]);
                        }
                        "ps_move_photo" => {
                            kc(vec![ F24, F13, F13]);
                        }
                        "ps_move_bg" => {
                            kc(vec![ F24, F13, F14]);
                        }
                        "ps_drop_shadow" => {
                            kc(vec![ F24, F21, F16]);
                        }
                        "ps_redbox" => {
                            kc(vec![ F24, F20, F19]);
                        }
                        "ps_scale" => {
                            kc(vec![ F16]);
                        }
                        "ps_distribute" => {
                            kc(vec![ F24, F23, F19]);
                        }
                        "code_test" => {
                            kc_mod( vec![Key::ControlLeft], vec![Key::KeyB]);
                        }
                        "zen_search" => {
                            zen_browser_search();
                        }
                      
                        "macros" => {
                          kc_macro(ev.macros);
                        }
                        _ => {}
                    },
                    KeymapEvent::Mod(mc) => {
                        update_ui(match mc.is_mod{true=>"MOD [ON]",false=>"MOD [OFF]"});
                        // if let Some(app_handle) = APP_HANDLE.get() {
                        //     _ = app_handle.emit("mod-change", mc.is_mod);
                        // }
                    }
                    KeymapEvent::Key(keys) => {
                       

                        // if let Some(app_handle) = APP_HANDLE.get() {
                        //     _ = app_handle.emit("key-press", keys.keys);
                        // }
                    }
                }
       
            }
            sleep(Duration::from_millis(10));
        }
    });
}

fn main(){
    let remap_me = RemapMe::new();
    let help = remap_me.to_display();
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 1), Clear(ClearType::All)).unwrap();
    for x in help.iter(){
        println!("{:<8} : {}",x.k,x.v);
    }
    spawn_listener();    
    remap_me.spawn();

}