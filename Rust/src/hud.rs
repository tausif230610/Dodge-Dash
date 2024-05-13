use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::engine::{Button, CanvasLayer, HBoxContainer, ICanvasLayer, Label, Timer};
#[derive(GodotClass)]
#[class(init,base=CanvasLayer)]
struct HUD{
    #[init(default=false)]
    
    mobile_mode:bool,
    base:Base<CanvasLayer>
}
#[godot_api]
impl ICanvasLayer for HUD {
    fn ready(&mut self){
        self.base_mut().get_tree().unwrap().call_group("Mobile".into(), "hide".into(), &[]);        
    }
}
#[godot_api]
impl HUD {
    #[signal]
    fn start_game();
    #[signal]
    fn toggle_help();
    #[func]
    fn show_temp_message(&mut self,text:GString){
        let textbox=&mut self.base_mut().get_node_as::<Label>("TextBoxLabel");
        let mut textboxtimer=self.base_mut().get_node_as::<Timer>("TextBoxLabel/Timer");
        textbox.set_text(text);
        textbox.show();
        textboxtimer.start();
        //failed callable trick
        // //let mut text_box_mutexed=Arc::new(Mutex::new(0));
        // //let (tx,rx)=channel::<Gd<Label>>();
        // //let (dta,t)=(&Arc::clone(&text_box_mutexed),tx.clone());
        // //textboxtimer.connect("timeout".into(), Callable::from_fn("timeout_handler", |args:&[&Variant]|{
        // //let mut textdata=text_box_mutexed.lock().unwrap();
        // //textbox.set_text(GString::from(""));
        // //Result::Ok(Variant::nil())
        // //}));
    }
    #[func]
    fn eth(&mut self){
        self.base_mut().emit_signal("toggle_help".into(), &[]);
    }
    #[func]
    fn update_scores(&mut self,score:u16,hscore:u16){
        let mut score_label=self.base_mut().get_node_as::<Label>("ScoreLabel");
        let mut hi_score_label=self.base_mut().get_node_as::<Label>("HScoreLabel");
        let mut score_string:String=String::from("SC: 0");
        score_string.push_str(score.to_string().as_str());
        let mut hi_score_string:String=String::from("HI: 0");
        hi_score_string.push_str(hscore.to_string().as_str());

        score_label.set_text(score_string.into());
        hi_score_label.set_text(hi_score_string.into());
        
    }
    #[func]
    fn on_start_button_pressed(&mut self){
        self.base_mut().get_node_as::<Button>("StartButton").hide();
        self.base_mut().emit_signal("start_game".into(), &[]);
        self.base_mut().get_node_as::<Button>("Buttons/Help_Toogle").hide();

    }
    #[func]
    fn txttmrout(&mut self){
    let mut textbox=self.base_mut().get_node_as::<Label>("TextBoxLabel");
    textbox.set_text(GString::from(""));
    textbox.hide();
    }
    #[func]
    fn show_game_over(&mut self){
        self.show_temp_message("Game Over".into());
        let mut textboxtimer=self.base_mut().get_node_as::<Timer>("TextBoxLabel/Timer");
        textboxtimer.start();
        textboxtimer.connect("timeout".into(), self.base_mut().callable("show_game_over_2"));

    }
    #[func]
    fn show_game_over_2(&mut self){
        let mut textboxtimer=self.base_mut().get_node_as::<Timer>("TextBoxLabel/Timer");
        
        textboxtimer.disconnect("timeout".into(),self.base_mut().callable("show_game_over_2"));
        let textbox=&mut self.base_mut().get_node_as::<Label>("TextBoxLabel");
        textbox.set_text("Dodge & Dash".into());
        textbox.show();
        
        let mut game_over_timer=self.base_mut().get_node_as::<Timer>("OverTimer");
        game_over_timer.start();


    }
    #[func]
    fn finalize_game_over(&mut self){
        self.base_mut().get_node_as::<Button>("StartButton").show();
        self.base_mut().get_node_as::<Button>("Buttons/Help_Toogle").show();
    }
    #[func]
    fn toggle_mobile(&mut self){
        self.mobile_mode=!self.mobile_mode;
        if !self.mobile_mode{
        self.base_mut().get_tree().unwrap().call_group("Mobile".into(), "hide".into(), &[]);
    }
    else {
        self.base_mut().get_tree().unwrap().call_group("Mobile".into(), "show".into(), &[]);
    }
}
    
    #[func]
    fn on_dash_change(&mut self,level:u8){
        let mut dashbutton=self.base_mut().get_node_as::<crate::ui_button::SUIButton>("Button_Container/Dashbutton");
        dashbutton.bind_mut().on_level_change(level);

    }
    #[func]
    fn on_shoot_change(&mut self,level:u8){
        let mut shootbutton=self.base_mut().get_node_as::<crate::ui_button::SUIButton>("Button_Container/ShootButton");
        shootbutton.bind_mut().on_level_change(level);
    }
    #[func]
    fn on_life_change(&mut self,life:u8){
        let mut life_container=self.base_mut().get_node_as::<HBoxContainer>("LifeContainer");
        life_container.call("on_health_change".into(), &[Variant::from(life)]);
    }
}