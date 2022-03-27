use std::collections::HashMap;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};
use rand::prelude::StdRng;
use rand::{RngCore, SeedableRng};
use sha3::{Digest, Sha3_256};
use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::{Pack, Pile, ShufflingErrors};

use crate::components::add_pile::AddPile;
use crate::components::integer_input::IntegerInput;
use crate::components::pack_list::PackList;
use crate::components::pile_list::PileList;
use crate::components::text_input::TextInput;

#[derive(Clone, PartialEq)]
pub enum Msg {
    AddPile { name: String, pile: Pile },
    DelPile(String),
    UpdateSeed(String),
    UpdatePackSize(Option<i128>),
    Pile,
    Shuffle,
    Error(Option<String>),
}

#[derive(Clone, PartialEq)]
pub enum State {
    Piling,
    Shuffled { packs: Vec<Pack<String>> },
}

#[derive(Clone, PartialEq)]
pub struct App {
    piles: HashMap<String, Pile>,
    state: State,
    seed: String,
    error_message: Option<String>,
    pack_size: u32,
}

fn get_seed(seed: &str) -> u64 {
    match seed.parse::<u64>() {
        Ok(s) => s,
        Err(_) => {
            let mut hasher = Sha3_256::new();
            hasher.update(seed.as_bytes());
            let full_hash = hasher.finalize();
            let mut rdr = Cursor::new(full_hash);
            rdr.read_u64::<BigEndian>().unwrap()
        }
    }
}

fn distribute_shuffle(app: &App) -> Result<Vec<Pack<String>>, String> {
    let seed = get_seed(&app.seed);
    let mut rng = StdRng::seed_from_u64(seed);
    let packs =
        match cube_shuffle_core::distribution_shuffle::shuffle(&app.piles, app.pack_size, &mut rng)
        {
            Ok(p) => p,
            Err(e) => {
                return match e {
                    ShufflingErrors::EmptyPacks => Err(String::from("Empty pack.")),
                    ShufflingErrors::UndividablePacks {
                        pack_size,
                        card_count,
                        overflow,
                    } => Err(format!(
                        "{} isn't dividable by {}, it overflows by {}.",
                        card_count, pack_size, overflow
                    )),
                };
            }
        };
    let owned_packs: Vec<Pack<String>> = packs
        .into_iter()
        .map(|pack| Pack {
            card_sources: pack
                .card_sources
                .into_iter()
                .map(|(k, v)| (k.clone(), v))
                .collect(),
        })
        .collect();
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
            seed: rng.next_u64().to_string(),
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
                self.seed = seed;
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
                    Ok(packs) => self.state = State::Shuffled { packs },
                    Err(e) => self.error_message = Some(e),
                }
                true
            }
            Msg::Error(e) => {
                self.error_message = e;
                true
            }
            Msg::DelPile(pile) => {
                self.piles.remove(&pile);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let content = match &self.state {
            State::Piling => {
                let add_pile = link.callback(|(name, pile)| Msg::AddPile { name, pile });
                let delete_pile = link.callback(Msg::DelPile);
                let update_seed = link.callback(Msg::UpdateSeed);
                let update_pack_size = link.callback(Msg::UpdatePackSize);
                let on_shuffle = link.callback(|_| Msg::Shuffle);
                let on_error = link.callback(|e| Msg::Error(Some(e)));
                html! {
                    <>
                        <div class="columns is-multiline is-centered">
                            <div class="column is-narrow">
                                <div class="field">
                                    <label class="label">{ "Seed" }</label>
                                    <div class="control">
                                        <TextInput
                                            value={ self.seed.clone() }
                                            on_change={ update_seed }
                                            placeholder="Randomness seed"
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
                        <PileList piles={ self.piles.to_owned() } { delete_pile }/>
                    </>
                }
            }
            State::Shuffled { packs } => {
                let re_pile = link.callback(|_| Msg::Pile);
                html! {
                    <>
                        <button class="button is-danger" onclick={ re_pile }>{ "Re-pile" }</button>
                        <PackList packs={ packs.clone() }/>
                    </>
                }
            }
        };

        let clear_error = link.callback(|_| Msg::Error(None));
        let error_html: Html = self.error_message.clone().map_or(html! {}, |e| {
            return html! {
                <div class="notification is-danger">
                    <button onclick={ clear_error } class="delete"></button>
                    <p>{ e }</p>
                </div>
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
                            <strong><a href="https://github.com/philipborg/CubeShuffle/blob/master/docs/distribution_shuffle.adoc" target="_blank">{ "instructions" }</a></strong>
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

#[cfg(test)]
mod tests {
    use crate::components::app::get_seed;
    use itertools::Itertools;
    use proptest::proptest;

    proptest! {
        #[test]
        fn get_seed_u64(seed:u64){
            assert_eq!(seed, get_seed(&seed.to_string()));
        }

        #[test]
        fn get_seed_str(seed:String) {
            get_seed(&seed);
        }
    }

    #[test]
    fn get_seed_unique() {
        let unique = ('A'..'z').map(|c| get_seed(&c.to_string())).all_unique();
        assert!(unique);
    }
}
