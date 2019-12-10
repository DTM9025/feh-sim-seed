#[macro_use]
extern crate seed;
use seed::prelude::*;

use std::convert::TryFrom;
use std::fmt;

use strum_macros::EnumIter;

use serde::{Deserialize, Serialize};

mod banner;
use banner::Banner;

mod goal;
use goal::{Goal, GoalKind, GoalPart, GoalPreset};

mod results;

mod sim;
use sim::Sim;

mod weighted_choice;

mod stats;

mod counter;
use counter::Counter;

mod subpages;

mod query_string;

// Model

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumIter, Serialize, Deserialize)]
pub enum Color {
    Red,
    Blue,
    Green,
    Colorless,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<u8> for Color {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Color::*;
        Ok(match value {
            0 => Red,
            1 => Blue,
            2 => Green,
            3 => Colorless,
            _ => return Err(()),
        })
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Pool {
    Focus,
    Fivestar,
    Fourstar,
    Threestar,
}

impl TryFrom<u8> for Pool {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Pool::*;
        Ok(match value {
            0 => Focus,
            1 => Fivestar,
            2 => Fourstar,
            3 => Threestar,
            _ => return Err(()),
        })
    }
}

/// The current page that the application is on.
#[derive(Copy, Clone, Debug)]
pub enum Page {
    Main,
    Help,
    Changelog,
}

impl Default for Page {
    fn default() -> Self {
        Page::Main
    }
}

/// Data model for the app.
#[derive(Default, Debug)]
struct Model {
    /// The data that the simulation has gathered so far.
    pub data: Counter,
    /// The parameters of the current banner.
    pub banner: Banner,
    /// The paremeters of the current goal.
    pub goal: Goal,
    /// The current page that the application is on.
    pub curr_page: Page,
    /// The point on the graph that the user has chose to highlight.
    pub graph_highlight: Option<f32>,
}

// Update

/// Event definition for the app.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Does nothing, not even re-render the page. Exists only to satisfy
    /// static typing in some situations.
    Null,
    /// Holds a collection of messages that will all be queued up at once.
    Multiple(Vec<Msg>),
    /// Gather data.
    Run,
    /// Change the number of focus units for a given color.
    BannerFocusSizeChange { color: Color, quantity: i8 },
    /// Change the starting rates.
    BannerRateChange { rates: (u8, u8) },
    /// Change whether the banner uses the old or new 5* pools.
    BannerFocusTypeToggle,
    /// Replace the banner with a new one.
    BannerSet { banner: Banner },
    /// Set the goal to a certain preset.
    GoalPresetChange { preset: GoalPreset },
    /// Set the number of copies to use for the preset.
    GoalPresetQuantityChange { quantity: u8 },
    /// Change the current preset into a custom goal.
    GoalMakeCustom,
    /// Change the color for an individual unit target.
    GoalPartColorChange { index: usize, color: Color },
    /// Change the number of copies for an individual unit target.
    GoalPartQuantityChange { index: usize, quantity: u8 },
    /// Add a new individual unit target.
    GoalPartAdd { color: Color, quantity: u8 },
    /// Change whether the individual targets all need to happen or just one.
    GoalKindChange { kind: GoalKind },
    /// Replace the goal with a new one.
    GoalSet { goal: Goal },
    /// Change which page of the application is open.
    PageChange(Page),
    /// Generate a permalink that saves the application's paremeters.
    Permalink,
    /// Highlight a point on the graph.
    GraphHighlight { frac: f32 },
}

