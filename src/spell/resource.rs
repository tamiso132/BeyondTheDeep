use bevy::prelude::Resource;

use super::{Class, SpellLevel, TargetAlignment, TargetBehavior};

#[derive(Resource)]
struct SpellDescriptions {
    spell_descriptions: Vec<Vec<Vec<SpellDescription>>>,
}

impl SpellDescriptions {
    fn new() -> SpellDescriptions {
        let mut spells = SpellDescriptions {
            spell_descriptions: vec![],
        };
        for _ in Class::Bard as u8..=Class::Wizard as u8 {
            spells.spell_descriptions.push(vec![]);
        }
        spells
    }

    fn push_spells(
        &mut self,
        class: Class,
        spell_level: SpellLevel,
        spells: Vec<SpellDescription>,
    ) {
        self.spell_descriptions[(class as u8) as usize][(spell_level as u8) as usize] = spells;
    }
}

#[derive(Resource)]
struct SpellTargetInfos {
    spell_descriptions: Vec<Vec<SpellTargetInfo>>,
}

struct SpellDescription {
    id: u16,
    name: String,
    description: String,
    mana_cost: u16,
}

struct SpellTargetInfo {
    cost: u8,
    range: u8,
    behaviour: TargetBehavior,
    aliament: TargetAlignment,
}

fn setup() {}
