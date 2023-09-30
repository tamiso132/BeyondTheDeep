use bevy::prelude::{Resource, World};

mod class;
mod component;

#[derive(Resource)]
struct test {
    i: u32,
}

fn main() {
    println!("Hello, world!");
    let mut world = World::default();
}
