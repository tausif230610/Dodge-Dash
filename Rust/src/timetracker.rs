use std::time::SystemTime;//required followed by default stuff
use godot::prelude::*;
use godot::engine:: Node;//timetracker is a base Node child
#[derive(GodotClass)]
#[class(init,base=Node)]//default init is ok
pub struct TimeTracker{
    #[init(default=Option::None)]
    systime:Option<SystemTime>,//so that any Null stuff is already dealt
    base:Base<Node>//node base
}
#[godot_api]
impl TimeTracker {
    #[func]
    //starts the count down
    pub fn start_count(&mut self){
        self.systime=Some(SystemTime::now());
    }
    #[func]
    //give the elapsed time data but wont reset as intended
    pub fn preview_elapsed_time_as_real_sec(&mut self)->real{
        match self.systime {
            Some(stime)=>{
                match stime.elapsed() {
                    Ok(el)=>el.as_secs_f32(),
                    Err(_er)=>0.0
                }
            },
            None=>{
                0.0
            }
        }
    }
    #[func]
    //gives time data and completely reset and wait for start count
    pub fn finalize_elapsed_time_as_real_sec(&mut self)->real{
        match self.systime {
            Some(stime)=>{
                match stime.elapsed() {
                    Ok(el)=>{
                        self.systime=None;
                        el.as_secs_f32()
                    },
                    Err(_er)=>0.0
                }
            },
            None=>{
                0.0
            }
        }
    }
}