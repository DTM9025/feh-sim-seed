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

        // When having multiple goals in Epitomized Path, the opitimal strategy
        // is to select the weapon with the highest amount of copies needed.
        // Our Epitomized Path implementation assumes the selected one is always
        // at index 0, so we sort the goals for 5* Weapons from highest to lowest
        // copies needed to choose the optimal selection.
        // if self.banner.epitomized_path {
        //     self.goal_data.copies_needed[ItemType::FiveWeapon as usize].sort_by(|x, y| y.cmp(x))
        // }
    }

    /// Simulates until reaching the current goal, then returns # of pulls done.
    pub fn roll_until_goal(&mut self) -> u32 {
        let mut five_pity_count = 1;
        let mut four_pity_count = 1;
        let mut five_focus_guarantee = false;
        let mut four_focus_guarantee = false;
        let mut fate_points = 0;
        let mut capturing_points = 0i8;

        let mut pull_count = 0;
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

            let sampled_pool = match sampled_pool {
                Pool::FivestarFocus => {
                    if self.banner.capturing_radiance && !five_focus_guarantee {
                        capturing_points = 0;
                    }
                    five_pity_count = 1;
                    four_pity_count += 1;
                    five_focus_guarantee = false;
                    sampled_pool
                }
                Pool::Fivestar => {
                    five_pity_count = 1;
                    four_pity_count += 1;

                    if !self.banner.capturing_radiance {
                        five_focus_guarantee = true;
                        sampled_pool
                    } else if capturing_points < 2 {
                        capturing_points += 1;
                        five_focus_guarantee = true;
                        sampled_pool
                    } else if capturing_points == 2 {
                        if self.rng.gen_bool(0.5) {
                            capturing_points = 0;
                            five_focus_guarantee = false;
                            Pool::FivestarFocus
                        } else {
                            capturing_points += 1;
                            five_focus_guarantee = true;
                            sampled_pool
                        }
                    } else {
                        capturing_points = 0;
                        five_focus_guarantee = false;
                        Pool::FivestarFocus
                    }
                }
                Pool::FourstarFocus => {
                    five_pity_count += 1;
                    four_pity_count = 1;
                    four_focus_guarantee = false;
                    sampled_pool
                }
                Pool::Fourstar => {
                    five_pity_count += 1;
                    four_pity_count = 1;
                    four_focus_guarantee = true;
                    sampled_pool
                }
                _ => {
                    five_pity_count += 1;
                    four_pity_count += 1;
                    sampled_pool
                }
            };

            if self.banner.epitomized_path {
                let path = (sampled_pool == Pool::Fivestar || sampled_pool == Pool::FivestarFocus) && fate_points >= 1;
                fate_points += self.pull_item(sampled_pool, path);
            } else {
                self.pull_item(sampled_pool, false);
            }

            pull_count += 1;

            if self.goal_data.is_met() {
                return pull_count;
            }
        }
    }

    /// Evaluates the result of selecting the given sample. Returns how Fate Points
    /// should be adjusted per result.
    fn pull_item(&mut self, sample_pool: Pool, path: bool) -> i8 {
        if sample_pool == Pool::Threestar || sample_pool == Pool::Fourstar {
            return 0;
        } else if sample_pool == Pool::Fivestar && !path {
            return 1;
        }

        let focus_count = if sample_pool == Pool::FivestarFocus || path {
            self.banner.focus_sizes[0] + self.banner.focus_sizes[1]
        } else {
            self.banner.focus_sizes[2] + self.banner.focus_sizes[3]
        };
        let which_unit = if path { 0 } else { self.rng.gen::<usize>() % focus_count as usize };

        let (idx, idy) = if sample_pool == Pool::FivestarFocus && which_unit < self.banner.focus_sizes[0] as usize {
            (0, which_unit)
        } else if sample_pool == Pool::FivestarFocus || path {
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

            // Resorts weapon goal so that the selected Epitomized Path (at 0)
            // always is the one with the highest copies needed.
            // TODO: Need to figure out stats on switching EP as doing so clears Fate Points
            // if idx == 1 && self.banner.epitomized_path {
            //     self.goal_data.copies_needed[idx].sort_by(|x, y| y.cmp(x))
            // }
        }

        if path {
            return -1;
        } else if idx == 1 && which_unit != 0 {
            return 1;
        } else {
            return 0;
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
