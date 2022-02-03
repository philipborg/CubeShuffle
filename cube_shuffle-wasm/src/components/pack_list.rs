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
            <PackCard { index } pack={ pack.clone() }/>
        }
    }).collect();
    packs
}