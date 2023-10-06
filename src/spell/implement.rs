use super::types::SpellAttributes;

const AOE_OFFSET: u32 = 0;
pub struct AoeSpell {}
fn create_aoe_spell(
    name: SpellAttributes::Name,
    description: SpellAttributes::Desc,
    range: SpellAttributes::Range,
    radius: SpellAttributes::Radius,
    target_aligment: SpellAttributes::TargetAlignment,
    spell_level: SpellAttributes::SpellLevel,
) -> AoeSpell {
    assert!(name.len() < 50);
    assert!(description.len() < 250);
    let name_bytes = name.as_bytes();
    let description_bytes = description.as_bytes();

    let mut u8_name: [u8; 50] = [0; 50];
    let mut u8_desc: [u8; 250] = [0; 250];

    // Copy bytes from the name and description into the fixed-size arrays
    u8_name[..name_bytes.len()].copy_from_slice(name_bytes);
    u8_desc[..description_bytes.len()].copy_from_slice(description_bytes);

    let description_info = SpellDescriptionInfo::new(u8_name, u8_desc);
    let target_info = SpellTargetInfo::new(
        range,
        super::TargetBehavior::Aoe,
        target_aligment,
        spell_level,
    );

    let aoe_property = AoeProperty { radius };

    AoeSpell {
        desc: description_info,
        target_info: target_info,
        property: aoe_property,
    }
}

fn create_aoe_spells(spells: &mut BasicSpellInfo) {}

lazy_static::lazy_static!(
    pub static ref AOE_SPELL_DESCRIPTION: Vec<AoeSpell> =

    vec![
        create_aoe_spell("
        name
        ", "
        You hurl a bubble of acid. Choose one creature you can see within range,
        or choose two creatures you can see
        within range that are within 5 feet of each other. 
        A target must succeed on a Dexterity saving throw or take 1d6 acid damage.
        ", 10, 5, TargetAlignment::All, SpellLevel::Eight),
    ];
);
