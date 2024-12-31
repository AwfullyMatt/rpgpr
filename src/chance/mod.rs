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

    pub fn weigh(&self) -> Option<usize> {
        // Set min/max values for normalization
        let min: f32 = self
            .pool
            .values()
            .copied()
            .fold(f32::INFINITY, |x, y| x.min(y));
        let max: f32 = self
            .pool
            .values()
            .copied()
            .fold(f32::NEG_INFINITY, |x, y| x.max(y));

        // Clone the hashmap and normalize its chance values
        // once I determine if pools are immutable or not
        // I can just mutate the original if necessary
        let normalized: HashMap<usize, f32> = self
            .pool
            .iter()
            .map(|(k, &v)| {
                let norm = (v - min) / (max - min);
                (k.clone(), norm)
            })
            .collect();

        // Spawn rng thread and gen an f32 from 0 to 1
        let mut rng = thread_rng();
        let gen = rng.gen_range(0.0..=1.0);

        // Cumulative probability to weigh against
        let mut prob: f32 = 0.0;

        // Find the first entry where
        let choice: Option<usize> = normalized.iter().find_map(|(k, &v)| {
            prob += v;
            if gen < prob {
                Some(k.clone())
            } else {
                None
            }
        });

        choice
    }
}
