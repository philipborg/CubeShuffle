use std::collections::HashMap;

use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::Pile;

use crate::components::add_pile::AddPile;
use crate::components::pile_card::pile_cards;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub piles: HashMap<String, Pile>,
    pub add_pile: Callback<(String, Pile)>,
    pub on_error: Callback<String>,
}

#[derive(Clone, PartialEq)]
pub enum Msg {
    AddPile {
        name: String,
        pile: Pile,
    },
}

#[derive(Clone, PartialEq)]
pub struct Piler;

impl Component for Piler {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddPile { name, pile } => {
                ctx.props().add_pile.emit((name, pile));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let add_pile = link.callback(|(name, pile)| Msg::AddPile { name, pile });
        let piles = pile_cards(&ctx.props().piles);
        return html! {
        <>
            <AddPile on_add={ add_pile } on_error={ &ctx.props().on_error }/>
            <hr/>
            { piles }
        </>
        };
    }
}
