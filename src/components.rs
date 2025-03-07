use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub bg: RGB,
    pub fg: RGB,
    pub glyph: rltk::FontCharType,
}

#[derive(Component, Debug)]
pub struct Player {}
