use seed::prelude::*;

use std::convert::TryFrom;
use std::fmt;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use serde::{Deserialize, Serialize};

use crate::{banner::Banner, ItemType, Msg};

/// Pre-set options for common goals.
#[derive(Copy, Clone, Debug, EnumIter, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalPreset {
    AnyFive,
    AnyFiveChar,
    FiveCharFocus,
    AnyFiveWeapon,
    FiveWeaponFocus,
    AnyFour,
    AnyFourChar,
    FourCharFocus,
    AnyFourWeapon,
    FourWeaponFocus,
}

impl fmt::Display for GoalPreset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::goal::GoalPreset::*;
        let s = match *self {
            AnyFive => "Any 5* Focus Item (Includes Weapon or Character)",
            AnyFiveChar => "Any 5* Focus Character",
            FiveCharFocus => "Specific 5* Focus Character",
            AnyFiveWeapon => "Any 5* Focus Weapon",
            FiveWeaponFocus => "Specific 5* Focus Weapon",
            AnyFour => "Any 4* Focus Item (Includes Weapon or Character)",
            AnyFourChar => "Any 4* Focus Character",
            FourCharFocus => "Specific 4* Focus Character",
            AnyFourWeapon => "Any 4* Focus Weapon",
            FourWeaponFocus => "Specific 4* Focus Weapon",
        };
        f.write_str(s)
    }
}

impl TryFrom<u8> for GoalPreset {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        for variant in GoalPreset::iter() {
            if variant as usize == value as usize {
                return Ok(variant);
            }
        }
        Err(())
    }
}

impl GoalPreset {
    /// Determines whether or not the selected preset is a goal that it is
    /// possible to achieve on the banner.
    pub fn is_available(self, banner: &Banner) -> bool {
        use GoalPreset::*;
        match self {
            AnyFive => {
                banner.focus_sizes[0] > 0 || banner.focus_sizes[1] > 0
            }
            AnyFiveChar | FiveCharFocus => banner.focus_sizes[0] > 0,
            AnyFiveWeapon | FiveWeaponFocus => banner.focus_sizes[1] > 0,
            AnyFour => {
                banner.focus_sizes[2] > 0 || banner.focus_sizes[3] > 0
            }
            AnyFourChar | FourCharFocus => banner.focus_sizes[2] > 0,
            AnyFourWeapon | FourWeaponFocus => banner.focus_sizes[3] > 0,
        }
    }

    /// Says whether or not the preset has only a single unit that counts for
    /// completing the goal.
    fn is_single_target(&self) -> bool {
        use GoalPreset::*;
        match self {
            FiveCharFocus
            | FiveWeaponFocus
            | FourCharFocus
            | FourWeaponFocus => true,
            _ => false,
        }
    }
}

/// Whether the given goal is to achieve all of the goal parts or just a single one.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalKind {
    Any,
    All,
}

/// A single unit that the goal is trying to obtain.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct GoalPart {
    pub item_type: ItemType,
    pub num_copies: u8,
    pub four_star: bool,
}

/// The flexible representation of a goal
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomGoal {
    pub kind: GoalKind,
    pub goals: Vec<GoalPart>,
}

/// The goal of a summoning session.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Goal {
    Custom(CustomGoal),
    Preset(GoalPreset, u8),
}

impl Default for Goal {
    fn default() -> Self {
        Goal::Preset(GoalPreset::AnyFive, 1)
    }
}

impl Goal {
    /// Convert the current preset into a custom goal or retreive the current
    /// custom goal.
    pub fn as_custom(&self, banner: &Banner) -> CustomGoal {
        use crate::goal::GoalKind::*;
        use crate::goal::GoalPreset::*;

        let (preset, count) = match self {
            Goal::Preset(preset, count) => (*preset, *count),
            Goal::Custom(custom) => return custom.clone(),
        };

        let count = if preset.is_single_target() {
            count.max(1)
        } else {
            1
        };

        let kind = Any;

        let mut custom_goal = CustomGoal {
            kind,
            goals: vec![],
        };

        let mut add_item_goal = |item_type: ItemType, four_star: bool| {
            custom_goal.goals.push(GoalPart {
                item_type: item_type,
                num_copies: count,
                four_star,
            });
        };
        // Add an individual GoalPart for each focus unit that matches the
        // conditions of the overall goal.
        match preset {
            AnyFive => {
                for idx in 0..2 {
                    for _ in 0..banner.focus_sizes[idx] {
                        add_item_goal(ItemType::try_from(idx as u8).unwrap(), false);
                    }
                }
            }
            AnyFiveChar => {
                for _ in 0..banner.focus_sizes[0] {
                    add_item_goal(ItemType::FiveChar, false);
                }
            }
            FiveCharFocus => add_item_goal(ItemType::FiveChar, false),
            AnyFiveWeapon => {
                for _ in 0..banner.focus_sizes[1] {
                    add_item_goal(ItemType::FiveWeapon, false);
                }
            }
            FiveWeaponFocus => add_item_goal(ItemType::FiveWeapon, false),
            AnyFour => {
                for idx in 2..4 {
                    for _ in 0..banner.focus_sizes[idx] {
                        add_item_goal(ItemType::try_from(idx as u8).unwrap(), true);
                    }
                }
            }
            AnyFourChar => {
                for _ in 0..banner.focus_sizes[2] {
                    add_item_goal(ItemType::FourChar, true);
                }
            }
            FourCharFocus => add_item_goal(ItemType::FourChar, true),
            AnyFourWeapon => {
                for _ in 0..banner.focus_sizes[3] {
                    add_item_goal(ItemType::FourWeapon, true);
                }
            }
            FourWeaponFocus => add_item_goal(ItemType::FourWeapon, true),
        }

        custom_goal
    }

