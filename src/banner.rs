use seed::prelude::*;

use crate::{ItemType, Msg};

/// Representation of a summoning focus.
#[derive(Copy, Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Banner {
    pub focus_sizes: [i8; 4],
    pub five_rate: u16,
    pub four_rate: u16,
    pub split_rates: (u8, u8),
    pub five_pity: u16,
    pub four_pity: u16,
    pub epitomized_path: bool,
}

impl Default for Banner {
    fn default() -> Self {
        Banner {
            focus_sizes: [1, 0, 3, 0],
            five_rate: 6,
            four_rate: 51,
            split_rates: (55, 45),
            five_pity: 73,
            four_pity: 8,
            epitomized_path: false,
        }
    }
}

impl Banner {
    /// Parses data from the representation used in query strings to share settings.
    pub fn from_query_string(s: &str) -> Option<Self> {
        let data = base64::decode(s).ok()?;
        bincode::deserialize(&data).ok()
    }
}

/// Section for choosing banner parameters.
pub fn banner_selector(banner: &Banner) -> Node<Msg> {
    let rate_option = |five_rate: u8, four_rate: u8, split_rates: (u8, u8), five_pity: u8, four_pity: u8, label: &str| -> Node<Msg> {
        let mut attrs = attrs![
            At::Value => format!("{} {} {} {} {} {}", five_rate, four_rate, split_rates.0, split_rates.1, five_pity, four_pity);
        ];
        if split_rates == banner.split_rates {
            attrs.add(At::Selected, "");
        }
        option![attrs, label]
    };
    div![
        id!["banner_selector"],
        div![
            select![
                id!["starting_rates"],
                input_ev("input", |text| {
                    if let &[Ok(first), Ok(second), Ok(third), Ok(fourth), Ok(fifth), Ok(sixth)] = &*text
                        .split_whitespace()
                        .map(str::parse::<u8>)
                        .collect::<Vec<_>>()
                    {
                        Msg::BannerRateChange {
                            five_rate: first as u16,
                            four_rate: second as u16,
                            split_rates: (third, fourth),
                            five_pity: fifth as u16,
                            four_pity: sixth as u16,
                        }
                    } else {
                        Msg::Null
                    }
                }),
                rate_option(6, 51, (55, 45), 73, 8, "Character Event Wish"),
                rate_option(7, 60, (75, 25), 62, 7, "Weapon Event Wish"),
                rate_option(6, 51, (100, 0), 73, 8, "Standard Wish"),
            ],
            if banner.split_rates == (75, 25) {
                nodes![
                    input![
                        id!["epitomized_path"],
                        simple_ev(Ev::Input, Msg::BannerEpitomizePathToggle),
                        attrs![At::Type => "checkbox"; At::Checked => banner.epitomized_path.as_at_value()],
                    ],
                    label![style![St::PaddingLeft => em(0.25)], attrs![At::For => "epitomized_path"], "Epitomized Path?"]
                ]
            } else {
                vec![]
            }
        ],
        div![
            id!["focus_counts"],
            label![
                attrs![
                    At::For => "focus_count_5c";
                ],
                "Focus 5* Characters:",
            ],
            input![
                id!["focus_count_5c"],
                input_ev("input", |text| {
                    if let Ok(quantity) = text.parse::<i8>() {
                        Msg::BannerFocusSizeChange {
                            item_type: ItemType::FiveChar,
                            quantity,
                        }
                    } else {
                        Msg::BannerFocusSizeChange {
                            item_type: ItemType::FiveChar,
                            quantity: -1,
                        }
                    }
                }),
                attrs![
                    At::Type => "number";
                    At::Class => "small_number";
                    At::Value => if banner.focus_sizes[0] >= 0 {
                        banner.focus_sizes[0].to_string()
                    } else {
                        "".to_string()
                    };
                    At::Min => 0;
                    At::Required => true;
                ]
            ],
            label![
                attrs![
                    At::For => "focus_count_5w";
                ],
                "Focus 5* Weapons:",
            ],
            input![
                id!["focus_count_5w"],
                input_ev("input", |text| {
                    if let Ok(quantity) = text.parse::<i8>() {
                        Msg::BannerFocusSizeChange {
                            item_type: ItemType::FiveWeapon,
                            quantity,
                        }
                    } else {
                        Msg::BannerFocusSizeChange {
                            item_type: ItemType::FiveWeapon,
                            quantity: -1,
                        }
                    }
                }),
                attrs![
                    At::Type => "number";
                    At::Class => "small_number";
                    At::Value => if banner.focus_sizes[1] >= 0 {
                        banner.focus_sizes[1].to_string()
                    } else {
                        "".to_string()
                    };
                    At::Min => 0;
                    At::Required => true;
                ]
            ],
        ],
        div![
            id!["focus_counts"],
            label![
                attrs![
                    At::For => "focus_count_4c";
                ],
                "Focus 4* Characters:",
            ],
            input![
                id!["focus_count_4c"],
                input_ev("input", |text| {
                    if let Ok(quantity) = text.parse::<i8>() {
                        Msg::BannerFocusSizeChange {
                            item_type: ItemType::FourChar,
                            quantity,
                        }
                    } else {
                        Msg::BannerFocusSizeChange {
                            item_type: ItemType::FourChar,
                            quantity: -1,
                        }
                    }
                }),
                attrs![
                    At::Type => "number";
                    At::Class => "small_number";
                    At::Value => if banner.focus_sizes[2] >= 0 {
                        banner.focus_sizes[2].to_string()
                    } else {
                        "".to_string()
                    };
                    At::Min => 0;
                    At::Required => true;
                ]
            ],
            label![
                attrs![
                    At::For => "focus_count_4w";
                ],
                "Focus 4* Weapons:",
            ],
            input![
                id!["focus_count_4w"],
                input_ev("input", |text| {
                    if let Ok(quantity) = text.parse::<i8>() {
                        Msg::BannerFocusSizeChange {
                            item_type: ItemType::FourWeapon,
                            quantity,
                        }
                    } else {
                        Msg::BannerFocusSizeChange {
                            item_type: ItemType::FourWeapon,
                            quantity: -1,
                        }
                    }
                }),
                attrs![
                    At::Type => "number";
                    At::Class => "small_number";
                    At::Value => if banner.focus_sizes[3] >= 0 {
                        banner.focus_sizes[3].to_string()
                    } else {
                        "".to_string()
                    };
                    At::Min => 0;
                    At::Required => true;
                ],
            ],
        ],
    ]
}
