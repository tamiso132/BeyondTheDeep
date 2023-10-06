use bevy::prelude::{Component, Entity};

#[derive(Component)]
struct Transform {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Component)]
struct BoxCollider {
    width: u32,
    height: u32,
}

struct BroadTree {
    array: Box<Option<[BroadTree; 4]>>,
    bound: (u32, u32),
}

impl BroadTree {
    fn push_entity(entity: Entity) {}
}
