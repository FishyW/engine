use crate::engine::prelude::*;

#[wasm_bindgen]
pub fn __init_scene_start() {
    let player = crate::objects::Player::default();
    crate::objects::Player::register(player);
}
