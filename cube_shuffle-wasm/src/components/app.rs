use std::collections::HashMap;
use std::ops::Rem;

use rand::{RngCore, SeedableRng};
use rand::prelude::StdRng;
use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::{Pack, Pile};

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
}

fn distribute_shuffle(app: &App) -> Vec<Pack<String>> {
    let mut rng = StdRng::seed_from_u64(app.seed);
    let packs = cube_shuffle_core::distribution_shuffle::shuffle(&app.piles, 15, &mut rng);
    packs.into_iter().map(|pack| {
        Pack {
            card_sources: pack.card_sources
                .into_iter()
                .map(|(k, v)| { (k.clone(), v) })
                .collect()
        }
    }).collect()
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
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
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
                self.state = State::Shuffled {
                    packs: distribute_shuffle(self)
                };
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

        return html! {
            <>
                <h1>{ "Cube Shuffle" }</h1>
                <h6><a href="https://github.com/philipborg" target="_blank">{ "by philipborg" }</a></h6>
                <hr/>
                { content }
            </>
        };
    }
}