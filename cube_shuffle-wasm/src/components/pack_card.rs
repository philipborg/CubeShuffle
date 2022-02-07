use itertools::Itertools;
use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::Pack;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub index: usize,
    pub pack: Pack<String>,
    pub checked: bool,
    pub onclick: Callback<usize>,
}


pub enum Msg {
    Clicked,
}

pub struct PackCard {}

impl Component for PackCard {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            Msg::Clicked => {
                props.onclick.emit(props.index);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();
        let sources: Html = props.pack.card_sources
            .iter()
            .sorted_unstable_by_key(|(name, _)| { name.as_str() })
            .map(|(name, amount)| {
                html! {
                   <tr>
                       <th>{ name }</th>
                       <td>{ amount }</td>
                   </tr>
               }
            })
            .collect();
        let checked_bg = if props.checked { " has-background-success" } else { "" };
        let on_click = link.callback(|_| Msg::Clicked);
        return html! {
            <div class="card" onclick={ on_click }>
                <div class={ "card-header-title".to_owned() + checked_bg }>
                    <label class="label">{ props.index + 1 }</label>
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
}