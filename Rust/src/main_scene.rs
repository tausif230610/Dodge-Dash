
use godot::engine::file_access::ModeFlags;
use godot::engine::utilities::{randf, randf_range, randomize};
use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::engine::{FileAccess, Marker2D, Node, PathFollow2D, Timer};
#[derive(GodotClass)]
#[class(base=Node)]
struct MainScene{
score:u16,
hscore:u16,
time_slow_factor:Option<u8>,
combo_level:u16,
#[export]
hscore_file_path:GString,
#[export]
mob_scene:Option<Gd<PackedScene>>,

base:Base<Node>
}
#[godot_api]
impl INode for MainScene {
    fn init(base:Base<Node>)->Self{
    Self{    score: 0,
            hscore: 0,
            time_slow_factor:None,
            combo_level:0,
    hscore_file_path: GString::new(),
    mob_scene:None,
    base }
    }
    fn ready(&mut self){
        self.score=0;
        self.hscore=match FileAccess::open(self.hscore_file_path.clone() , ModeFlags::READ){
            Some(file_stuf)=>{
                file_stuf.get_16()
            },
            None=>0,
        };
        self.start_game();
    }

}
#[godot_api]
impl MainScene {
    #[func]
    fn on_score_timer_timeout(&mut self){
        self.score+=1;
    }
    #[func]
    fn mob_death_handler(&mut self){
    godot_print!("mob_died");
    let timer=&mut self.base_mut().get_node_as::<Timer>("ComboLevelReseter");
    timer.start();
    self.combo_level+=1;
    self.score+=self.combo_level*10;
    }
    #[func]
    fn combo_level_reseter(&mut self){
        if self.combo_level!=0{
        self.combo_level=0;
        }
    }
    #[func]
    fn on_mob_timer_timeout(&mut self){
        
        let mut mob_scene=self.mob_scene.as_mut().unwrap().instantiate_as::<crate::mindless_mover::MindlessMover>();
        let mut mobloc=self.base_mut().get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");
        mobloc.set_progress_ratio(randf() as f32);
        let mob_position=mobloc.get_position();
        let mob_rotation=mobloc.get_rotation()+(crate::PI/2.0)+randf_range(-(crate::PI/4.0) as f64, (crate::PI/4.0) as f64) as f32;
        mob_scene.bind_mut().base_mut().set_position(mob_position);
        if let Some(sd) = self.time_slow_factor {
            mob_scene.bind_mut().slow_down_handler(sd, false);
        }
        mob_scene.bind_mut().angle=crate::_godot_radian_angle_to_semi_broken_hex_angle(mob_rotation);
        mob_scene.connect("death".into(), self.base_mut().callable("mob_death_handler"));
        self.base_mut().add_child(mob_scene.clone().upcast());
    }
    #[func]
    fn on_start_timer_timeout(&mut self) {
        let mut score_timer=self.base_mut().get_node_as::<Timer>("ScoreTimer");
        let mut  mob_timer=self.base_mut().get_node_as::<Timer>("MobTimer");
        score_timer.start();
        mob_timer.start();
    }
    #[func]
    fn game_over(&mut self){
        let mut score_timer=self.base_mut().get_node_as::<Timer>("ScoreTimer");
        let mut  mob_timer=self.base_mut().get_node_as::<Timer>("MobTimer");
        score_timer.stop();
        mob_timer.stop();
    }
    #[func]
    fn player_time_slow_handler(&mut self,speed_devisor:u8){
        if let None=self.time_slow_factor{
        let mut slow_down_handler=self.base_mut().get_node_as::<Timer>("SlowDownTimer");
        self.base_mut().get_tree().unwrap().call_group("Mob".into(), "slow_down_handler".into(), &[Variant::from(speed_devisor),Variant::from(false)]);
        slow_down_handler.start();
        self.time_slow_factor=Some(speed_devisor);
        }
    }
    #[func]
    fn on_slow_down_end(&mut self){
        let tsfactor=self.time_slow_factor.unwrap();
        self.base_mut().get_tree().unwrap().call_group("Mob".into(), "slow_down_handler".into(), &[Variant::from(tsfactor),Variant::from(true)]);
        self.time_slow_factor=None;
    }
    #[func]
    fn on_order_66(&mut self){
        let mob_num=self.base_mut().get_tree().as_mut().unwrap().get_nodes_in_group("Mob".into()).len() as u16;
        self.score+=mob_num*10;
        self.base_mut().get_tree().unwrap().call_group("Mob".into(), "on_viewport_exit".into(), &[]);
    }
    #[func]
    fn start_game(&mut self){
        randomize();
        let mut start_timer=self.base_mut().get_node_as::<Timer>("StartTimer");
        let start_pos:Vector2=self.base_mut().get_node_as::<Marker2D>("PlayerStartLocation").get_position();
        let mut player=self.base_mut().get_node_as::<crate::player::Player>("Player");
        player.bind_mut().start(start_pos);
        start_timer.start();
        self.base_mut().get_tree().unwrap().call_group("Mob".into(),"kill_command".into(), &[]);
    }

}
