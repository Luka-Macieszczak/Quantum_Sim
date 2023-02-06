mod state_boxes;
mod context;

use nalgebra::{Complex, UnitVector2,};
use yew::prelude::*;

enum Msg {
    AddOne
}
struct CounterComponent {
    count: i64,
    qubit_state: Vec<Vec<UnitVector2<Complex<f32>>>>
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {count: 0, qubit_state: context::create_state(200)}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                {state_boxes::discrete_state_box(1)}

                {for self.qubit_state.clone().into_iter().flat_map(|state| state.clone().into_iter().map(|qubit| {
                    html!{<div >{state_boxes::quantized_state_box(qubit, 0, 0)}</div>}
                }))}
                
                <p>{ self.count }</p>
                <p>{ self.qubit_state[0][0].x}{" "}{self.qubit_state[0][0].y}</p>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
            </div>
        }
    }
}
fn main() {
    yew::start_app::<CounterComponent>();
}
