//! `StatMap` for e.g. [PlayerCharacter], etc.

use std::{collections::HashMap, ops::{AddAssign, SubAssign}};

use rpgassist::stat::{Stat, StatBase};
use serde::{Deserialize, Serialize};

/// [Stat] map for e.g. [PlayerCharacter] etc.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StatMap(HashMap<StatBase, Stat>);

impl Default for StatMap {
    /// Generate default [StatMap] with more or less sensible default values for each stat present.
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(StatBase::Age, StatBase::Age.default());
        map.insert(StatBase::App, StatBase::App.default());
        map.insert(StatBase::Cha, StatBase::Cha.default());
        map.insert(StatBase::Con, StatBase::Con.default());
        map.insert(StatBase::Dex, StatBase::Dex.default());
        map.insert(StatBase::Int, StatBase::Int.default());
        map.insert(StatBase::Mag, StatBase::Mag.default());
        map.insert(StatBase::Str, StatBase::Str.default());
        map.insert(StatBase::Will, StatBase::Will.default());
        Self(map)
    }
}

impl StatMap {
    pub fn set(&mut self, stat: Stat) -> Result<(), String> {
        if !self.0.contains_key(&stat.stat_base()) {
            return Err(format!("StatMap does not accept Stat with type '{:?}'", stat.stat_base()))
        }
        self.0.insert(stat.stat_base(), stat);
        Ok(())
    }

    pub fn set_modified_by(&mut self, modifier: Stat) -> Result<(), String> {
        if !self.0.contains_key(&modifier.stat_base()) {
            return Err(format!("StatMap does not accept Stat with type '{:?}'", modifier.stat_base()))
        }
        self.0.entry(modifier.stat_base()).and_modify(|e|*e += modifier);
        Ok(())
    }
}

impl AddAssign<Stat> for StatMap {
    fn add_assign(&mut self, rhs: Stat) {
        self.set_modified_by(rhs.clone())
            .expect(format!("INTERNAL ERROR: Stat '{rhs:?}' is not compatible with this particular StatMap!").as_str())
    }
}

impl AddAssign<i32> for StatMap {
    /// += for each base attribute in the map, excluding modifier kinds.
    fn add_assign(&mut self, rhs: i32) {
        // These are both modifiers, but…
        self.0.entry(StatBase::App).and_modify(|e| *e += rhs );
        self.0.entry(StatBase::Cha).and_modify(|e| *e += rhs );
        // The core stats:
        self.0.entry(StatBase::Con).and_modify(|e| *e += rhs );
        self.0.entry(StatBase::Dex).and_modify(|e| *e += rhs );
        self.0.entry(StatBase::Int).and_modify(|e| *e += rhs );
        self.0.entry(StatBase::Str).and_modify(|e| *e += rhs );
    }
}

impl SubAssign<i32> for StatMap {
    fn sub_assign(&mut self, rhs: i32) {
        (*self) += -(rhs)
    }
}