use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::{Odds, Pile};

use crate::components::integer_input::IntegerInput;
use crate::components::text_input::TextInput;

pub enum Msg {
    Add,
    UpdateName(String),
    UpdateCards(Option<i128>),
    UpdateRandomness(Option<i128>),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub on_add: Callback<(String, Pile)>,
}

pub struct AddPile {
    name: String,
    cards: u32,
    randomness: Odds,
}

impl Component for AddPile {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {
            name: String::new(),
            cards: 0,
            randomness: 0.0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                if self.name.is_empty() {
                    return false;
                }
                let pile = Pile {
                    cards: self.cards,
                    randomness: self.randomness,
                };
                ctx.props().on_add.emit((self.name.clone(), pile));
                false
            }
            Msg::UpdateName(name) => {
                self.name = name;
                false
            }
            Msg::UpdateCards(cards) => {
                self.cards = cards.map_or(0, |i| u32::try_from(i).unwrap_or(0));
                false
            }
            Msg::UpdateRandomness(randomness) => {
                self.randomness = match randomness {
                    None => { 0. }
                    Some(rand) => {
                        if rand.is_negative() {
                            0.
                        } else if rand > 100 {
                            1.
                        } else {
                            (rand as f64) / 100.
                        }
                    }
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let update_name = ctx.link().callback(Msg::UpdateName);
        let update_cards = ctx.link().callback(Msg::UpdateCards);
        let update_randomness = ctx.link().callback(Msg::UpdateRandomness);
        let submit = ctx.link().callback(|_| Msg::Add);
        return html! {
            <>
                <TextInput on_change={ update_name } value={self.name.clone()}/>
                <IntegerInput min=0 on_change={ update_cards } step=1/>
                <IntegerInput min=0 max=100 on_change={ update_randomness } step=5/>
                <button onclick={submit}>{ "Add" }</button>
            </>
        };
    }
}