use crate::Msg;
use seed::prelude::*;

/// The header of one of the informational pages.
fn header() -> Node<Msg> {
    header![
        style![
            "text-align" => "start";
        ],
        a![
            "Back",
            attrs! [
                At::Href => "/orbbehavior/";
            ]
        ],
    ]
}

/// Page contents for the about page.
pub fn about() -> Vec<Node<Msg>> {
    let mut els = vec![header()];
    els.extend(El::from_markdown(include_str!("subpages/about.md")));
    els
}
