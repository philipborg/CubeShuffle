use itertools::Itertools;
use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::Pack;

use crate::components::pack_card::PackCard;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub packs: Vec<Pack<String>>,
}

pub struct PackItem {
    pub pack: Pack<String>,
    pub checked: bool,
}

pub enum Msg {
    Check(usize),
}

pub struct PackList {
    packs: Vec<PackItem>,
}

impl Component for PackList {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self{
            packs: ctx.props().packs.iter()
                .map(|pack|{
                    PackItem{
                        pack: pack.to_owned(),
                        checked: false,
                    }
                })
                .collect(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Check(index) => {
                self.packs[index].checked = !(self.packs[index].checked);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let packs: Html = self.packs.iter()
            .enumerate()
            .sorted_by_key(|(_, pi)| pi.checked)
            .map(|(index, pack_item)| {
                let on_click = ctx.link().callback(|index| {Msg::Check(index)});
                html! {
                    <div class="column is-narrow">
                        <PackCard
                            index={ index }
                            pack={ pack_item.pack.clone() }
                            checked={ pack_item.checked }
                            onclick={ on_click }
                        />
                    </div>
                }
            })
            .collect();
        return html! {
            <div class="columns is-multiline is-centered">
                { packs }
            </div>
        };
    }
}