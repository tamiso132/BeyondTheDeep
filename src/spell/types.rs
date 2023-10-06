pub mod SpellAttributes {
    pub type Name = [u8; 50];
    pub type Desc = [u8; 250];
    pub type Radius = u8;
    pub type Range = u8;

    #[repr(u8)]
    #[derive(Clone)]
    pub enum TargetBehavior {
        SingleTarget,
        MultiTarget,
        SelfTarget,
        Aoe,
    }

    #[repr(u8)]
    #[derive(Clone)]
    pub enum TargetAlignment {
        Ally,
        Enemy,
        Object,
        All,
    }

    #[repr(u8)]
    #[derive(Clone)]
    pub enum SpellSchool {
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
    pub enum SpellLevel {
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
    pub enum Class {
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
    pub enum Duration {
        Instant,
        Rounds(u8),
        Concentration,
        LongRest,
    }
}
pub mod AoeAttributes {
    pub type Radius = u8;
}
