//! 756: Monsters
use std::fs;

use cchol_pm::{Gendered, HasName, HasRollRange};
use dicebag::DiceExt;
use lazy_static::lazy_static;
use rpgassist::{gender::{Gender, HasGender}, ext::IsNamed};
use serde::{Deserialize, Serialize};

use crate::{racial::{Race, race::RACES}, roll_range::*, serialize::{default_pc_save_cr_range, deserialize_cr_range, validate_cr_ranges}};

static MONSTER_FILE: &'static str = "./data/monsters.json";
lazy_static! {
    static ref MONSTERS: Vec<Monster>
        = serde_jsonc::from_str(
            &fs::read_to_string(MONSTER_FILE)
                .expect(format!("Error with '{MONSTER_FILE}'?!").as_str())
        ).expect("JSON error");

    static ref MONSTER_RANGE: RollRange = validate_cr_ranges("MONSTERS", &*MONSTERS, None);
}

#[derive(Debug, Deserialize, Serialize, Clone, HasRollRange, Gendered, HasName)]
pub struct Monster {
    name: String,
    #[serde(default, skip_serializing)]
    variants: Vec<String>,
    #[serde(deserialize_with = "deserialize_cr_range")]
    _cr_range: RollRange,
    #[serde(default)]
    gender: Gender,
}

impl Monster {
    /// Generate a random monster.
    pub fn random() -> Self {
        // +4 → accommodate for a) Race (some beastman), b) Race (reptilians), c/d) GM specials 756A & 756B.
        let roll = 1.d((*(*MONSTER_RANGE).end() + 4) as usize);
        if roll > *MONSTER_RANGE.end() {
            let gender = Gender::random();
            match roll - *MONSTER_RANGE.end() - 1 {
                ..=0 => return Self { name: {
                    let bs = RACES.iter().filter(|r| r.is_beastman()).collect::<Vec<&'static Race>>();
                    let idx = 1.d(bs.len()) - 1;
                    bs[idx].name().into()
                }, variants: vec![], _cr_range: default_pc_save_cr_range(), gender },
                1 => return Self { name: {
                    let bs = RACES.iter().filter(|r| r.is_reptilian()).collect::<Vec<&'static Race>>();
                    let idx = 1.d(bs.len()) - 1;
                    bs[idx].name().into()
                }, variants: vec![], _cr_range: default_pc_save_cr_range(), gender },
                2 => return Self { name: "GM#756A".into(), variants: vec![], _cr_range: default_pc_save_cr_range(), gender },
                _ => return Self { name: "GM#756B".into(), variants: vec![], _cr_range: default_pc_save_cr_range(), gender },
            }
        }
        let mon = MONSTERS.get_random_in_range(&MONSTER_RANGE);
        let name = if mon.variants.is_empty() {
            mon.name.clone()
        } else {
            let idx = 1.d(mon.variants.len());
            mon.variants[idx - 1].clone()
        };
        Self { name, variants: vec![], ..mon.clone() }
    }
}

#[cfg(test)]
mod monster_tests {
    use crate::racial::monster::MONSTER_RANGE;

    #[test]
    fn monster_file_data_integrity() {
        let x = &*MONSTER_RANGE;
        assert_eq!(1, *x.start());
        // by default there's 16 monster 'categories'; +4 for hardcoded specials…
        assert!(20 <= *x.end() + 4);
    }
}