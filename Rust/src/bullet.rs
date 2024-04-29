use godot::obj::NewAlloc;
use godot:: prelude::*;
// this is the bullet behaviour node 2d for position
use godot::engine::{Area2D, Node2D};// inti stuff
#[derive(GodotClass)]
#[class(base=Node2D)]
struct BulletBehaviour{
    #[export]
    actual_base:Gd<crate::mindless_mover::MindlessMover>,// actual base that would be manupulated
    base:Base<Node2D>,
}
#[godot_api]
impl INode2D for BulletBehaviour{
    // init stuff
    fn init(base: Base<Node2D>) -> Self {
        Self { actual_base: NewAlloc::new_alloc(), base }
    }
    // set the base type to bullet
    fn ready(&mut self){
        self.actual_base.bind_mut().set_type(crate::actortype::Types::Bullet);
    }

}
#[godot_api]
impl BulletBehaviour {
    #[func]
    // set the mindless mover behaviour
    // chase bullet but ignore everything
    fn set_behaviour(type_of_area:GString)->i8{
        let typ=crate::actortype::Types::godot_gstring_deserilize(type_of_area);
        match typ {
            crate::actortype::Types::Mob=>{
                1
            },
            _=>{
                0
            }
        }
    }
    #[func]
    fn on_mob_collide(&self,mut area:Gd<Area2D>){
        // if reach an mob  then check if it is a mob. if so then init a kill command on mob
        match area.try_call("get_type".into(), &[]) {
            Ok(res)=>{
                match  res.try_to::<GString>(){
                    Ok(str)=>{

                        match crate::actortype::Types::godot_gstring_deserilize(str) {
                            crate::actortype::Types::Mob=>{
                                area.call("kill_command".into(), &[]);
                            }
                            _=>{

                            }
                        }
                    }
                    Err(_)=>{

                    }
                }
            },
            Err(_)=>{

            }
        }
    }
}