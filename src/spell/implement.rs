use super::{
    resource::{BasicSpellInfo, SpellDescriptionInfo, SpellTargetInfo},
    AoeProperty, SpellLevel, TargetAlignment,
};

const AOE_OFFSET: u32 = 0;

fn create_aoe_spell(
    spells: &mut BasicSpellInfo,
    name: String,
    description: String,
    range: u8,
    radius: u8,
    target_aligment: TargetAlignment,
    spell_level: SpellLevel,
) {
    let description_info = SpellDescriptionInfo { name, description };
    let target_info = SpellTargetInfo::new(
        range,
        super::TargetBehavior::Aoe,
        target_aligment,
        spell_level,
    );

    let id = spells.push(description_info, target_info);

    let aoe_property = AoeProperty {
        radius,
        id: id as u8,
    };
}

fn create_aoe_spells(spells: &mut BasicSpellInfo) {
    let title = "Acid Splash";
    let description = 
    "You hurl a bubble of acid. 
    Choose one creature you can see within range, or choose two creatures you can see within range that are within 5 feet of each other.
    A target must succeed on a Dexterity saving throw or take 1d6 acid damage.
    At Higher Levels. This spell's damage increases by 1d6 when you reach 5th level (2d6), 11th level (3d6), and 17th level (4d6).";

    create_aoe_spell(
        spells,
        title.to_string(),
        description.to_string(),
        60,
        5,
        TargetAlignment::Enemy,
        SpellLevel::Cantrip,
    )
}

