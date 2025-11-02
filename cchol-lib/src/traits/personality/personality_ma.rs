pub mod mental_affliction {
    use std::fmt::Display;

    use dicebag::DiceExt;
    use rpgassist::details::ProperCaseExt;
    use serde::{Deserialize, Serialize};

    use crate::{IsNamed, misc::SeriousWound, racial::Race, social::people::OtherPeople, traits::personality::{self, AffectsAlignment, Alignment, DLNTrait, PersonalityTrait, TraitRollResult, TraitVec, exotic_trait::{self, ExoticTrait}, phobias, random_darkside, random_lightside}};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ExtraPersona {
        traits: TraitVec,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum ObsessiveHatredKind {
        AnyNonhuman,
        ParticularNonhuman(Race),
        Monsters,
        Someone(OtherPeople),
    }

    impl ObsessiveHatredKind {
        pub fn random() -> Self {
            match 1.d4() {
                ..=1 => Self::AnyNonhuman,
                2 => Self::ParticularNonhuman(Race::random_nonhuman().clone()),
                _ => unimplemented!()
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum ObsessiveBehaviorKind {
        Devotion(TraitVec),
    }

    impl ObsessiveBehaviorKind {
        fn random(bans: &TraitVec) -> Self {
            match 1.d10() {
                ..=1 => Self::Devotion(random_lightside(bans).as_vec()),
                2 => Self::Devotion(random_darkside(bans).as_vec()),
                _ => unimplemented!()
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum MentalAffliction {
        Catatonia,
        CompulsiveLying,
        Depression,
        Hallucinations,
        Hypochondria,
        HystericalInjury { perceived_wound: SeriousWound },
        ManicDepressive,
        Megalomania,
        Paranoia,
        SplitPersonality { extras: Vec<ExtraPersona> },
    }

    impl AffectsAlignment for MentalAffliction {
        fn alignment(&self) -> Alignment {
            match self {
                Self::SplitPersonality { .. } => Alignment::NotApplicable,
                Self::CompulsiveLying => Alignment::D,
                Self::Paranoia => Alignment::D,
                Self::Hallucinations => Alignment::NotApplicable,
                Self::HystericalInjury { .. } => Alignment::NotApplicable,
                Self::Catatonia => Alignment::NotApplicable,
                Self::Megalomania => Alignment::D,
                Self::ManicDepressive => Alignment::D,
                Self::Hypochondria => Alignment::N,
                Self::Depression => Alignment::NotApplicable,
            }
        }
    }

    impl Display for MentalAffliction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::SplitPersonality { .. } => write!(f, "split personality"),
                Self::CompulsiveLying => write!(f, "compulsive lying"),
                Self::Paranoia => write!(f, "paranoia"),
                Self::Hallucinations => write!(f, "hallucinations"),
                Self::Catatonia => write!(f, "catatonia"),
                Self::Megalomania => write!(f, "megalomania"),
                Self::ManicDepressive => write!(f, "manic-depressive"),
                Self::Hypochondria => write!(f, "hypochondria"),
                Self::Depression => write!(f, "depression"),
                Self::HystericalInjury { perceived_wound } => write!(f, "hysterical injury: {perceived_wound}"),
            }
        }
    }

    impl IsNamed for MentalAffliction {
        fn name(&self) -> String {
            self.to_string().proper_case()
        }
    }

    impl From<MentalAffliction> for PersonalityTrait {
        fn from(value: MentalAffliction) -> Self {
            PersonalityTrait::EX(ExoticTrait::MA(value))
        }
    }

    /// Generate random mental affliction(s).
    pub fn random(bans: &TraitVec) -> TraitRollResult {
        let mut traits: TraitVec = vec![];
        let mut count = 1;
        while count > 0 {
            count -= 1;
            match 2.d10() {
                ..=2 => traits.push(MentalAffliction::SplitPersonality { extras: {
                    let mut extras = vec![];
                    for _ in 0..1.d3() {
                        let mut p = ExtraPersona { traits: personality::random(bans).as_vec() };
                        if 1.d100() <= 60 {
                            p.traits.extend(exotic_trait::random(bans).as_vec());
                        }
                        extras.push(p);
                    }
                    extras
                } }.into()),

                3 => traits.push(MentalAffliction::CompulsiveLying.into()),
                4 => traits.push(MentalAffliction::Paranoia.into()),
                5 => traits.push(MentalAffliction::Hallucinations.into()),
                6 => traits.push(MentalAffliction::Catatonia.into()),
                7 => traits.push(MentalAffliction::Megalomania.into()),
                8 => traits.extend(phobias::random(bans).as_vec()),
                9 => traits.push(MentalAffliction::ManicDepressive.into()),
                10 => traits.push(MentalAffliction::Hypochondria.into()),
                11|12 => traits.push(MentalAffliction::Depression.into()),
                13 => traits.push(MentalAffliction::HystericalInjury { perceived_wound: SeriousWound::random(bans) }.into()),

                // 5, 6, ... etc. upto 19 TODO

                _ => {
                    count += 1.d3() + 1;
                    continue;
                }
            }
        }

        TraitRollResult::AddMultiple(traits)
    }
}