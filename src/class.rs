struct LevelUp {}

struct Level {
    current_xp: u16,
    xp_to_level: u16,
}

struct Barbarian {}

struct BarbarianlevelUp {}

struct Spell {
    name: String,
    description: String,
    mana_cost: u16,
}
#[repr(u8)]
enum SpellType {
    SingleTarget,
    MultiTarget,
}

struct SingleTargetEnemy {}

struct SingleTargetAlly {}

struct MultiTargetAlly {}

struct MultiTargetEnemy {}
