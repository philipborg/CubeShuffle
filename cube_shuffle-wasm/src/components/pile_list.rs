use cube_shuffle_core::distribution_shuffle::Pile;
use std::collections::HashMap;
use yew::prelude::*;

use crate::components::pile_card::PileCard;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub piles: HashMap<String, Pile>,
    pub delete_pile: Callback<String>,
    pub pack_size: usize,
}

#[function_component(PileList)]
pub fn pile_card(props: &Props) -> Html {
    let total_cards: u32 = props.piles.values().map(|p| p.cards).sum();
    let expected_packs = if props.pack_size > 0 {
        total_cards as usize / props.pack_size
    } else {
        0
    };

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

    let summary_html = if !props.piles.is_empty() {
        html! {
            <div class="box has-background-info-light mb-4">
                <div class="columns is-centered is-vcentered">
                    <div class="column is-narrow has-text-centered">
                        <p class="heading">{ "Total Cards" }</p>
                        <p class="title is-4">{ total_cards }</p>
                    </div>
                    <div class="column is-narrow has-text-centered">
                        <p class="heading">{ "Piles" }</p>
                        <p class="title is-4">{ props.piles.len() }</p>
                    </div>
                    <div class="column is-narrow has-text-centered">
                        <p class="heading">{ "Expected Packs" }</p>
                        <p class="title is-4">{ expected_packs }</p>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <>
            { summary_html }
            <div class="columns is-multiline is-centered">
                { cards }
            </div>
        </>
    }
}
