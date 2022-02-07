use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::Pile;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub pile: Pile,
    pub delete: Callback<String>,
}

pub enum Msg {
    Delete
}

pub struct PileCard {
}

impl Component for PileCard {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self{}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            Msg::Delete => {
                props.delete.emit(props.name.to_owned());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let delete = ctx.link().callback(|_| Msg::Delete);
        let pile = props.pile;
        let randomness = pile.randomness * 100.0;
        return html! {
            <article class="message is-medium">
                <div class="message-header">
                    <label>{ props.name.clone() }</label>
                    <button class="delete" onclick={ delete }></button>
                </div>
                <div class="message-body has-background-white">
                    <table class="table is-fullwidth">
                        <tbody>
                            <tr>
                                <th>{ "Cards" }</th>
                                <td>{ pile.cards }</td>
                            </tr>
                            <tr>
                                <th>{ "Randomness" }</th>
                                <td>{ randomness }</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </article>
        };
    }
}