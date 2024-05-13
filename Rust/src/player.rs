
use std::f32::consts::PI;

use godot::obj::WithBaseField;
use godot:: prelude::*;// init stuff
use godot::engine::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, Timer};

// the player has an animated sprite 2d for the sprites. Area 2d as the base class and Node 2d for well stuff for Kill mob upon Dash
use crate:: DFPS;//To stick with pixel/second type calculations
use crate::{specialbutton::SpecialButton,controler::Controler};// to interface with the controler
// this 2 function exist because i can make this fuctions. Or the already given min max function was a headace
pub fn min<T:std::cmp::PartialOrd>(v1:T,v2:T)->T{
if v1>v2{
v2
}
else {
    v1
}
}
pub fn max<T:std::cmp::PartialOrd>(v1:T,v2:T)->T{
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
pub struct Player{
    #[export]
    #[init(default=5.0)]
    speed:real,//speed in any direction in pixel/seconds(due to DFPS)
    // the scene of bullet. something the player use to shoot bullets
    #[export]
    #[init(default=None)]
    bullet_scene:Option<Gd<PackedScene>>,
    #[export]
    #[init(default=None)]
    // the sound effect handler which play the hit sound upon the player getting hit.
    hit_sfx:Option<Gd<AudioStreamPlayer>>,
    // the death particle scene
    #[export]
    #[init(default=None)]
    death_particle_scene:Option<Gd<PackedScene>>,
    // the blast particle scene
    #[export]
    #[init(default=None)]
    balst_particle_scene:Option<Gd<PackedScene>>,
    #[init(default=crate::actortype::Types::Player(false))]
    type_of_actor:crate::actortype::Types,
    #[init(default=false)]
    #[export]
    // is invisible 
    is_invisible_due_to_rec_mode:bool,
    #[export]
    #[init(default=false)]
    // is invincible
    is_invincible:bool,
    #[export]
    #[init(default=20.0)]
    dashlength:real,// if i ever dash i would like to travle a specific distance. this keeps the dash constant(more like sonic 06 air dash).
    #[init(default=true)]// so that i dont accidentally dash or shoot uppon death
    is_alive:bool,
    #[init(default=3)]
    // max life and current life
    max_life:u8,
    current_life:u8,
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
    #[init(default=12.0)]// cause why not?
    #[export]
    _bullet_speed:f32,
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
        self.is_alive=false;
        
    }
    fn process(&mut self,delta:f64){
        
        //first get the mutable reference to the Animated sprite 2d
        let sprite=&mut self.base_mut().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        // then get the mutable reference to the controler. then uppon getting it call the get direction method to get a vector 2.
        let direcion= self.base_mut().get_node_as::<Controler>("Controler").bind_mut().get_direction();
    
        // Up walk sprite selection basically
        sprite.set_animation(match direcion.y !=0.0 {
            true=>"Up".into(),
            _=>"Walk".into()
        });
        sprite.set_flip_h(direcion.x<0.0);
        sprite.set_flip_v(direcion.y>0.0);
        //old code
        // if direcion.y !=0.0{
        //     sprite.set_animation("Walk".into());
        //     sprite.set_flip_h(direcion.x<0.0);
        //     sprite.set_flip_v(false);

        // }
        // else if direcion.x !=0.0 {
        //     sprite.set_animation("Up".into());
        //     sprite.set_flip_v(direcion.y>0.0);
        // }
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
    // on reset stuff has to be reinitilized
    fn reinit(&mut self){
        self.rem_dash=1;
        self.rem_shoot=1;
        self.current_life=self.max_life;
        self.is_alive=true;

        // let everyone know that dash and shoot and life has changed
        let variant=Variant::from(self.current_life);
        self.base_mut().emit_signal("life_change".into(), &[variant]);
        let varient=Variant::from(self.rem_dash);
        self.base_mut().emit_signal("dash_change".into(),&[varient]);
        let varient=Variant::from(self.rem_shoot);
        self.base_mut().emit_signal("shoot_change".into(),&[varient]);
    }
    #[func]
    // so that other potential collider know its a player
    pub fn get_type(&mut self)->GString{
        let mut p=self.type_of_actor;
        return p.godot_gstring_serilize();
    }
    // internal function to set the players type (is dashing)
    pub fn set_type(&mut self,typ:crate::actortype::Types){
        self.type_of_actor=typ;
    }
    
    #[signal]
    fn death();//// request the main scene to process what would happen after the initial hit
    // if the player has died
    #[signal]
    fn request_time_slow_or_mob_speed_reduction_after_dash(speed_devisor:u8);// request the main scene to slow down the mobs along side with dashing
    /*#[signal]
    fn request_invincibility();//// request the main scene to gain invincivility aka hit would always kill the enemy unfortunately + score boost
    */
    // player can do it on its own now
    #[signal]
    //let everyone know that dash or shoot number has changed
    fn dash_change(dsnum:u8);
    #[signal]
    fn shoot_change(shnum:u8);
    #[signal]
    fn life_change(lfnum:u8);
    #[signal]
    fn request_order_66();// request the main scene for executing order 66 on the Mobs. No score bonus but in an instant all enemy would die
    #[func]
    // make the player invincible and activate the timer
    pub fn activate_invincibility(&mut self){
        self.is_invincible=true;
        let timer=&mut self.base_mut().get_node_as::<Timer>("UnkillableTime");
        timer.start();
        self.base_mut().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D").get_material().unwrap().call("set_shader_parameter".into(), &[Variant::from("reffect"),Variant::from(true)]);
        godot_print!("Player is invanurable");
    }
    #[func]
    // after the player passed its phase then stop the player
    pub fn normalize_player(&mut self){
        godot_print!("player is normal");
        self.is_invincible=false;
        self.base_mut().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D").get_material().unwrap().call("set_shader_parameter".into(), &[Variant::from("reffect"),Variant::from(false)]);
    }
    #[func]
    ////ask the main scene if it was a genune hit or kill.
    //decide whether it was a kill or a miss. player can distroy there own bullets. it isnt a bug it is a feature
    fn _on_enemy_area_enter(&mut self,mut body:Gd<Area2D>){
        // if the player is dashing then grab the soul of mob(since bullet is made of mob soul) else get hit aka lose a life 
        if let crate::actortype::Types::Player(true)=self.type_of_actor{
            body.call("kill_command".into(),&[]);
            self.rem_shoot+=1;
            //let everyone know that shoot has changed
            let varient=Variant::from(self.rem_shoot);
            self.base_mut().emit_signal("shoot_change".into(),&[varient]);
            self.rem_shoot=min(self.rem_shoot, 3);
        }// no body wana lose life while blinking
        else if self.is_invincible {
            body.call("kill_command".into(),&[]);
            self.rem_shoot+=1;
            self.rem_shoot=min(self.rem_shoot,3);
        // let everyone know that dash has changed
        let varient=Variant::from(self.rem_dash);
        self.base_mut().emit_signal("dash_change".into(), &[varient]);
            // de case where de player is invincible 
        }
        else if !self.is_invisible_due_to_rec_mode{
            self.hit_sfx.as_mut().unwrap().play();
            // if current life is zero then game over
            self.current_life=self.current_life.wrapping_sub(1);
            // let everyone know that life has changed
            let variant=Variant::from(self.current_life);
            self.base_mut().emit_signal("life_change".into(), &[variant]);
            if self.current_life==0{
                // death 
                self.on_genuen_death();
            }
            else {
                //make the player unkillable for a while
                self.is_invisible_due_to_rec_mode=true;
                // set the timer
                let mut inv_timer=self.base_mut().get_node_as::<Timer>("Invincible_Time");
                //start it and connect its time out to the player class
                inv_timer.start();

                let sprite=&mut self.base_mut().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
                // make the sprite blink
                sprite.get_material().as_mut().unwrap().call("set_shader_parameter".into(), &[Variant::from("effect"),Variant::from(0.0)]);

            }
        }
    }
    #[func]
    fn on_inv_timer_timeout(&mut self){
        // get the sprite, stop it from blinking and turn its invincibility flag
        let sprite=&mut self.base_mut().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        sprite.get_material().as_mut().unwrap().call("set_shader_parameter".into(), &[Variant::from("effect"),Variant::from(1.0)]);
        self.is_invisible_due_to_rec_mode=false;
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
        // charge values
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

                    self.rem_dash-=1;
                },
                2=>{
                    // dash + time stop

                    self.base_mut().emit_signal("request_time_slow_or_mob_speed_reduction_after_dash".into(), &[Variant::from(2 as u8)]);
                    self.rem_dash-=2;
                },
                3=>{
                    // dash + matrix mode

                    self.base_mut().emit_signal("request_time_slow_or_mob_speed_reduction_after_dash".into(), &[Variant::from(4 as u8)]);
                    self.activate_invincibility();
                    self.rem_dash-=3;
                },
                _=>{
                    // it wont happen hahaha
                    panic!("Impossible it should be from 1 to 3 not {}",dscharge);
                }
                
            }
            // let everyone know dash changed
            let varient=Variant::from(self.rem_dash);
            self.base_mut().emit_signal("dash_change".into(), &[varient]);
            // the dash is set to be a fixed direction so its justified
            let mut blast_scene=self.balst_particle_scene.clone();
            // let mut parent=self.base_mut().get_parent().clone().as_mut().unwrap();
            self.base_mut().get_parent().as_mut().unwrap().add_child(
                if let Some(part_scene)=blast_scene.as_mut(){
                    let mut bpscene=part_scene.instantiate_as::<crate::explosion_particle::OneTimeParticle>();
                    bpscene.bind_mut().base_mut().set_position(current_position);
                    bpscene.bind_mut().base_mut().set_rotation(direction.angle()+ crate::PI);
                    bpscene.upcast()
                    // parent.add_child(bpscene.clone().upcast())
                    
                }
                else {
                    crate::explosion_particle::OneTimeParticle::new_alloc().upcast()
                }
            );
            current_position+=direction*self.dashlength*(dscharge as f32);
            
            },
            SpecialButton::Shoot=>{
            match shcharge {
                0=>{},
                1=>{
                    // take the bullet scene and instanciet it
                    let mut bullet_s=self.bullet_scene.as_mut().unwrap().instantiate_as::<crate::mindless_mover::MindlessMover>();
                    // set its angle to the direction
                    bullet_s.bind_mut().set_angle(direction.angle());
                    // set the speed to default bullet speed
                    bullet_s.bind_mut().set_speed(self._bullet_speed);
                    // set bullets position to players position
                    bullet_s.bind_mut().base_mut().set_position(self.base_mut().get_position());
                    //append it to the scene tree
                    self.base_mut().get_parent().unwrap().add_child(bullet_s.clone().upcast());
                    self.rem_shoot-=1;
                },
                2=>{
                    // same as 1
                    for i in 0..3 {
                        let mut bullet_s=self.bullet_scene.as_mut().unwrap().instantiate_as::<crate::mindless_mover::MindlessMover>();
                        bullet_s.bind_mut().set_angle(direction.angle()-(PI/2.0)+((PI*i as f32)/2.0));
                        bullet_s.bind_mut().set_speed(self._bullet_speed);
                        bullet_s.bind_mut().base_mut().set_position(self.base_mut().get_position());
                        self.base_mut().get_parent().unwrap().add_child(bullet_s.clone().upcast());
                    }
                    self.rem_shoot-=2;
                },
                3=>{
                    // Order 66 should be handled by main scene
                    self.base_mut().emit_signal("request_order_66".into(), &[]);
                    self.rem_shoot-=3;
                },
                _=>{
                    //same as the dash case
                    panic!("Imposible it should be from 1 to 3 not {}",shcharge);
                }
            
            }
            //let everyone know that shoot has changed
            let varient=Variant::from(self.rem_shoot);
            self.base_mut().emit_signal("shoot_change".into(), &[varient]);
            // upon shooting the player should feel a back thrust
            current_position-=direction*self.dashlength*(shcharge as f32)/4.0;
            let mut blast_scene=self.balst_particle_scene.clone();
            // let mut parent=self.base_mut().get_parent().clone().as_mut().unwrap();
            self.base_mut().get_parent().as_mut().unwrap().add_child(
                if let Some(part_scene)=blast_scene.as_mut(){
                    let mut bpscene=part_scene.instantiate_as::<crate::explosion_particle::OneTimeParticle>();
                    bpscene.bind_mut().base_mut().set_position(current_position);
                    bpscene.bind_mut().base_mut().set_rotation(direction.angle());
                    bpscene.upcast()
                    // parent.add_child(bpscene.clone().upcast())
                    
                }
                else {
                    crate::explosion_particle::OneTimeParticle::new_alloc().upcast()
                }
            );
            // if let Some(part_scene)=self.balst_particle_scene.as_ref(){
            //     let mut bpscene=part_scene.instantiate_as::<crate::explosion_particle::OneTimeParticle>();
            //     bpscene.bind_mut().base_mut().set_position(current_position);
            //     bpscene.bind_mut().base_mut().set_rotation(direction.angle());
            //     // parent.add_child(bpscene.clone().upcast())
                
            // }
            },
            SpecialButton::None=>{
                // same as the panic cases
                panic!("Imposible this should be ether Dash or Shoot Not {:?}" ,butn);
            }
        }
        // for more control
        let dur:f64=self.duration as f64;
        //// the player is dashing so the main scene should know about it
        // nah now the player has a type. it is localized
        self.set_type(crate::actortype::Types::Player(true));
        // tween animating the player casually

        aniamtor_tween.tween_property(self.base_mut().clone().upcast(), "position".into(), Variant::from(current_position), dur);
        aniamtor_tween.connect("finished".into(), self.base().callable("on_tween_anim_stop"));
        
    }
    }
    #[func]
    fn on_tween_anim_stop(&mut self){
        // just tell the main scene that we have stoped dashing
        self.set_type(crate::actortype::Types::Player(false));

    }
    #[func]
    fn on_dash_recharge(&mut self){
        // get the timer
        let tmr=&mut self.base_mut().get_node_as::<Timer>("Recharge_Dash");
        // recharge till rem dash=3
        if self.rem_dash<3{
        self.rem_dash+=1;
        // let everyone know that dash has changed
        let varient=Variant::from(self.rem_dash);
        self.base_mut().emit_signal("dash_change".into(), &[varient]);
        }
        // 
        tmr.set_wait_time(min::<f64>(3.0,max::<f64>(self.rem_dash as f64,1.0)));
        
    }
    #[func]
    fn on_genuen_death(&mut self){
        // hide self
        self.base_mut().hide();
        // get the collision shape and disable it and start a particle effect
        let cs2d=&mut self.base_mut().get_node_as::<CollisionShape2D>("CollisionShape2D");
        cs2d.set_deferred("disabled".into(), Variant::from(true));
        self.is_alive=false;
        if let Some(particle_info)=self.death_particle_scene.as_ref(){
            let mut part_scene=particle_info.instantiate_as::<crate::explosion_particle::OneTimeParticle>();
            part_scene.set_position(self.base_mut().get_position());
            part_scene.set_rotation(self.base_mut().get_rotation());
            //part_scene.set_emitting(true);
            self.base_mut().get_parent().unwrap().add_child(part_scene.clone().upcast());
        }
        self.base_mut().emit_signal("death".into(), &[]);
    }
    #[func]
    pub fn start(&mut self,pos:Vector2){
        
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
