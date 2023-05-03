use nalgebra::{UnitVector2, Complex};
use yew::prelude::*;

/*
pub struct StateComponent {
    state: UnitVector2<Complex<f32>>,
    isDiscrete: bool,
    a_idx: i32,
    b_idx: i32,

}

enum Msg {
    GetState
}

impl Component for StateComponent {
    type Properties = ();
    type Message = Msg;

    fn create(_ctx: &Context<Self>, state: UnitVector2<Complex<f32>>, isDiscrete, a_idx, b_idx) -> Self {
        Self {state, isDiscrete, a_idx, b_idx}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetState => {
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.isDiscrete{
            return discrete_state_box(0)
        }
        return quantized_state_box()
    }

    fn discrete_state_box(state: i32) -> Html{
        html!{
            <div class="zero-state-box">
                {"| "}{state}{" >"}
            </div>
        }
    }
    
    pub fn quantized_state_box(self) -> Html{
        html!{
            <div class="zero-state-box">
                {"| "}{"ψ"}<sub>{format!("{}{}", self.a_idx, self.b_idx)}</sub>{" >"}
            </div>
        }
    }
}
*/

pub fn discrete_state_box(state: i32) -> Html{
    html!{
        <div class="zero-state-box">
            {"| "}{state}{" >"}
        </div>
    }
}

pub fn quantized_state_box(state: UnitVector2<Complex<f32>>, a_idx: i32, b_idx: i32) -> Html{
    html!{
        <div class="zero-state-box">
            {"| "}{"ψ"}<sub>{format!("{}{}", a_idx, b_idx)}</sub>{" >"}
        </div>
    }
}