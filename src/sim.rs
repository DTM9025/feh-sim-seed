use crate::*;

use rand::distributions::Distribution;
use rand::distributions::Bernoulli;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use goal::{CustomGoal, GoalKind};

/// A structure holding the information for a sequence of summoning
/// sessions done until a certain goal is reached. Keeps some cached information
/// in order to make the simulation as fast as possible.
#[derive(Debug)]
pub struct Sim {
    banner: Banner,
    goal: CustomGoal,
    rng: SmallRng,
    bernouli: Bernoulli,
    goal_data: GoalData,
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
        let sim = Sim {
            banner,
            goal: goal.as_custom(&banner),
            rng: SmallRng::from_entropy(),
            bernouli: Bernoulli::from_ratio(banner.split_rates.0 as u32, 100).unwrap(),
            goal_data: GoalData {
                items_needed: [false; 4],
                copies_needed: [vec![], vec![], vec![], vec![]],
            },
        };
        sim
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
        let mut five_pity_count = 1;
        let mut four_pity_count = 1;
        let mut five_focus_guarantee = false;
        let mut four_focus_guarantee = false;

        let mut pull_count = 0 as u32;
        self.init_goal_data();
        loop {
            let five_prob = if five_pity_count > self.banner.five_pity {
                self.banner.five_rate + (five_pity_count - self.banner.five_pity) * 10 * self.banner.five_rate
            } else {
                self.banner.five_rate
            };

            let four_prob = if four_pity_count > self.banner.four_pity {
                self.banner.four_rate + (four_pity_count - self.banner.four_pity) * 10 * self.banner.four_rate
            } else {
                self.banner.four_rate
            };

            let sampled_pool = self.sample_pool(five_prob as f32 / 1000.0, four_prob as f32 / 1000.0, five_focus_guarantee, four_focus_guarantee);
            
            self.pull_item(sampled_pool);

            match sampled_pool {
                Pool::FivestarFocus => {
                    five_pity_count = 1;
                    four_pity_count += 1;
                    five_focus_guarantee = false;
                }
                Pool::Fivestar => {
                    five_pity_count = 1;
                    four_pity_count += 1;
                    five_focus_guarantee = true;
                }
                Pool::FourstarFocus => {
                    five_pity_count += 1;
                    four_pity_count = 1;
                    four_focus_guarantee = false;
                }
                Pool::Fourstar => {
                    five_pity_count += 1;
                    four_pity_count = 1;
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
    fn sample_pool(&mut self, five_prob: f32, four_prob: f32, five_focus: bool, four_focus: bool) -> Pool {
        let choice = self.rng.gen::<f32>();

        if choice < five_prob && five_focus {
            return Pool::FivestarFocus;
        } else if choice < five_prob {
            if self.bernouli.sample(&mut self.rng) {
                return Pool::FivestarFocus;
            }
            return Pool::Fivestar;
        } else if choice < five_prob + four_prob && four_focus {
            return Pool::FourstarFocus;
        } else if choice < five_prob + four_prob {
            if self.bernouli.sample(&mut self.rng) {
                return Pool::FourstarFocus;
            }
            return Pool::Fourstar;
        } else {
            return Pool::Threestar;
        }
    }
}
