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
    pub on_error: Callback<String>,
}

pub struct AddPile {
    name: String,
    cards: usize,
    randomness: i128,
}

impl Component for AddPile {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {
            name: String::new(),
            cards: 50,
            randomness: 10,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                if self.name.is_empty() {
                    ctx.props()
                        .on_error
                        .emit(String::from("Missing pile name."));
                    return false;
                }
                let pile = Pile {
                    cards: self.cards,
                    randomness: (self.randomness as Odds) * 0.01,
                };
                ctx.props().on_add.emit((self.name.clone(), pile));
                self.name = String::new();
                true
            }
            Msg::UpdateName(name) => {
                self.name = name;
                false
            }
            Msg::UpdateCards(cards) => {
                self.cards = cards.map_or(0, |i| usize::try_from(i).unwrap_or(0));
                false
            }
            Msg::UpdateRandomness(randomness) => {
                self.randomness = match randomness {
                    None => 0,
                    Some(r) => r.max(0).min(100),
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
                <div class="field">
                    <label class="label">{ "Pile name" }</label>
                    <div class="control">
                        <TextInput on_change={ update_name } value={ self.name.clone() } placeholder={ "Name of the pile" }
                            tooltip={
                                "The name of the pile.\n\
                                Should be a archetype grouping attribute of the cards, such as \"Green\", \"Blue\", \"Colorless\" and so on.\n"
                            }
                        />
                    </div>
                </div>
                <div class="field">
                    <label class="label">{ "Card count" }</label>
                    <div class="control">
                        <IntegerInput min=0 on_change={ update_cards } step=1 value={ self.cards as i128 } placeholder={ "Number of cards in pile" }
                            tooltip={
                                "The number of cards in this pile.\n\
                                Each individual card should only belong to a single pile."
                            }
                        />
                    </div>
                </div>
                <div class="field">
                    <label class="label">{ "Randomness" }</label>
                    <div class="control">
                        <IntegerInput min=0 max=100 on_change={ update_randomness } step=5 value={ self.randomness } placeholder={ "Percentage of randomness" }
                            tooltip={
                                "The isolated percentage chance that any card in this pile will be randomly distributed instead of evenly distributed to packs.\n\
                                This will randomly place the card in any slot in a pack which is vacant due to that card also being randomly distributed.\n\
                                Meaning at least two piles must have more than 0% for any effect, as otherwise they can only fill card slots they themself vacated.\n"
                            }
                        />
                    </div>
                </div>
                <div class="field">
                    <div class="control">
                        <button class="button is-primary" onclick={ submit }>{ "Add" }</button>
                    </div>
                </div>
            </>
        };
    }
}
