use cube_shuffle_core::distribution_shuffle::Pile;
use std::collections::HashMap;
use yew::prelude::*;

use crate::components::pile_card::PileCard;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub piles: HashMap<String, Pile>,
    pub delete_pile: Callback<String>,
}

#[function_component(PileList)]
pub fn pile_card(props: &Props) -> Html {
    let cards: Html = props
        .piles
        .iter()
        .map(|(name, pile)| {
            html! {
                <div class="column is-narrow">
                    <PileCard
                        name={ name.clone() }
                        pile={ *pile }
                        delete={ &props.delete_pile }
                    />
                </div>
            }
        })
        .collect();

    html! {
        <div class="columns is-multiline is-centered">
            { cards }
        </div>
    }
}
