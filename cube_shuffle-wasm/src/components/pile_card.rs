use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::Pile;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub pile: Pile,
    pub delete: Callback<()>,
}

#[function_component(PileCard)]
pub fn pile_card(props: &Props) -> Html {
    let pile = props.pile;
    let randomness = pile.randomness * 100.0;
    let del = props.delete.reform(|_| {});
    return html! {
        <article class="message is-medium">
            <div class="message-header">
                <label>{ props.name.clone() }</label>
                <button class="delete" onclick={ del }></button>
            </div>
            <div class="message-body has-background-white">
                <table class="table is-fullwidth">
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
        </article>
    };
}