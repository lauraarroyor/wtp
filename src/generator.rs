use std::{
    env::Args,
    iter::{Peekable, Skip},
    ops::RangeInclusive,
};

use rand::rng;

use crate::{
    game::Game,
    poke::{PokeInfo, region_from_str},
    util,
};

/// A random Pokémon generator.
#[derive(Clone, Debug)]
pub struct Generator {
    /// A mask of allowed regions (least significant bit is Kanto).
    mask: u32,
}

impl Generator {
    /// A list of all region 'dex ranges, from Kanto to Paldea.
    const REGIONS: [RangeInclusive<usize>; 9] = [
        1..=151,
        152..=251,
        252..=386,
        387..=493,
        494..=649,
        650..=721,
        722..=809,
        810..=898,
        899..=1025,
    ];

    /// Returns a new generator from the command line arguments.
    pub fn new() -> Result<Self, String> {
        use std::env::args;

        let mut args = args().skip(1).peekable();
        let mut r = Self { mask: !0 };

        while let Some(arg) = args.next() {
            match arg.as_str() {
                // Exclude the following regions.
                "-x" => r.disable_regions(&mut args),

                // Enable only the following regions.
                "-r" => r.enable_only_regions(&mut args),

                _ => return Err(format!("Invalid parameter: {arg}")),
            }
        }

        Ok(r)
    }

    /// Generates a random Pokémon info.
    pub async fn generate(&self) -> Result<Game, String> {
        use util::get_poke_json;

        let (region, n) = self.get_random_index()?;
        let json = get_poke_json(n).await?;
        let info = PokeInfo::new(&json, region)?;

        Ok(Game::new(info))
    }

    /// Reads the arguments to disable the given regions.
    fn disable_regions(&mut self, iter: &mut Peekable<Skip<Args>>) {
        while let Some(region) = iter.peek().and_then(|s| region_from_str(s)) {
            self.mask &= !(1 << region);
            iter.next();
        }
    }

    /// Reads the arguments to enable the given regions.
    fn enable_only_regions(&mut self, iter: &mut Peekable<Skip<Args>>) {
        self.mask = 0;

        while let Some(region) = iter.peek().and_then(|s| region_from_str(s)) {
            self.mask |= 1 << region;
            iter.next();
        }
    }

    /// Returns a random number from the active regions and its region index.
    fn get_random_index(&self) -> Result<(usize, usize), String> {
        use rand::seq::IteratorRandom;

        let iter = Self::REGIONS
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| self.mask & (1 << i) != 0)
            .flat_map(|(i, r)| r.map(move |n| (i, n)));

        iter.choose(&mut rng())
            .ok_or_else(|| "No regions were enabled".to_string())
    }
}
