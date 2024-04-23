

use godot::prelude::*;
use nanoserde::{SerJson,DeJson};

#[derive(Clone, Copy,nanoserde::SerJson,nanoserde::DeJson,PartialEq)]
#[nserde(tag="type")]
pub enum Types {
    Player(bool),
    Mob,
    Bullet,
    Unknown
}
impl Types {
    pub fn godot_gstring_serilize(&mut self)->GString{
        return self.serialize_json().into();
    }
    pub fn godot_gstring_deserilize(str:GString)->Self{
        let tp=DeJson::deserialize_json(str.to_string().as_str()).unwrap_or(Types::Unknown);
        tp
    }
}
impl std::fmt::Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.serialize_json())
}
}