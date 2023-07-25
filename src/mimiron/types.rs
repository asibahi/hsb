use anyhow::{anyhow, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt::Display, iter};

#[derive(PartialEq, Eq, Hash)]
pub enum Class {
    DeathKnight,
    DemonHunter,
    Druid,
    Hunter,
    Mage,
    Paladin,
    Priest,
    Rogue,
    Shaman,
    Warlock,
    Warrior,
    Neutral,
}
impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Class::DeathKnight => "DeathKnight",
            Class::DemonHunter => "DemonHunter",
            Class::Druid => "Druid",
            Class::Hunter => "Hunter",
            Class::Mage => "Mage",
            Class::Paladin => "Paladin",
            Class::Priest => "Priest",
            Class::Rogue => "Rogue",
            Class::Shaman => "Shaman",
            Class::Warlock => "Warlock",
            Class::Warrior => "Warrior",
            Class::Neutral => "Neutral",
        };
        write!(f, "{s}")
    }
}
impl TryFrom<u8> for Class {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Self> {
        match value {
            1 => Ok(Class::DeathKnight),
            14 => Ok(Class::DemonHunter),
            2 => Ok(Class::Druid),
            3 => Ok(Class::Hunter),
            4 => Ok(Class::Mage),
            5 => Ok(Class::Paladin),
            6 => Ok(Class::Priest),
            7 => Ok(Class::Rogue),
            8 => Ok(Class::Shaman),
            9 => Ok(Class::Warlock),
            10 => Ok(Class::Warrior),
            12 => Ok(Class::Neutral),
            _ => Err(anyhow!("Class does not exist.")),
        }
    }
}

pub enum Rarity {
    Common,
    Free,
    Rare,
    Epic,
    Legendary,
}
impl Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = match self {
            Rarity::Common => "common".white(),
            Rarity::Free => "free".white(),
            Rarity::Rare => "rare".blue(),
            Rarity::Epic => "epic".purple(),
            Rarity::Legendary => "Legendary".bright_yellow().bold(),
        }
        .italic();
        write!(f, "{r}")
    }
}
impl TryFrom<u8> for Rarity {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            1 => Ok(Rarity::Common),
            2 => Ok(Rarity::Free),
            3 => Ok(Rarity::Rare),
            4 => Ok(Rarity::Epic),
            5 => Ok(Rarity::Legendary),
            _ => Err(anyhow!("Rarity does not exist")),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum SpellSchool {
    Arcane,
    Fire,
    Frost,
    Nature,
    Holy,
    Shadow,
    Fel,
}
impl Display for SpellSchool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SpellSchool::Arcane => "Arcane",
            SpellSchool::Fire => "Fire",
            SpellSchool::Frost => "Frost",
            SpellSchool::Nature => "Nature",
            SpellSchool::Holy => "Holy",
            SpellSchool::Shadow => "Shadow",
            SpellSchool::Fel => "Fel",
        };

        write!(f, "{s}")
    }
}
impl TryFrom<u8> for SpellSchool {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            1 => Ok(SpellSchool::Arcane),
            2 => Ok(SpellSchool::Fire),
            3 => Ok(SpellSchool::Frost),
            4 => Ok(SpellSchool::Nature),
            5 => Ok(SpellSchool::Holy),
            6 => Ok(SpellSchool::Shadow),
            7 => Ok(SpellSchool::Fel),
            _ => Err(anyhow!("Spell School doesn't exist")),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum MinionType {
    Undead,
    Murloc,
    Demon,
    Mech,
    Elemental,
    Beast,
    Totem,
    Pirate,
    Dragon,
    All,
    Quilboar,
    Naga,
}
impl Display for MinionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            MinionType::Undead => "Undead",
            MinionType::Murloc => "Murloc",
            MinionType::Demon => "Demon",
            MinionType::Mech => "Mech",
            MinionType::Elemental => "Elemental",
            MinionType::Beast => "Beast",
            MinionType::Totem => "Totem",
            MinionType::Pirate => "Pirate",
            MinionType::Dragon => "Dragon",
            MinionType::All => "Amalgam",
            MinionType::Quilboar => "Quilboar",
            MinionType::Naga => "Naga",
        };

        write!(f, "{t}")
    }
}
impl TryFrom<u8> for MinionType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            11 => Ok(MinionType::Undead),
            14 => Ok(MinionType::Murloc),
            15 => Ok(MinionType::Demon),
            17 => Ok(MinionType::Mech),
            18 => Ok(MinionType::Elemental),
            20 => Ok(MinionType::Beast),
            21 => Ok(MinionType::Totem),
            23 => Ok(MinionType::Pirate),
            24 => Ok(MinionType::Dragon),
            26 => Ok(MinionType::All),
            43 => Ok(MinionType::Quilboar),
            92 => Ok(MinionType::Naga),
            _ => Err(anyhow!("Minion Type not implemented")),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RuneCost {
    blood: u8,
    frost: u8,
    unholy: u8,
}
impl Display for RuneCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bl = "B".repeat(self.blood as usize);
        let fr = "F".repeat(self.frost as usize);
        let un = "U".repeat(self.unholy as usize);
        write!(f, "{bl}{fr}{un}")
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CardData {
    // Unique identifiers
    id: usize,
    slug: String,

    // basic info
    collectible: u8,

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
    artist_name: String,
    image: String,
    flavor_text: String,
    crop_image: String,

    // Related cards
    parent_id: usize,
    copy_of_card_id: Option<usize>,
    child_ids: Option<Vec<usize>>,

    keyword_ids: Option<Vec<i64>>,
}

pub enum CardType {
    Hero {
        armor: u8,
    },
    Minion {
        attack: u8,
        health: u8,
        minion_types: HashSet<MinionType>,
    },
    Spell {
        school: Option<SpellSchool>,
    },
    Weapon {
        attack: u8,
        durability: u8,
    },
    Location {
        durability: u8,
    },
}
impl Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardType::Hero { armor } => write!(f, "Hero card with {armor} armor"),
            CardType::Minion {
                attack,
                health,
                minion_types,
            } => {
                if minion_types.is_empty() {
                    write!(f, "{attack}/{health} minion")
                } else {
                    let types = minion_types
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join("/");
                    write!(f, "{attack}/{health} {types}")
                }
            }
            CardType::Spell { school } => match school {
                Some(s) => write!(f, "{s} spell"),
                None => write!(f, "spell"),
            },
            CardType::Weapon { attack, durability } => write!(f, "{attack}/{durability} weapon"),
            CardType::Location { durability } => write!(f, "{durability} durability location"),
        }
    }
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
    /*
    image: String,
    crop_image: String,

    tokens: HashSet<usize>,

    flavor_text: String,
    */
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.bold();
        let id = self.id;
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

        write!(
            f,
            "{name}({id}): {rarity} {class} {runes}{cost} mana {card_info} from set {set}.\n{text}"
        )
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
            /*
            image: c.image,
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
pub struct Response {
    pub cards: Vec<Card>,
    // card_count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Authorization {
    pub access_token: String,
    pub expires_in: i64,
    token_type: String,
}

#[derive(Deserialize)]
pub struct Credentials {
   pub blizzard_client_id: String,
   pub blizzard_client_secret: String,
}
