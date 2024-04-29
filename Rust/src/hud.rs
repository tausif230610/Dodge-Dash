use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::engine::{CanvasLayer, HBoxContainer, ICanvasLayer, Label,Timer};
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
    fn start_game(&mut self);
    #[func]
    fn show_temp_message(&mut self,text:GString){
        let textbox=&mut self.base_mut().get_node_as::<Label>("TextBoxLabel");
        let mut textboxtimer=self.base_mut().get_node_as::<Timer>("TextBoxLabel/Timer");
        textbox.set_text(text);
        textboxtimer.start();

        // let mut text_box_mutexed=Arc::new(Mutex::new(0));
        // //let (tx,rx)=channel::<Gd<Label>>();
        // //let (dta,t)=(&Arc::clone(&text_box_mutexed),tx.clone());
        // textboxtimer.connect("timeout".into(), Callable::from_fn("timeout_handler", |args:&[&Variant]|{
        // let mut textdata=text_box_mutexed.lock().unwrap();
        // //textbox.set_text(GString::from(""));
        // Result::Ok(Variant::nil())
        // }));
    }
    // #[func]
    // fn txttmrout(&mut self){
    //     let mut textbox=self.base_mut().get_node_as::<Label>("TextBoxLabel");
    //     textbox.set_text(GString::from(""));
    // }
    #[func]
    fn show_game_over(&mut self){
        self.show_temp_message("Game Over".into());

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