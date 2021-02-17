use crate::*;

use rand::distributions::Distribution;
use rand::distributions::Bernoulli;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use weighted_choice::{WeightedIndex2, WeightedIndex5};

use goal::{CustomGoal, GoalKind};

/// A structure holding the information for a sequence of summoning
/// sessions done until a certain goal is reached. Keeps some cached information
/// in order to make the simulation as fast as possible.
#[derive(Debug)]
pub struct Sim {
    banner: Banner,
    goal: CustomGoal,
    tables: RandTables,
    rng: SmallRng,
    bernouli: Bernoulli,
    goal_data: GoalData,
}

/// Precalculated tables for the probabilities of units being randomly chosen.
#[derive(Debug, Copy, Clone, Default)]
struct RandTables {
    pool_dists: [WeightedIndex5; 2],
    item_dists: [WeightedIndex2; 5],
}

/// Scratch space for representing the goal in a way that is faster to work with.
#[derive(Debug, Clone)]
struct GoalData {
    pub items_needed: [bool; 4],
    pub copies_needed: [Vec<u8>; 4],
}

impl GoalData {
    fn is_met(&self) -> bool {
        self.items_needed == [false, false, false, false]
    }
}

impl Sim {
    /// Creates a new simulator for the given banner and goal, doing some
    /// moderately expensive initialization. Avoid running in a hot loop, but
    /// it's not a problem to call somewhat frequently.
    pub fn new(banner: Banner, goal: Goal) -> Self {
        let mut sim = Sim {
            banner,
            goal: goal.as_custom(&banner),
            tables: RandTables::default(),
            rng: SmallRng::from_entropy(),
            bernouli: Bernoulli::from_ratio(banner.split_rates.0 as u32, 100).unwrap(),
            goal_data: GoalData {
                items_needed: [false; 4],
                copies_needed: [vec![], vec![], vec![], vec![]],
            },
        };
        sim.init_probability_tables();
        sim
    }

    /// Initializes the precalculated tables used for fast random sampling.
    fn init_probability_tables(&mut self) {
        self.tables.pool_dists[0] = WeightedIndex5::new(self.bases());
        self.tables.pool_dists[1] = WeightedIndex5::new(self.soft_pity_bases());
    }

    // Initializes the internal representation of a goal.
    fn init_goal_data(&mut self) {
        self.goal_data.items_needed = [false, false, false, false];
        for i in 0..4 {
            self.goal_data.copies_needed[i].clear();
        }
        for &goal in &self.goal.goals {
            self.goal_data.copies_needed[goal.item_type as usize].push(goal.num_copies);
            self.goal_data.items_needed[goal.item_type as usize] = true;
        }
    }

    /// Simulates until reaching the current goal, then returns # of pulls done.
    pub fn roll_until_goal(&mut self) -> u32 {
        let mut five_pity_count = 0;
        let mut four_pity_count = 0;
        let mut five_focus_guarantee = false;
        let mut four_focus_guarantee = false;

        let mut pull_count = 0 as u32;
        self.init_goal_data();
        loop {
            let soft_pity = five_pity_count >= (self.banner.soft_pity - 1) as u32;
            let five_pity = five_pity_count >= (self.banner.hard_pity - 1) as u32;
            let four_pity = four_pity_count >= 9;

            let sampled_pool = self.sample_pool(five_pity, four_pity, five_focus_guarantee, four_focus_guarantee, soft_pity);
            
            self.pull_item(sampled_pool);

            match sampled_pool {
                Pool::FivestarFocus => {
                    five_pity_count = 0;
                    four_pity_count = 0;
                    five_focus_guarantee = false;
                }
                Pool::Fivestar => {
                    five_pity_count = 0;
                    four_pity_count = 0;
                    five_focus_guarantee = true;
                }
                Pool::FourstarFocus => {
                    four_pity_count = 0;
                    four_focus_guarantee = false;
                }
                Pool::Fourstar => {
                    four_pity_count = 0;
                    four_focus_guarantee = true;
                }
                _ => {
                    five_pity_count += 1;
                    four_pity_count += 1;
                }
            }

            pull_count += 1;

            if self.goal_data.is_met() {
                return pull_count;
            }
        }
    }

