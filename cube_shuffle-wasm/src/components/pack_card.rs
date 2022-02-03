use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::Pack;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub index: usize,
    pub pack: Pack<String>,
}

#[function_component(PackCard)]
pub fn pack_card(props: &Props) -> Html {
    let sources: Html = props.pack.card_sources.iter().map(|(name, amount)| {
        html! {
           <tr>
               <td>{ name }</td>
               <td>{ amount }</td>
           </tr>
       }
    }).collect();
    return html! {
        <div>
            <label>{ props.index }</label>
            <hr/>
            <table>
                { sources }
            </table>
        </div>
    };
}