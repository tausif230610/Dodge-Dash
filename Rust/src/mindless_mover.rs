use std::f32::consts::TAU;// get the tau constant its important
use godot:: prelude::*;//init stuff
use godot::engine::{Area2D, IArea2D};//mindless mover is a child type of area 2d

use crate::DFPS;// need the fps
#[derive(GodotClass)]
#[class(init,base=Area2D)]

pub struct MindlessMover{
    angle:u8,
    speed:u8,
    #[init(default=None)]
    #[export]
    behaviour:Option<Gd<Object>>,
    base:Base<Area2D>
}
#[godot_api]
impl IArea2D for MindlessMover {
    fn process(&mut self,delta:f64){
        let velocity=Vector2::new(self.speed as f32, 0.0).rotated(self.angle as f32 * TAU /256.0);
        let mut position=self.base_mut().get_position();
        position+=velocity*delta as f32 *DFPS;
        self.base_mut().set_position(position);
    }
}
#[godot_api]
impl MindlessMover {
    #[func]
    fn on_viewport_exit(&mut self){
        self.base_mut().queue_free();
    }
}