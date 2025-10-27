use std::collections::HashMap;
use paste::paste;

use dicebag::{DiceExt, IsOne};
use rpgassist::ranking::Rank;

use crate::skill::{Skill, SkillBase};

static SK_ARTISTIC: [&str; 5] = [
    /*876*/"Art: Painting",
    /*876*/"Art: Drawing",
    /*876*/"Art: Sculpting",
    /*876*/"Art: Jeweller",
    /*876*/"Art: Architecture"];
static SK_MUSICAL: [&str; 7] = [
    /*876*/"Music: Play Common Instrument",
    /*876*/"Music: Sing",
    /*876*/"Music: Songwriter",
    /*876*/"Music: Musical Theatre",
    /*876*/"Make/repair Musical Instruments",
    /*876*/"Music: Play Exotic Instrument",
    /*876*/"Music: Play-by-Ear"];
static SK_TEXTILES: [&str; 5] = [
    /*876*/"Sewing",
    /*876*/"Weaving",
    /*876*/"Tapestry Design",
    /*876*/"Embroidery",
    /*876*/"Knitting"];
static SK_THEATRICAL: [&str; 7] = [
    /*876*/"Acting",
    /*876*/"Artistic Dancing",
    /*876*/"Oration",
    /*876*/"Story-telling",
    // 876-16-5 would be 876-9 choice
    /*876*/"Disguise",
    // 876-16-7 would be 876-17 choice
    /*876*/"Voice Impersonation",
    /*876*/"Juggling"];
static SK_CIRCUS: [&str; 6] = [
    /*876*/"Aerial Acrobatics",
    /*876*/"Tight-rope Walking",
    /*876*/"Animal Training",
    /*876*/"Clowning",
    // 876-17-5 would be 876-9 choice
    /*876*/"Disguise",
    /*876*/"Horsemanship"];
static SK_MISCELLANEOUS: [&str; 10] = [
    /*876*/"Astronomy",
    /*876*/"Astrology",
    /*876*/"Calligraphy",
    /*876*/"Lassoing",
    /*876*/"Wine Tasting",
    /*876*/"Sailing: Small Craft",
    /*876*/"Haggling",
    /*876*/"Diplomacy",
    /*876*/"Prestidigitation",
    /*876*/"Imitate Monster Noises"];

macro_rules! generate_random_skillname_at_least_once {
    // To be used only with verified data… run `cargo test data_integrity` just in case.
    ($vec:ident, $tbl:ident, $has_rerolls:expr) => {paste!{{
        let mut num = 1;
        let dice_size = $tbl.len() + if $has_rerolls {1} else {0};
        while num > 0 {
            num -= 1;
            let r = 1.d(dice_size);
            if $has_rerolls && r == dice_size {
                num += 1.d2() + 1
            } else {
                $vec.push($tbl[r-1]);
            }
        }
    }}};
}

/// Generate random unusual skill(s).
pub fn generate_unusual_skills() -> Vec<Skill> {
    fn generate_normally() -> Vec<&'static str> {
        let mut sks = vec![];
        match 1.d(18) {
            ..=1 => sks.push(/*876*/"Social Dancing"),
            2 => sks.push(/*876*/"Professional Gambling"),
            3 => sks.push(/*876*/"Pick Pockets"),
            4 => sks.push(/*876*/"Gourmet Cooking"),
            5 => sks.push(/*876*/"Sexual Seduction"),
            6 => sks.push(/*876*/"Skiing"),
            7 => sks.push(/*876*/"Skating"),
            8 => generate_random_skillname_at_least_once!(sks, SK_ARTISTIC, true),
            9 => generate_random_skillname_at_least_once!(sks, SK_MUSICAL, true),
            10 => generate_random_skillname_at_least_once!(sks, SK_TEXTILES, true),
            11 => sks.push(/*876*/"Mountaineering"),
            12 => sks.push(/*876*/"Opposite Hand Weapon Use"),
            13 => sks.push(/*876*/"Mathematical Skill"),
            14 => sks.push(/*876*/"Model Making"),
            15 => sks.push(/*876*/"Inventing"),
            16 => match 1.d10() {
                    1 => generate_random_skillname_at_least_once!(sks, SK_MUSICAL, true),
                    2 => generate_random_skillname_at_least_once!(sks, SK_CIRCUS, true),
                    _ => generate_random_skillname_at_least_once!(sks, SK_THEATRICAL, true)
                },
            17 => if 1.d8().is_one() {
                    generate_random_skillname_at_least_once!(sks, SK_MUSICAL, true)
                } else {
                    generate_random_skillname_at_least_once!(sks, SK_CIRCUS, true)
                },
            _ => generate_random_skillname_at_least_once!(sks, SK_MISCELLANEOUS, false)
        }
        sks
    }

    let sks = match 1.d20() {
        ..=18 => generate_normally(),
        19 => {
            let mut sks = vec![];
            for _ in 0..2.d3() {
                sks.extend(generate_normally());
            }
            sks
        },
        _ => {
            let mut sks = vec![];
            let gens = generate_normally();
            for _ in 0..1.d2() {
                sks.extend(&gens);
            }
            sks
        }
    };

    let mut skills_map: HashMap<&'static str, Skill> = HashMap::new();
    for name in sks {
        let rank_inc = 1;
        skills_map.entry(name)
            .and_modify(|sk| *sk += rank_inc)
            .or_insert_with(|| Skill::from((SkillBase::from(name), Rank::from(if 1.d6() == 6 {4} else {3}))));
    }

    skills_map.into_values().collect()
}

#[cfg(test)]
mod unusual_skills_tests {
    use crate::skill::{unusual::{SK_ARTISTIC, SK_CIRCUS, SK_MISCELLANEOUS, SK_MUSICAL, SK_TEXTILES, SK_THEATRICAL}, SkillBase};

    macro_rules! assert_all_entries_present {
        ($tbl:ident) => {
            $tbl.iter().all(|name|{
                let _ = SkillBase::from(*name);
                println!("{} '{name}' present and accounted for.", stringify!($tbl));
                true
            });
        };
    }

    #[test]
    fn verify_predefined_data_integrity() {
        assert_all_entries_present!(SK_ARTISTIC);
        assert_all_entries_present!(SK_CIRCUS);
        assert_all_entries_present!(SK_MISCELLANEOUS);
        assert_all_entries_present!(SK_MUSICAL);
        assert_all_entries_present!(SK_TEXTILES);
        assert_all_entries_present!(SK_THEATRICAL);
    }
}