    /// Checks whether or not the goal is possible on the given banner.
    pub fn is_available(&self, banner: &Banner) -> bool {
        match self {
            Goal::Custom(custom_goal) => custom_goal
                .goals
                .iter()
                .any(|&GoalPart { item_type, .. }| banner.focus_sizes[item_type as usize] > 0),
            Goal::Preset(preset, _) => preset.is_available(banner),
        }
    }

    /// Parses data from the representation used in query strings to share settings.
    pub fn from_query_string(s: &str) -> Option<Self> {
        let data = base64::decode(s).ok()?;
        bincode::deserialize(&data).ok()
    }
}

/// Section for selecting the goal.
pub fn goal_selector(goal: &Goal, banner: &Banner) -> Node<Msg> {
    let mut select = select![
        id!["goal"],
        input_ev("input", |text| {
            if let Some(preset) = text
                .parse::<u8>()
                .ok()
                .and_then(|id| GoalPreset::try_from(id).ok())
            {
                Msg::GoalPresetChange { preset }
            } else if text == "custom" {
                Msg::GoalMakeCustom
            } else {
                Msg::Null
            }
        }),
    ];
    select.add_child(option![
        attrs![
            At::Value => "custom";
        ],
        if let Goal::Custom(_) = goal {
            attrs![
                At::Selected => "";
            ]
        } else {
            attrs![]
        },
        "Custom goal",
    ]);
    for preset in GoalPreset::iter() {
        let mut attrs = attrs! [
            At::Value => preset as usize;
        ];
        if !preset.is_available(banner) {
            attrs.add(At::Disabled, "");
        } else if let Goal::Preset(goal_preset, _) = goal {
            if *goal_preset == preset {
                attrs.add(At::Selected, "");
            }
        }
        select.add_child(option![attrs, preset.to_string(),]);
    }
    div![
        id!["goal_selector"],
        select,
        if let Goal::Preset(preset, count) = goal {
            if preset.is_single_target() {
                span![
                    label![
                        attrs![
                            At::For => "goal_count";
                        ],
                        "Count: ",
                    ],
                    input![
                        id!["goal_count"],
                        input_ev("input", |text| {
                            if let Ok(quantity) = text.parse::<u8>() {
                                Msg::GoalPresetQuantityChange { quantity }
                            } else {
                                Msg::GoalPresetQuantityChange { quantity: 0 }
                            }
                        }),
                        attrs! [
                            At::Type => "number";
                            At::Value => if *count > 0 {
                                count.to_string()
                            } else {
                                "".to_string()
                            };
                            At::Class => "small_number";
                            At::Min => 1;
                            At::Required => true;
                        ],
                    ]
                ]
            } else {
                seed::empty()
            }
        } else {
            seed::empty()
        },
        advanced_goal_selector(goal),
    ]
}

/// Subsection for selecting the goal using the detailed representation instead of
/// a preset.
fn advanced_goal_selector(goal: &Goal) -> Node<Msg> {
    if let Goal::Custom(custom_goal) = goal {
        let mut base = div![style!["margin-left" => "2em";]];
        if custom_goal.goals.len() > 1 {
            base.add_child(select![
                input_ev(Ev::Input, |text| match &*text {
                    "Any" => Msg::GoalKindChange {
                        kind: GoalKind::Any
                    },
                    "All" => Msg::GoalKindChange {
                        kind: GoalKind::All
                    },
                    _ => Msg::Null,
                }),
                option![
                    attrs![
                        At::Value => "Any";
                    ],
                    if custom_goal.kind == GoalKind::Any {
                        attrs![At::Selected => ""]
                    } else {
                        attrs![]
                    },
                    "Any of these",
                ],
                option![
                    attrs![
                        At::Value => "All";
                    ],
                    if custom_goal.kind == GoalKind::All {
                        attrs![At::Selected => ""]
                    } else {
                        attrs![]
                    },
                    "All of these",
                ],
            ]);
        }

        for (index, goal_part) in custom_goal.goals.iter().enumerate() {
            let mut item_select = select![input_ev(Ev::Input, move |value| {
                if let Some(item_type) = value
                    .parse::<u8>()
                    .ok()
                    .and_then(|num| ItemType::try_from(num).ok())
                {
                    Msg::GoalPartItemTypeChange { index, item_type }
                } else {
                    Msg::Null
                }
            }),];
            for item_type in ItemType::iter() {
                let mut attrs = attrs![At::Value => item_type as usize];
                if goal_part.item_type == item_type {
                    attrs.add(At::Selected, "");
                }
                item_select.add_child(option![attrs, item_type.to_string()]);
            }
            base.add_child(div![
                button![
                    simple_ev(
                        Ev::Click,
                        Msg::GoalPartQuantityChange { index, quantity: 0 }
                    ),
                    "X",
                ],
                input![
                    input_ev(Ev::Input, move |value| {
                        if let Ok(quantity) = value.parse::<u8>() {
                            Msg::GoalPartQuantityChange { index, quantity }
                        } else {
                            Msg::Null
                        }
                    }),
                    attrs![
                        At::Type => "number";
                        At::Class => "small_number";
                        At::Min => 0;
                        At::Required => true;
                        At::Value => goal_part.num_copies;
                    ]
                ],
                " copies of a specific ",
                item_select,
                " focus item",
            ]);
        }

        base.add_child(button![
            simple_ev(
                Ev::Click,
                Msg::GoalPartAdd {
                    item_type: ItemType::FiveChar,
                    quantity: 1
                }
            ),
            "+",
        ]);

        base
    } else {
        seed::empty()
    }
}
