use godot::prelude::*;
use godot::engine::CanvasLayer;
#[derive(GodotClass)]
#[class(init,base=CanvasLayer)]
struct HUD{
    base:Base<CanvasLayer>
}