use bevy::prelude::Component;
mod resource;

#[derive(Component)]
struct PreparedSpells {}

#[derive(Component)]
struct AoeProperty {
    id: u16,
    radius: u8,
}

#[derive(Component)]
struct MultiTargetProperty {
    id: u16,
    amount: u8,
}

#[repr(u8)]
enum TargetBehavior {
    SingleTarget,
    MultiTarget,
    SelfTarget,
    Aoe,
}

#[repr(u8)]
enum TargetAlignment {
    Ally,
    Enemy,
}

#[repr(u8)]
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
enum Duration {
    Instant,
    Rounds(u8),
    Concentration,
    LongRest,
}