/// Update model with the given message.
fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    log!(msg);
    match msg {
        Msg::Null => {
            orders.skip();
        }
        Msg::Multiple(messages) => {
            orders.skip();
            for msg in messages {
                orders.send_msg(msg);
            }
        }
        Msg::BannerFocusSizeChange { color, quantity } => {
            model.banner.focus_sizes[color as usize] = quantity;
            model.data.clear();
        }
        Msg::BannerRateChange { rates } => {
            model.banner.starting_rates = rates;
            model.data.clear();
            if rates == (8, 0) {
                // Convenient handling for legendary banners, since they
                // always have the same focus pool sizes.
                model.banner.focus_sizes = [3, 3, 3, 3];
            } else if rates == (6, 0) {
                // Another special kind of banner
                model.banner.focus_sizes = [2, 2, 2, 2];
            }
        }
        Msg::BannerFocusTypeToggle => {
            model.banner.new_units = !model.banner.new_units;
            model.data.clear();
        }
        Msg::BannerSet { banner } => {
            model.banner = banner;
            model.data.clear();
        }
        Msg::Run => {
            if !model.goal.is_available(&model.banner) {
                return;
            }
            let mut sim = Sim::new(model.banner, model.goal.clone());
            let mut limit = 100;
            let perf = seed::window().performance().unwrap();
            let start = perf.now();

            // Exponential increase with a loose target of 500 ms of calculation.
            // Time per simulation varies wildly depending on device performance
            // and sim parameters, so it starts with a very low number and goes
            // from there.
            while perf.now() - start < 250.0 {
                for _ in 0..limit {
                    let result = sim.roll_until_goal();
                    model.data[result] += 1;
                }
                limit *= 2;
            }

            model.graph_highlight = None;
        }
        Msg::GoalPresetChange { preset } => {
            let count = if let Goal::Preset(_, count) = model.goal {
                count
            } else {
                1
            };
            if preset.is_available(&model.banner) {
                model.goal = Goal::Preset(preset, count);
                model.data.clear();
            }
        }
        Msg::GoalPresetQuantityChange { quantity } => {
            if let Goal::Preset(_, count) = &mut model.goal {
                *count = quantity;
                model.data.clear();
            }
        }
        Msg::GoalPartColorChange { index, color } => {
            if let Goal::Custom(custom_goal) = &mut model.goal {
                custom_goal.goals[index].unit_color = color;
                model.data.clear();
            }
        }
        Msg::GoalMakeCustom => {
            model.goal = Goal::Custom(model.goal.as_custom(&model.banner));
            model.data.clear();
        }
        Msg::GoalPartQuantityChange { index, quantity } => {
            if let Goal::Custom(custom_goal) = &mut model.goal {
                if quantity == 0 {
                    custom_goal.goals.remove(index);
                } else {
                    custom_goal.goals[index].num_copies = quantity;
                }
                model.data.clear();
            }
        }
        Msg::GoalPartAdd { color, quantity } => {
            if let Goal::Custom(custom_goal) = &mut model.goal {
                custom_goal.goals.push(GoalPart {
                    unit_color: color,
                    num_copies: quantity,
                });
                model.data.clear();
            }
        }
        Msg::GoalKindChange { kind } => {
            if let Goal::Custom(custom_goal) = &mut model.goal {
                custom_goal.kind = kind;
                model.data.clear();
            }
        }
        Msg::GoalSet { goal } => {
            model.goal = goal;
            model.data.clear();
        }
        Msg::PageChange(page) => {
            model.curr_page = page;
        }
        Msg::Permalink => {
            let url = seed::Url::new(vec!["fehstatsim/"]).search(&format!(
                "banner={}&goal={}&run=1",
                base64::encode(&bincode::serialize(&model.banner).unwrap()),
                base64::encode(&bincode::serialize(&model.goal).unwrap())
            ));
            seed::push_route(url);
        }
        Msg::GraphHighlight { frac } => {
            model.graph_highlight = Some(frac);
        }
    }
}

// View

/// Display the current state.
fn view(model: &Model) -> Vec<El<Msg>> {
    match model.curr_page {
        Page::Main => main_page(model),
        Page::Help => subpages::help(),
        Page::Changelog => subpages::changelog(),
    }
}

/// Display the main page of the application.
fn main_page(model: &Model) -> Vec<El<Msg>> {
    vec![
        header![
            a![
                "Help",
                attrs! [
                    At::Href => "/fehstatsim/help";
                ],
            ],
            " | v0.1.0 ",
            a![
                "Changelog",
                attrs![
                    At::Href => "/fehstatsim/changelog";
                ],
            ],
        ],
        div![
            id!["content"],
            goal::goal_selector(&model.goal, &model.banner),
            banner::banner_selector(&model.banner),
            div![
                style![
                    "display" => "flex";
                    "align-items" => "center";
                ],
                button![
                    simple_ev(Ev::Click, Msg::Run),
                    if !model.goal.is_available(&model.banner) {
                        attrs![At::Disabled => true]
                    } else {
                        attrs![]
                    },
                    if model.data.is_empty() { "Run" } else { "More" }
                ],
                permalink(),
            ],
            results::results(&model.data, model.graph_highlight),
        ],
    ]
}

fn permalink() -> El<Msg> {
    svg![
        id!["permalink"],
        class!["padleft"],
        simple_ev(Ev::Click, Msg::Permalink),
        attrs![
            At::ViewBox => "0 0 150 50";
        ],
        rect![attrs![
            At::Width => 60;
            At::Height => 50;
            "rx" => 25;
            "x" => 5;
        ]],
        rect![attrs![
            At::Width => 40;
            At::Height => 30;
            "rx" => 15;
            At::Fill => "white";
            "x" => 15;
            "y" => 10;
        ]],
        rect![attrs![
            At::Width => 60;
            At::Height => 50;
            "rx" => 25;
            "x" => 75;
        ]],
        rect![attrs![
            At::Width => 40;
            At::Height => 30;
            "rx" => 15;
            At::Fill => "white";
            "x" => 85;
            "y" => 10;
        ]],
        rect![attrs![
            At::Width => 65;
            At::Height => 15;
            At::Fill => "white";
            "x" => 35;
            "y" => 17.5;
        ]],
        rect![attrs![
            At::Width => 60;
            At::Height => 10;
            "rx" => 5;
            "x" => 40;
            "y" => 20;
        ]]
    ]
}

/// Queue up messages based on the URL with which the application was loaded.
fn routes(url: &seed::Url) -> Msg {
    let mut messages = vec![];

    messages.push(match url.path.get(1).map(String::as_str) {
        Some("help") => Msg::PageChange(Page::Help),
        Some("changelog") => Msg::PageChange(Page::Changelog),
        _ => Msg::PageChange(Page::Main),
    });

    if let Some(banner) = query_string::get(url, "banner").and_then(Banner::from_query_string) {
        messages.push(Msg::BannerSet { banner });
    }

    if let Some(goal) = query_string::get(url, "goal").and_then(Goal::from_query_string) {
        messages.push(Msg::GoalSet { goal });
    }

    if let Some("1") = query_string::get(url, "run") {
        messages.push(Msg::Run);
    }

    Msg::Multiple(messages)
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .routes(routes)
        .finish()
        .run();
}
