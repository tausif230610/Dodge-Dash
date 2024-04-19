//actually i wanted it to be its own class but sicne rust class cant have child so this is a specialized
//work around.

use godot::prelude::*;//init stuff
use godot::engine::Node;// i dont need any extra interface

#[derive(GodotClass)]
#[class(no_init,base=Node)]
pub struct MobBehaviour{
    #[export]
    max_speed:u8,
    #[export]
    min_speed:u8,
    //#[init(default=)]
    //pub signeture:u32,
    base:Base<Node>
}

#[godot_api]
impl MobBehaviour {
    #[func]
    fn reinit(){

    }
}