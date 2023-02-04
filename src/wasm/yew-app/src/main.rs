use nalgebra::{Complex, UnitVector2, Vector2};
use num_traits::One;
use num_traits::Zero;
use yew::prelude::*;

enum Msg {
    AddOne
}
struct CounterComponent {
    count: i64,
    qubits: Vec<Vec<UnitVector2<Complex<f32>>>>
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {count: 0, qubits: vec![vec![UnitVector2::new_normalize(Vector2::new(Complex::one(), Complex::zero()))]]}
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
                <p>{ self.count }</p>
                <p>{ self.qubits[0][0].x}{" "}{self.qubits[0][0].y}</p>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
            </div>
        }
    }
}
fn main() {
    yew::start_app::<CounterComponent>();
}
