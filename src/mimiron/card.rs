use anyhow::{anyhow, Result};
use colored::Colorize;
use serde::Deserialize;
use std::{collections::HashSet, fmt::Display, iter};

use crate::mimiron::card_details::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CardData {
    // Unique identifiers
    id: usize,
    // slug: String,

    // basic info

    // collectible: u8,
    card_type_id: u8,
    class_id: u8,
    multi_class_ids: Vec<u8>,

    rarity_id: u8,
    card_set_id: usize,

    name: String,
    text: String,

    // Stats
    mana_cost: u8,
    rune_cost: Option<RuneCost>,

    attack: Option<u8>,
    health: Option<u8>,
    durability: Option<u8>,
    armor: Option<u8>,

    // Additional Info
    minion_type_id: Option<u8>,
    multi_type_ids: Option<Vec<u8>>,

    spell_school_id: Option<u8>,

    // Flavor
    //artist_name: String,
    image: String,
    //flavor_text: String,
    //crop_image: Option<String>,

    // Related cards
    //parent_id: usize,
    copy_of_card_id: Option<usize>,
    //child_ids: Option<Vec<usize>>,

    //keyword_ids: Option<Vec<i64>>,
}

#[derive(Deserialize)]
#[serde(try_from = "CardData")]
pub struct Card {
    pub id: usize,
    pub card_set: usize,

    pub name: String,
    pub class: HashSet<Class>,

    pub cost: u8,
    pub rune_cost: Option<RuneCost>,

    pub card_type: CardType,
    pub rarity: Rarity,

    pub text: String,

    pub dup: bool,

    pub image: String,
    /*
    crop_image: String,

    tokens: HashSet<usize>,

    flavor_text: String,
    */
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.rarity.partial_cmp(&other.rarity) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.cost.partial_cmp(&other.cost) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.name.partial_cmp(&other.name)
    }
}
impl std::hash::Hash for Card {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl Eq for Card {}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.rarity.cmp(&other.rarity) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.cost.cmp(&other.cost) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.name.cmp(&other.name)
    }
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.bold();
        let cost = self.cost;

        let runes = match &self.rune_cost {
            Some(r) => format!("{r} "),
            None => String::new(),
        };

        let set = self.card_set;
        let text = &self.text;

        let rarity = &self.rarity;

        let class = self
            .class
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("/")
            .green();

        let card_info = &self.card_type;

        let img = &self.image;

        write!(
            f,
            "{name:25} {rarity} {class} {runes}{cost} mana {card_info}."
        )?;

        if f.alternate() {
            write!(f, "Set {set}.\n{text}\nImage: {img}")?;
        }
        Ok(())
    }
}
impl TryFrom<CardData> for Card {
    type Error = anyhow::Error;

    fn try_from(c: CardData) -> Result<Self> {
        let c = Card {
            id: c.id,
            card_set: c.card_set_id,
            name: c.name,
            class: if c.multi_class_ids.is_empty() {
                HashSet::from([c.class_id.try_into()?])
            } else {
                c.multi_class_ids
                    .into_iter()
                    .map(Class::try_from)
                    .collect::<Result<HashSet<_>>>()?
            },
            cost: c.mana_cost,
            rune_cost: c.rune_cost,
            card_type: match c.card_type_id {
                3 => CardType::Hero {
                    armor: c.armor.unwrap(),
                },
                4 => CardType::Minion {
                    attack: c.attack.unwrap(),
                    health: c.health.unwrap(),
                    minion_types: match (c.minion_type_id, c.multi_type_ids) {
                        (None, _) => HashSet::new(),
                        (Some(t), None) => HashSet::from([t.try_into()?]),
                        (Some(t), Some(v)) => iter::once(t)
                            .chain(v)
                            .map(MinionType::try_from)
                            .collect::<Result<HashSet<_>>>()?,
                    },
                },
                5 => CardType::Spell {
                    school: c.spell_school_id.map(SpellSchool::try_from).transpose()?,
                },
                7 => CardType::Weapon {
                    attack: c.attack.unwrap(),
                    durability: c.durability.unwrap(),
                },
                39 => CardType::Location {
                    durability: c.durability.unwrap(),
                },
                _ => return Err(anyhow!("Card type does not exist")),
            },
            rarity: c.rarity_id.try_into()?,
            text: c.text,

            dup: c.copy_of_card_id.is_some(),

            image: c.image,
            /*
            crop_image: c.crop_image,
            tokens: match c.child_ids {
                Some(v) => HashSet::from_iter(v),
                None => HashSet::new(),
            },
            flavor_text: c.flavor_text,
            */
        };

        Ok(c)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardSearchResponse {
    pub cards: Vec<Card>,
    // card_count: usize,
}
