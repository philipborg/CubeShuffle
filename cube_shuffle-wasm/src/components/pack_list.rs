use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::Pack;

use crate::components::pack_card::PackCard;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub packs: Vec<Pack<String>>,
}

#[function_component(PackList)]
pub fn pack_list(props: &Props) -> Html {
    let packs: Html = props.packs.iter().enumerate().map(|(index, pack)| {
        html! {
            <div class="column is-narrow">
                <PackCard index={ index + 1 } pack={ pack.clone() }/>
            </div>
        }
    }).collect();
    return html! {
        <div class="columns is-multiline is-centered">
            { packs }
        </div>
    };
}