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
        <div class="card">
            <div class="card-header-title">
                <label>{ props.name.clone() }</label>
            </div>
            <div class="card-content">
                <table class="table is-hoverable is-fullwidth">
                    <tbody>
                        <tr>
                            <th>{ "Cards" }</th>
                            <td>{ pile.cards }</td>
                        </tr>
                        <tr>
                            <th>{ "Randomness" }</th>
                            <td>{ randomness }</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    };
}

pub fn pile_cards(piles: &HashMap<String, Pile>) -> Html {
    let cards: Html = piles.iter().map(|(name, pile)| html! {
        <div class="column is-narrow">
            <PileCard name={ name.clone() } pile={ *pile }/>
        </div>
    }).collect();

    return html! {
        <div class="columns is-multiline is-centered">
            { cards }
        </div>
    }
}