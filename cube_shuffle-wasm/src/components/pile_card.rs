use std::collections::HashMap;

use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::Pile;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub pile: Pile,
}

#[function_component(PileCard)]
pub fn pile_card(props: &Props) -> Html {
    let pile = props.pile;
    let randomness = pile.randomness * 100.0;
    return html! {
        <div>
            <label>{ props.name.clone() }</label>
            <label>{ format!("Cards: {}", pile.cards) }</label>
            <label>{ format!("Randomness: {}%", randomness) }</label>
        </div>
    };
}

pub fn pile_cards(piles: &HashMap<String, Pile>) -> Html {
    return piles.iter().map(|(name, pile)| html! {
        <PileCard name={ name.clone() } pile={ *pile }/>
    }).collect();
}