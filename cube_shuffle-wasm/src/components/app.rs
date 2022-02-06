use std::collections::HashMap;

use rand::{RngCore, SeedableRng};
use rand::prelude::StdRng;
use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::{Pack, Pile, ShufflingErrors};

use crate::components::integer_input::IntegerInput;
use crate::components::pack_list::PackList;
use crate::components::pile_card::pile_cards;
use crate::components::add_pile::AddPile;

#[derive(Clone, PartialEq)]
pub enum Msg {
    AddPile {
        name: String,
        pile: Pile,
    },
    UpdateSeed(Option<i128>),
    UpdatePackSize(Option<i128>),
    Pile,
    Shuffle,
    Error(String),
}

#[derive(Clone, PartialEq)]
pub enum State {
    Piling,
    Shuffled {
        packs: Vec<Pack<String>>
    },
}

#[derive(Clone, PartialEq)]
pub struct App {
    piles: HashMap<String, Pile>,
    state: State,
    seed: u64,
    error_message: Option<String>,
    pack_size: u32,
}

fn distribute_shuffle(app: &App) -> Result<Vec<Pack<String>>, String> {
    let mut rng = StdRng::seed_from_u64(app.seed);
    let packs = match cube_shuffle_core::distribution_shuffle::shuffle(&app.piles, app.pack_size, &mut rng) {
        Ok(p) => { p }
        Err(e) => {
            return match e {
                ShufflingErrors::EmptyPacks => {
                    Err(String::from("Empty pack."))
                }
                ShufflingErrors::UndividablePacks { pack_size, card_count, overflow } => {
                    Err(format!("{} isn't dividable by {}, it overflows by {}.", card_count, pack_size, overflow))
                }
            };
        }
    };
    let owned_packs: Vec<Pack<String>> = packs.into_iter().map(|pack| {
        Pack {
            card_sources: pack.card_sources
                .into_iter()
                .map(|(k, v)| { (k.clone(), v) })
                .collect()
        }
    }).collect();
    Ok(owned_packs)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let mut rng = StdRng::from_entropy();
        Self {
            piles: HashMap::new(),
            state: State::Piling,
            seed: rng.next_u64(),
            error_message: None,
            pack_size: 15,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        self.error_message = None;
        match msg {
            Msg::AddPile { name, pile } => {
                self.piles.insert(name, pile);
                true
            }
            Msg::UpdateSeed(seed) => {
                self.seed = seed
                    .and_then(|s| { u64::try_from(s).ok() })
                    .unwrap_or_else(|| { StdRng::from_entropy().next_u64() });
                true
            }
            Msg::UpdatePackSize(pack_size) => {
                self.pack_size = pack_size
                    .and_then(|ps| u32::try_from(ps).ok())
                    .unwrap_or(15);
                true
            }
            Msg::Pile => {
                self.state = State::Piling;
                true
            }
            Msg::Shuffle => {
                match distribute_shuffle(self) {
                    Ok(packs) => {
                        self.state = State::Shuffled {
                            packs
                        }
                    }
                    Err(e) => {
                        self.error_message = Some(e)
                    }
                }
                true
            }
            Msg::Error(e) => {
                self.error_message = Some(e);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let content = match &self.state {
            State::Piling => {
                let add_pile = link.callback(|(name, pile)| Msg::AddPile { name, pile });
                let update_seed = link.callback(Msg::UpdateSeed);
                let update_pack_size = link.callback(Msg::UpdatePackSize);
                let on_shuffle = link.callback(|_| Msg::Shuffle);
                let on_error = link.callback(Msg::Error);
                let piles = pile_cards(&self.piles);
                html! {
                    <>
                        <div class="columns is-multiline is-centered">
                            <div class="column is-narrow">
                                <div class="field">
                                    <label class="label">{ "Seed" }</label>
                                    <div class="control">
                                        <IntegerInput
                                            value={ i128::from(self.seed) }
                                            on_change={ update_seed }
                                            placeholder="Randomness seed"
                                            min={ i128::from(u64::MIN) }
                                            max={ i128::from(u64::MAX) }
                                        />
                                    </div>
                                </div>
                                <div class="field">
                                    <label class="label">{ "Pack size" }</label>
                                    <div class="control">
                                        <IntegerInput
                                            value={ i128::from(self.pack_size) }
                                            on_change={ update_pack_size }
                                            placeholder={ "Number of cards per shuffled pack" }
                                            min={ 0 }
                                            max={ i128::from(u32::MAX) }
                                        />
                                    </div>
                                </div>
                                <div class="field">
                                    <div class="control">
                                        <button class="button is-success" onclick={ on_shuffle }>{ "Shuffle" }</button>
                                    </div>
                                </div>
                            </div>
                            <div class="column is-narrow">
                                <AddPile { on_error } on_add={ add_pile }/>
                            </div>
                        </div>
                        { piles }
                    </>
                }
            }
            State::Shuffled { packs } => {
                let re_pile = link.callback(|_| { Msg::Pile });
                html! {
                    <>
                        <button class="button is-danger" onclick={ re_pile }>{ "Re-pile" }</button>
                        <PackList packs={ packs.clone() }/>
                    </>
                }
            }
        };

        let error_html: Html = self.error_message
            .clone()
            .map_or(html! {}, |e| {
                return html! {
                <>
                    <p>{ e }</p>
                </>
            };
            });

        return html! {
            <>
                <section class="section has-background-black-ter">
                    <div class="container">
                        <h1 class="title has-text-light">{ "Cube Shuffle" }</h1>
                        <p class="subtitle has-text-grey-light">
                            <strong><a href="https://github.com/philipborg/CubeShuffle" target="_blank">{ "Code" }</a></strong>
                            { " and "}
                            <strong><a href="https://github.com/philipborg/CubeShuffle/blob/master/README.md" target="_blank">{ "instructions" }</a></strong>
                            { " by " }
                            <strong><a href="https://github.com/philipborg" target="_blank" rel="author">{ "philipborg" }</a></strong>
                        </p>
                    </div>
                </section>
                { error_html }
                { content }
            </>
        };
    }
}