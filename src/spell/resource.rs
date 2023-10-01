use bevy::prelude::Resource;

use super::{Class, SpellLevel, TargetAlignment, TargetBehavior};

#[derive(Resource)]
/// All spells has this attribute
pub struct BasicSpellInfo {
    descriptions_infos: Vec<SpellDescriptionInfo>,
    target_infos: Vec<SpellTargetInfo>,
}
impl BasicSpellInfo {
    pub fn push(
        &mut self,
        description_info: SpellDescriptionInfo,
        target_info: SpellTargetInfo,
    ) -> u16 {
        self.descriptions_infos.push(description_info);
        self.target_infos.push(target_info);

        (self.target_infos.len() - 1) as u16
    }
}

struct AoeProperties {}

/// All spells has this attribute
pub struct SpellDescriptionInfo {
    pub name: String,
    pub description: String,
}

#[derive(Clone)]
pub struct SpellTargetInfo {
    pub range: u8,
    pub behaviour: TargetBehavior,
    pub aliament: TargetAlignment,
    pub spell_level: SpellLevel,
}
impl SpellTargetInfo {
    pub fn new(
        range: u8,
        behaviour: TargetBehavior,
        aliament: TargetAlignment,
        spell_level: SpellLevel,
    ) -> Self {
        Self {
            range,
            behaviour,
            aliament,
            spell_level,
        }
    }
}
