//actually i wanted it to be its own class but sicne rust class cant have child so this is a specialized
//work around.



use godot::engine::utilities::{randi_range, randomize};// rng stuff
use godot::obj::NewAlloc;// for alocation
use godot:: prelude::*;//init stuff
use godot::engine::{AnimatedSprite2D,Node2D};

use crate::mindless_mover::MindlessMover; // i dont need any extra interface
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MobBehaviour{
    #[export]
    max_speed:u8,
    #[export]
    min_speed:u8,
    #[export]
    actual_base:Gd<MindlessMover>,
    base:Base<Node2D>
}

#[godot_api]
impl INode2D for MobBehaviour {
fn init(base:Base<Node2D>)->Self{
Self {
    max_speed: 10,
    min_speed: 5,
    actual_base:NewAlloc::new_alloc(),
    
    base }
}
fn ready(&mut self){

let animator=&mut self.base_mut().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
let base_mover=&mut self.actual_base;
base_mover.bind_mut().set_type(crate::actortype::Types::Mob);
randomize();

let speed_of_mob=randi_range(self.min_speed as i64, self.max_speed as i64) as u8;
base_mover.bind_mut().set_speed(speed_of_mob);
let mob_types=animator.get_sprite_frames().unwrap().get_animation_names();
animator.set_animation(mob_types.get(randi_range(0, (mob_types.len() as i64) -1) as usize).into());
animator.play();
}
}
#[godot_api]
impl MobBehaviour {
    #[func]
    pub fn set_behaviour(type_of_area:GString)->i8{
        let typ=crate::actortype::Types::godot_gstring_deserilize(type_of_area);
        match typ {
            crate::actortype::Types::Player(dsh)=>{
                match dsh {
                    true=>{
                        -1
                    },
                    false=>{
                        1
                    }
                }
            },
            crate::actortype::Types::Bullet=>{
                -1
            },
            _=>0,
        }
    }
}