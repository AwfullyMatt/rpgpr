use bevy::{prelude::*, utils::hashbrown::HashMap};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub struct ChancePlugin;
impl Plugin for ChancePlugin {
    fn name(&self) -> &str {
        "Chance Plugin"
    }

    fn build(&self, app: &mut App) {
        app;
    }
}

#[derive(Clone, Copy, Default, Deserialize, Serialize)]
pub struct Chance {
    pub success: f32,
    pub failure: f32,
}

#[derive(Clone, Copy, Default, Deserialize, Serialize)]
pub enum ChanceKind {
    #[default]
    DEFAULT,
    LOOT,
    ENEMY,
    BACKGROUND,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Weightings(pub Vec<Weighting>);

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Weighting {
    pub kind: ChanceKind,
    pub chance: Chance,
    pub pool: HashMap<usize, f32>,
}
impl Weighting {
    pub fn new(success: f32, failure: f32, kind: ChanceKind, pool: HashMap<usize, f32>) -> Self {
        Self {
            kind,
            chance: Chance { success, failure },
            pool,
        }
    }

    pub fn weigh(&self) {
        let weight: f32 = self.pool.values().sum();
        let len = self.pool.len();
        let mut min: f32 = 0.;
        let mut max: f32 = 0.;

        for &value in self.pool.values() {
            if min == 0. {
                min = value;
            } else {
                min = min.min(value);
            }

            if max == 0. {
                max = value;
            } else {
                max = max.max(value);
            }
        }

        let vec: Vec<f32> = self
            .pool
            .values()
            .map(|f| (f - min) / (max - min))
            .collect();

        let mut rng = thread_rng();

        let f: f32 = rng.gen_range(0.0..1.0);
    }
}
