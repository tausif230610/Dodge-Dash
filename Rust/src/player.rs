use godot::obj::WithBaseField;
use godot::prelude::*;// init stuff
use godot::engine::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, Timer};// the player has an animated sprite 2d for the sprites. Area 2d as the base class and Node 2d for well stuff for Kill mob upon Dash
use crate::DFPS;//To stick with pixel/second type calculations
use crate::{specialbutton::SpecialButton,controler::Controler};// to interface with the controler
// this 2 function exist because i can make this fuctions. Or the already given min max function was a headace
fn min<T:std::cmp::PartialOrd>(v1:T,v2:T)->T{
if v1>v2{
v2
}
else {
    v1
}
}
fn max<T:std::cmp::PartialOrd>(v1:T,v2:T)->T{
if v1>v2{
    v1
    }
else {
    v2
    } 
}/* 
//!WARNING! The charge level is devided into 3 segments
//!WARNING! The charge level is devided into 3 segments
//!WARNING! The charge level is devided into 3 segments
//!WARNING! The charge level is devided into 3 segments
*/
//a reminder for me that i basically devided the charge level into 3 parts. 3 is always the standerd. 
#[derive(GodotClass)]//basic init stuff
#[class(init,base=Area2D)]
struct Player{
    #[export]
    #[init(default=5.0)]
    speed:real,//speed in any direction in pixel/seconds(due to DFPS)
    #[export]
    #[init(default=20.0)]
    dashlength:real,// if i ever dash i would like to travle a specific distance. this keeps the dash constant(more like sonic 06 air dash).
    #[init(default=true)]// so that i dont accidentally dash or shoot uppon death
    is_alive:bool,
    #[init(default=3.0)]//seconds
    max_charge_time:real,// in mts seconds the charge would be complete
    #[init(default=1)]
    rem_dash:u8,// counter for remaning dashs. wont surpass 3.
    #[init(default=1)]
    rem_shoot:u8,//counter for remaning shoots. also wont surpass 3. 
    // this is a interesting calculation
    //0,1,2,3->(00)->rem_dash, 0,1,2,3-:(00) -> rem_shoot. so through some bit hacks 
    //ds sh
    //__ __
    //00 00->u4? half of u8 rem_dash_and_shoot? now how to do bit hacks(def val = 0101=> 1+8=>9)
    // or use bit_struct. u2
    viewportsize:Vector2,// the viewport size so that the player do not go out of bounds
    #[init(default=0.1)]// amount of time by when the player finish its tween animation. needs to be fine adjusted
    duration:f32,
    #[init(default=12)]// cause why not?
    _bullet_speed:u8,
    base:Base<Area2D>
}
#[godot_api]
impl IArea2D for Player {
    // get the viewport size and set it to itself
    // then hide itself
    fn ready(&mut self){
        let sizeofviewport=self.base_mut().get_viewport_rect().size;
        self.viewportsize=sizeofviewport;
        self.base_mut().hide();
    }
    fn process(&mut self,delta:f64){
        
        //first get the mutable reference to the Animated sprite 2d
        let sprite=&mut self.base_mut().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        // then get the mutable reference to the controler. then uppon getting it call the get direction method to get a vector 2.
        let direcion= self.base_mut().get_node_as::<Controler>("Controler").bind_mut().get_direction();
        // Up walk sprite selection basically
        if direcion.x !=0.0{
            sprite.set_animation("Walk".into());
            sprite.set_flip_h(direcion.x<0.0);
            sprite.set_flip_v(false);

        }
        else if direcion.y !=0.0 {
            sprite.set_animation("Up".into());
            sprite.set_flip_v(direcion.y>0.0);
        }
        // why would the player try to move when its standing
        if direcion.length() !=0.0{
            sprite.play();
        }
        else {
            sprite.stop();
        }
        /*
        hacky way to get the velocity.
        first take the speed and multiply it by the direction. then take the delta for smooth animation
        and finally multiply the whole thing by DFPS or default_frame_rate as a Reference. Aka in a 
        60 fps display the player would move a set amount of pixel per second
        so [vel]=pixel*frate/second==(pixel/second) *(frame !time)/(second !time) aka fps is a conversion constant 
        */
        let velocity=self.speed*direcion*(delta as f32)*DFPS;
        // take the current position
        let mut curpos=self.base_mut().get_position();
        //position+=velocity or should i say velocity*time since it also has time stuff built into it
        curpos+=velocity;
        // make it follow the borders
        while curpos.y>self.viewportsize.y {
            curpos.y-=self.viewportsize.y;
        }
        while curpos.y<0.0 {
            curpos.y+=self.viewportsize.y;
        }
        while curpos.x>self.viewportsize.x {
            curpos.x-=self.viewportsize.x;
        }
        while curpos.x<0.0 {
            curpos.x+=self.viewportsize.x;
        }
        self.base_mut().set_position(curpos);

    }
    
}
#[godot_api]
impl Player {
    #[func]
    fn reinit(&mut self){
        self.rem_dash=1;
        self.rem_shoot=1;
        self.is_alive=true;

    }
    #[signal]
    fn hit(body:Gd<Node2D>);// request the main scene to process what would happen after the initial hit
    #[signal]
    fn request_time_slow_or_mob_speed_reduction_after_dash(speed_devisor:f32);// request the main scene to slow down the mobs along side with dashing
    #[signal]
    fn request_invincibility();// request the main scene to gain invincivility aka hit would always kill the enemy unfortunately + score boost
    #[signal]
    fn request_order_66();// request the main scene for executing order 66 on the Mobs. No score bonus but in an instant all enemy would die
    #[signal]
    fn dashing();// let the main scene know that the player is dashing
    #[signal]
    fn stoped_dashing();// let the main scene know that the player has stoped dashing
    #[func]
    //ask the main scene if it was a genune hit or kill.
    fn _on_enemy_area_enter(&mut self,body:Gd<Area2D>){
        self.base_mut().emit_signal("hit".into(), &[Variant::from(body)]);
    }
    #[func]
    fn on_controler_queue_input(&mut self,butn:SpecialButton,heldtime:real){
        // cause if you are dead why bother firing?
        if self.is_alive{
        // since all the animation in terms of player only is just dash and shoot aka forward and reverse movement aside from other stuffs
        // a tween node is needed for procedural animations
        let mut aniamtor_tween=self.base_mut().get_tree().unwrap().create_tween().unwrap();
        // get the direction from the controler node.
        let direction= self.base_mut().get_node_as::<Controler>("Controler").bind_mut().get_direction();
        // get the current position
        let mut current_position=self.base().get_position();
        // get the minimum posible charge unit
        
        let min_charge_val=self.max_charge_time/3.0;
        // exercise for the reader if some one is actually reading this code then thanks
        let charge_level=(min::<f32>(self.max_charge_time, max::<f32>(1.0, heldtime))/min_charge_val) as u8;
        let dscharge=min::<u8>(charge_level, self.rem_dash);
        let shcharge=min::<u8>(charge_level, self.rem_shoot);
        
        // matching the buttons
        match butn {
            SpecialButton::Dash=>{
            match dscharge {
                0=>{

                },

                1=>
                {   // just a basic dash
                    godot_print!("basic dash commited");
                    self.rem_dash-=1;
                },
                2=>{
                    // dash + time stop
                    godot_print!("basic dash + time slow not invincible");
                    self.base_mut().emit_signal("request_time_slow_or_mob_speed_reduction_after_dash".into(), &[Variant::from(1.2 as f32)]);
                    self.rem_dash-=2;
                },
                3=>{
                    // dash + matrix mode
                    godot_print!("basic dash + time stop + invincible");
                    self.base_mut().emit_signal("request_time_slow_or_mob_speed_reduction_after_dash".into(), &[Variant::from(100.2 as f32)]);
                    self.base_mut().emit_signal("request_invincibility".into(), &[]);
                    self.rem_dash-=3;
                },
                _=>{
                    // it wont happen hahaha
                    panic!("Impossible it should be from 1 to 3 not {}",dscharge);
                }
                
            }
            // the dash is set to be a fixed direction so its justified
            current_position+=direction*self.dashlength*(dscharge as f32);
            
            },
            SpecialButton::Shoot=>{
            match shcharge {
                0=>{},
                1=>{
                    //bullet has not been implemented
                    godot_print!("One bullet has been shoot");
                    self.rem_shoot-=1;
                },
                2=>{
                    // same as 1
                    godot_print!("3 big bullet has been shoot");
                    self.rem_shoot-=2;
                },
                3=>{
                    // Order 66 should be handled by main scene
                    self.base_mut().emit_signal("request_order_66".into(), &[]);
                    godot_print!("Order 66 for the mobs");
                    self.rem_shoot-=3;
                },
                _=>{
                    //same as the dash case
                    panic!("Imposible it should be from 1 to 3 not {}",shcharge);
                }
            
            }
            // upon shooting the player should feel a back thrust
            current_position-=direction*self.dashlength*(shcharge as f32)/4.0;
            },
            SpecialButton::None=>{
                // same as the panic cases
                panic!("Imposible this should be ether Dash or Shoot Not {:?}" ,butn);
            }
        }
        // for more control
        let dur:f64=self.duration as f64;
        // the player is dashing so the main scene should know about it
        self.base_mut().emit_signal("dashing".into(), &[]);
        // tween animating the player casually
        aniamtor_tween.tween_property(self.base_mut().clone().upcast(), "position".into(), Variant::from(current_position), dur);
        aniamtor_tween.connect("finished".into(), self.base().callable("on_tween_anim_stop"));
        
    }
    }
    #[func]
    fn on_tween_anim_stop(&mut self){
        // just tell the main scene that we have stoped dashing
        self.base_mut().emit_signal("stoped_dashing".into(), &[]); 
    }
    #[func]
    fn on_dash_recharge(&mut self){
        // get the timer
        let tmr=&mut self.base_mut().get_node_as::<Timer>("Recharge_Dash");
        // recharge till rem dash=3
        if self.rem_dash<3{
        self.rem_dash+=1;
        }
        // 
        tmr.set_wait_time(min::<f64>(3.0,max::<f64>(self.rem_dash as f64,1.0)));
        
    }
    #[func]
    fn on_genuen_death(&mut self){
        // hide self
        self.base_mut().hide();
        // get the collision shape and disable it
        let cs2d=&mut self.base_mut().get_node_as::<CollisionShape2D>("CollisionShape2D");
        cs2d.set_deferred("disabled".into(), Variant::from(true));
        self.is_alive=false;
    }
    #[func]
    fn start(&mut self,pos:Vector2){
        
        // show the player
        self.base_mut().show();
        // take the collision shape 2d and make it responsive
        let cs2d=&mut self.base_mut().get_node_as::<CollisionShape2D>("CollisionShape2D");
        cs2d.set_disabled(false);
        // set player position
        self.base_mut().set_position(pos);
        self.reinit();
    }

}
