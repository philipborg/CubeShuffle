use std::collections::HashMap;
use std::ops::Rem;

use rand::{RngCore, SeedableRng};
use rand::prelude::StdRng;
use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::{Pack, Pile, ShufflingErrors};

use crate::components::integer_input::IntegerInput;
use crate::components::pack_list::PackList;
use crate::components::piler::Piler;

#[derive(Clone, PartialEq)]
pub enum Msg {
    AddPile {
        name: String,
        pile: Pile,
    },
    UpdateSeed(Option<i128>),
    Pile,
    Shuffle,
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
}

fn distribute_shuffle(app: &App) -> Result<Vec<Pack<String>>, String> {
    let mut rng = StdRng::seed_from_u64(app.seed);
    let packs = match cube_shuffle_core::distribution_shuffle::shuffle(&app.piles, 15, &mut rng) {
        Ok(p) => {p}
        Err(e) => {
            return match e {
                ShufflingErrors::EmptyPacks => {
                    Err(String::from("Empty pack."))
                }
                ShufflingErrors::UndividablePacks{ pack_size, card_count, overflow } => {
                    Err(format!("{} isn't dividable by {}, it overflows by {}.", card_count, pack_size, overflow))
                }
            }
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
            error_message: None
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
                let cut: i128 = seed.unwrap_or(0)
                    .max(-i128::MAX)
                    .abs()
                    .rem(i128::from(u64::MAX));
                self.seed = cut as u64;
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let content = match &self.state {
            State::Shuffled { packs } => {
                let re_pile = link.callback(|_| { Msg::Pile });
                html! {
                    <>
                        <button onclick={ re_pile } >{ "Re-pile" }</button>
                        <hr/>
                        <PackList packs={ packs.clone() }/>
                    </>
                }
            }
            State::Piling => {
                let add_pile = link.callback(|(name, pile)| Msg::AddPile { name, pile });
                let update_seed = link.callback(Msg::UpdateSeed);
                let on_shuffle = link.callback(|_| Msg::Shuffle);
                html! {
                    <>
                        <IntegerInput
                            value={ i128::from(self.seed) }
                            on_change={ update_seed }
                            placeholder="Seed"
                            min={ i128::from(u64::MIN) }
                            max={ i128::from(u64::MAX) }
                        />
                        <button onclick={ on_shuffle }>{ "Shuffle" }</button>
                        <Piler piles={ self.piles.clone() } add_pile={ add_pile }/>
                    </>
                }
            }
        };

        let error_html: Html = self.error_message
            .clone()
            .map_or(html!{}, |e| { return html! {
                <>
                    <p>{ e }</p>
                    <hr/>
                </>
            }});

        return html! {
            <>
                <h1>{ "Cube Shuffle" }</h1>
                <h6><a href="https://github.com/philipborg" target="_blank">{ "by philipborg" }</a></h6>
                <hr/>
                { error_html }
                { content }
            </>
        };
    }
}