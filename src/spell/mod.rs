use bevy::{
    prelude::{Commands, Component, Entity, Query},
    utils::HashMap,
};
mod implement;
mod resource;

struct AoeProperty {
    id: u8,
    radius: u8,
}

#[repr(u8)]
#[derive(Clone)]
enum TargetBehavior {
    SingleTarget,
    MultiTarget,
    SelfTarget,
    Aoe,
}

#[repr(u8)]
#[derive(Clone)]
enum TargetAlignment {
    Ally,
    Enemy,
    Object,
    All,
}

#[repr(u8)]
#[derive(Clone)]
enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

#[repr(u8)]
#[derive(Clone)]
enum SpellLevel {
    Cantrip,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[repr(u8)]
#[derive(Clone)]
enum Class {
    Bard,
    Cleric,
    Druid,
    Paladin,
    Ranger,
    Sorcerer,
    Warlock,
    Wizard,
}

#[repr(u8)]
#[derive(Clone)]
enum Duration {
    Instant,
    Rounds(u8),
    Concentration,
    LongRest,
}
