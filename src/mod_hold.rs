use std::time::{Duration, Instant};


const THRESHOLD:Duration  = Duration::from_millis(200);
pub struct Hold{
    pressed:bool,
    locked:bool,
    press_time: Instant,
}
impl Hold{
    pub fn new()->Self{
        Self{
            pressed:false,
            locked:false,
            press_time: Instant::now(),
        }
    }
    pub fn should_lock(&mut self,lock:bool){
        self.locked = lock;
    }
    pub fn is_locked(&self)->bool{        
        self.locked

    }
    pub fn is_pressed(&self)->bool{
        self.pressed
    }
    pub fn press(&mut self){
        if !self.pressed{
            self.pressed = true;
            self.locked = true;
            self.press_time = Instant::now();
        }

      
    }
    pub fn release(&mut self)->bool{
        self.pressed = false;
        self.locked = false;
        let elapsed = self.press_time.elapsed();
  
        if elapsed < THRESHOLD{
            return false
        }else{
            true
        }
        
     

    }

  
}