use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: Option<i128>,
    pub on_change: Callback<Option<i128>>,
    pub step: Option<i128>,
    pub placeholder: Option<String>,
    pub min: Option<i128>,
    pub max: Option<i128>,
}

fn get_value_from_input_event(e: InputEvent) -> Option<i128> {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.value().into());
    let value = target.value();
    value.parse::<i128>().ok()
}

fn map_number(value: Option<i128>) -> String {
    value.map_or_else(String::new, |f| { f.to_string() })
}

#[function_component(IntegerInput)]
pub fn number_input(props: &Props) -> Html {
    let Props { value, on_change, min, max, step, placeholder } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event));
    });

    html! {
        <input
            class="input"
            type="number"
            value={ map_number(value) }
            min={ map_number(min) }
            max={ map_number(max) }
            step={ map_number(step) }
            { oninput }
            { placeholder }
        />
    }
}