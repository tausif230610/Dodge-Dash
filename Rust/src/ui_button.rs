use godot::prelude::*;
use godot::engine::{TouchScreenButton,ITouchScreenButton,TextureProgressBar};
#[derive(GodotClass)]
#[class(base=TouchScreenButton)]
struct SUIButton{
#[export]
joint_progress_bar1:Option<Gd<TextureProgressBar>>,
#[export]
joint_progress_bar2:Option<Gd<TextureProgressBar>>,
#[export]
joint_progress_bar3:Option<Gd<TextureProgressBar>>,
#[export]
behaviour:Option<Gd<Node>>,
base:Base<TouchScreenButton>
}
#[godot_api]
impl ITouchScreenButton for SUIButton {
    fn init(base: Base<TouchScreenButton>) -> Self {
        SUIButton {
            joint_progress_bar1: None,
            joint_progress_bar2:None,
            joint_progress_bar3:None,
            behaviour: None,
            base }
    }
    fn ready(&mut self){
        let press_call=self.base().callable("on_pressed");
        let release_call=self.base().callable("on_released");
        self.base_mut().connect("pressed".into(), press_call);
        self.base_mut().connect("released".into(), release_call);
    }
    
    
}
#[godot_api]
impl SUIButton {
    #[func]
    fn on_pressed(&mut self){
        if let Some(true_behaviour) = self.behaviour.as_mut(){
            true_behaviour.call("press".into(), &[]);
        }
    }
    #[func]
    fn on_released(&mut self){
        if let Some(true_behaviour) = self.behaviour.as_mut(){
            true_behaviour.call("release".into(), &[]);
        }
    }
}