    /// Evaluates the result of selecting the given sample. Returns `true` if the
    /// sample made the rate increase reset.
    fn pull_item(&mut self, sample_pool: Pool) {
        if sample_pool == Pool::Threestar
            || sample_pool == Pool::Fourstar
            || sample_pool == Pool::Fivestar
        {
            return;
        }

        let focus_count = if sample_pool == Pool::FivestarFocus {
            self.banner.focus_sizes[0] + self.banner.focus_sizes[1]
        } else {
            self.banner.focus_sizes[2] + self.banner.focus_sizes[3]
        };
        let which_unit = self.rng.gen::<usize>() % focus_count as usize;

        let (idx, idy) = if sample_pool == Pool::FivestarFocus && which_unit < self.banner.focus_sizes[0] as usize {
            (0, which_unit)
        } else if sample_pool == Pool::FivestarFocus {
            (1, which_unit - self.banner.focus_sizes[0] as usize)
        } else if which_unit < self.banner.focus_sizes[2] as usize {
            (2, which_unit)
        } else {
            (3, which_unit - self.banner.focus_sizes[2] as usize)
        };

        if idy < self.goal_data.copies_needed[idx].len() {
            if self.goal_data.copies_needed[idx][idy] > 1 {
                self.goal_data.copies_needed[idx][idy] -= 1;
            } else {
                self.goal_data.copies_needed[idx].remove(idy);
                if self.goal.kind == GoalKind::Any {
                    self.goal_data.items_needed = [false, false, false, false];
                } else if self.goal_data.copies_needed[idx].len() == 0 {
                    self.goal_data.items_needed[idx] = false;
                }
            }
        }
    }

    /// Chooses a weighted random unit from the summoning pool. `pity_incr` is the
    /// number of times that the 5* rates have increased by 0.5% total.
    fn sample_pool(&mut self, five_pity: bool, four_pity: bool, five_focus: bool, four_focus: bool, soft_pity: bool) -> Pool {
        let pool = if soft_pity {
            Pool::try_from(self.tables.pool_dists[1].sample(&mut self.rng) as u8).unwrap()
        } else {
            Pool::try_from(self.tables.pool_dists[0].sample(&mut self.rng) as u8).unwrap()
        };

        if five_pity && five_focus {
            return Pool::FivestarFocus;
        } else if five_pity && !(pool == Pool::Fivestar || pool == Pool::FivestarFocus) {
            if self.bernouli.sample(&mut self.rng) {
                return Pool::FivestarFocus;
            }
            return Pool::Fivestar;
        } else if five_focus && pool == Pool::Fivestar {
            return Pool::FivestarFocus
        } else if four_pity && four_focus && !(pool == Pool::Fivestar || pool == Pool::FivestarFocus) {
            return Pool::FourstarFocus;
        } else if four_pity && pool == Pool::Threestar {
            if self.bernouli.sample(&mut self.rng) {
                return Pool::FourstarFocus;
            }
            return Pool::Fourstar;
        } else if four_focus && pool == Pool::Fourstar {
            return Pool::FourstarFocus
        } else {
            return pool;
        }
    }

    /// Gives the base probabilities of selecting a unit from each pool.
    fn bases(&self) -> [f32; 5] {
        let fiverate = (self.banner.five_rate as f32) / 10.0;
        let fourrate = (self.banner.four_rate as f32) / 10.0;
        let (focus, nonfocus) = self.banner.split_rates;

        let fivefocus = fiverate * (focus as f32) / 100.0;
        let fivestar = fiverate * (nonfocus as f32) / 100.0;
        let fourfoucs = fourrate * (focus as f32) / 100.0;
        let fourstar =fourrate * (nonfocus as f32) / 100.0;
        let threestar = 100.0 - fiverate - fourrate;
        [fivefocus, fivestar, fourfoucs, fourstar, threestar]

    }

    /// Gives the base probabilities of selecting a unit from each pool with soft pity.
    fn soft_pity_bases(&self) -> [f32; 5] {
        let fiverate = 32.0;
        let fourrate = (self.banner.four_rate as f32) / 10.0;
        let (focus, nonfocus) = self.banner.split_rates;

        let fivefocus = fiverate * (focus as f32) / 100.0;
        let fivestar = fiverate * (nonfocus as f32) / 100.0;
        let fourfoucs = fourrate * (focus as f32) / 100.0;
        let fourstar =fourrate * (nonfocus as f32) / 100.0;
        let threestar = 100.0 - fiverate - fourrate;
        [fivefocus, fivestar, fourfoucs, fourstar, threestar]

    }
}
