use yew::prelude::*;

use itertools::Itertools;

use cube_shuffle_core::distribution_shuffle::Pack;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub index: usize,
    pub pack: Pack<String>,
}

#[function_component(PackCard)]
pub fn pack_card(props: &Props) -> Html {
    let sources: Html = props.pack.card_sources
        .iter()
        .sorted_unstable_by_key(|(name,_)| {name.as_str()})
        .map(|(name, amount)| {
        html! {
           <tr>
               <th>{ name }</th>
               <td>{ amount }</td>
           </tr>
       }
    }).collect();
    return html! {
        <div class="card">
            <div class="card-header-title">
                <label class="label">{ props.index }</label>
            </div>
            <div class="card-content">
                <table class="table is-hoverable is-fullwidth is-striped">
                    <tbody>
                        { sources }
                    </tbody>
                </table>
            </div>
        </div>
    };
}