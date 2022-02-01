use std::collections::HashMap;

use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::Pile;

use crate::components::add_pile::AddPile;
use crate::components::pile_card::pile_cards;

#[derive(Clone, PartialEq)]
pub enum Msg {
    AddPile {
        name: String,
        pile: Pile,
    },
}

#[derive(Clone, PartialEq)]
pub struct App {
    piles: HashMap<String, Pile>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            piles: HashMap::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddPile { name, pile } => {
                self.piles.insert(name, pile);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let add_pile = link.callback(|(name, pile)| Msg::AddPile { name, pile });
        let piles = pile_cards(&self.piles);
        html! {
            <>
                <h1>{ "Cube Shuffle" }</h1>
                <h6><a href="https://github.com/philipborg" target="_blank">{ "by philipborg" }</a></h6>
                <hr/>
                <AddPile on_add={add_pile} />
                <hr/>
                { piles }
            </>
        }
    }
}